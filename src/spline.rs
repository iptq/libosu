// WARNING: totally unsafe vector manipulation below!
// TODO: figure out if this is type-safe

use std::collections::VecDeque;
use std::mem::ManuallyDrop;

use ordered_float::NotNan;

use crate::hitobject::SliderSplineKind;
use crate::math::Math;
use crate::math::Point;

/// Represents a spline, a set of points that represents the actual shape of a slider, generated
/// from the control points.
pub struct Spline {
    /// The actual points
    pub spline_points: Vec<P>,

    /// The cumulative lengths over the points. The indices correspond to the spline_points field
    pub cumulative_lengths: Vec<NotNan<f64>>,
}

impl Spline {
    /// Create a new spline from the control points of a slider.
    pub fn from_control(
        kind: SliderSplineKind,
        control_points: &[Point<i32>],
        pixel_length: f64,
    ) -> Self {
        // no matter what, if there's 2 control points, it's linear
        let mut kind = kind.clone();
        if control_points.len() == 2 {
            kind = SliderSplineKind::Linear;
        }

        let points = control_points
            .iter()
            .map(|p| Point(p.0 as f64, p.1 as f64))
            .collect::<Vec<_>>();
        let spline_points = match kind {
            SliderSplineKind::Linear => {
                let start = points[0];
                let unit = (points[1] - points[0]).norm();
                let end = points[0] + unit * pixel_length;
                vec![start, end]
            }
            SliderSplineKind::Perfect => {
                let (p1, p2, p3) = (points[0], points[1], points[2]);
                let (center, radius) = Math::circumcircle(p1, p2, p3);

                // find the t-values of the start and end of the slider
                let t0 = (center.1 - p1.1).atan2(p1.0 - center.0);
                let mut mid = (center.1 - p2.1).atan2(p2.0 - center.0);
                let mut t1 = (center.1 - p3.1).atan2(p3.0 - center.0);

                // make sure t0 is less than t1
                while mid < t0 {
                    mid += std::f64::consts::TAU;
                }
                while t1 < t0 {
                    t1 += std::f64::consts::TAU;
                }
                if mid > t1 {
                    t1 -= std::f64::consts::TAU;
                }

                // circumference is 2 * pi * r, slider length over circumference is length/(2 * pi * r)
                let direction_unit = (t1 - t0) / (t1 - t0).abs();
                let new_t1 = t0 + direction_unit * (pixel_length / radius);

                let mut t = t0;
                let mut c = Vec::new();
                loop {
                    if !((new_t1 >= t0 && t < new_t1) || (new_t1 < t0 && t > new_t1)) {
                        break;
                    }

                    let rel = Point(t.cos() * radius, -t.sin() * radius);
                    c.push(center + rel);

                    t += (new_t1 - t0) / pixel_length;
                }
                c
            }
            SliderSplineKind::Bezier => {
                let mut idx = 0;
                let mut whole = Vec::new();
                let mut cumul_length = 0.0;
                let mut last_circ: Option<P> = None;
                let mut check_push = |whole: &mut Vec<P>, point: P| -> bool {
                    if cumul_length < pixel_length {
                        whole.push(point);
                        if let Some(circ) = last_circ {
                            cumul_length += circ.distance(point);
                        }
                        last_circ = Some(point);
                        true
                    } else {
                        false
                    }
                };

                // split the curve by red-anchors
                for i in 1..points.len() {
                    if points[i].0 == points[i - 1].0 && points[i].1 == points[i - 1].1 {
                        let spline = calculate_bezier(&points[idx..i]);

                        // check if it's equal to the last thing that was added to whole
                        if let Some(last) = whole.last() {
                            if spline[0] != *last {
                                check_push(&mut whole, spline[0]);
                            }
                        } else {
                            check_push(&mut whole, spline[0]);
                        }

                        // add points, making sure no 2 are the same
                        for points in spline.windows(2) {
                            if points[0] != points[1] {
                                if !check_push(&mut whole, points[1]) {
                                    break;
                                }
                            }
                        }

                        idx = i;
                        continue;
                    }
                }

                let spline = calculate_bezier(&points[idx..]);
                if let Some(last) = whole.last() {
                    if spline[0] != *last {
                        check_push(&mut whole, spline[0]);
                    }
                } else {
                    check_push(&mut whole, spline[0]);
                }
                for points in spline.windows(2) {
                    if points[0] != points[1] {
                        if !check_push(&mut whole, points[1]) {
                            break;
                        }
                    }
                }
                whole
            }
            _ => todo!(),
        };

        let mut cumulative_lengths = Vec::with_capacity(spline_points.len());
        let mut curr = 0.0;
        // using NotNan here because these need to be binary-searched over
        // and f64 isn't Ord
        cumulative_lengths.push(unsafe { NotNan::unchecked_new(curr) });
        for points in spline_points.windows(2) {
            let dist = points[0].distance(points[1]);
            curr += dist;
            cumulative_lengths.push(unsafe { NotNan::unchecked_new(curr) });
        }

        Spline {
            spline_points,
            cumulative_lengths,
        }
    }

    /// Calculate the point at which the slider ball would be after it has traveled a distance of
    /// `length` into the slider.
    pub fn point_at_length(&self, length: f64) -> P {
        let length_notnan = unsafe { NotNan::unchecked_new(length) };
        match self.cumulative_lengths.binary_search(&length_notnan) {
            Ok(idx) => self.spline_points[idx],
            Err(idx) => {
                let n = self.spline_points.len();
                if idx == 0 && self.spline_points.len() > 2 {
                    return self.spline_points[0];
                } else if idx == n {
                    return self.spline_points[n - 1];
                }

                let (len1, len2) = (
                    self.cumulative_lengths[idx - 1].into_inner(),
                    self.cumulative_lengths[idx].into_inner(),
                );
                let proportion = (length - len1) / (len2 - len1);

                let (p1, p2) = (self.spline_points[idx - 1], self.spline_points[idx]);
                (p2 - p1) * proportion + p1
            }
        }
    }
}

type P = Point<f64>;
type V<T> = (*mut T, usize, usize);
fn calculate_bezier(points: &[P]) -> Vec<P> {
    let points = points.to_vec();
    let mut output = Vec::new();
    let n = points.len() - 1;
    let last = points[n];

    let mut to_flatten = VecDeque::new();
    let mut free_buffers = VecDeque::new();

    to_flatten.push_back(vec_to_parts(points));
    let p = n;
    let buf1 = vec_to_parts(vec![Point(0.0, 0.0); p + 1]);
    let buf2 = vec_to_parts(vec![Point(0.0, 0.0); p * 2 + 1]);

    let left_child = buf2;
    while !to_flatten.is_empty() {
        let parent = to_flatten.pop_front().unwrap();
        let parent_slice = unsafe { std::slice::from_raw_parts_mut(parent.0, parent.1) };

        if bezier_flat_enough(parent_slice) {
            bezier_approximate(parent_slice, &mut output, buf1, buf2, p + 1);
            free_buffers.push_front(parent);
            continue;
        }

        let right_child = if free_buffers.is_empty() {
            let buf = vec![Point(0.0, 0.0); p + 1];
            vec_to_parts(buf)
        } else {
            free_buffers.pop_front().unwrap()
        };
        bezier_subdivide(parent_slice, left_child, right_child, buf1, p + 1);

        let left_child = unsafe { std::slice::from_raw_parts(left_child.0, left_child.1) };
        for i in 0..p + 1 {
            parent_slice[i] = left_child[i];
        }

        to_flatten.push_front(right_child);
        to_flatten.push_front(parent);
    }

    output.push(last);
    output
}

const TOLERANCE: f64 = 0.25;
fn bezier_flat_enough(curve: &[P]) -> bool {
    for i in 1..(curve.len() - 1) {
        let p = curve[i - 1] - curve[i] * 2.0 + curve[i + 1];
        if p.0 * p.0 + p.1 * p.1 > TOLERANCE * TOLERANCE / 4.0 {
            return false;
        }
    }
    true
}

fn bezier_approximate(curve: &[P], output: &mut Vec<P>, buf1: V<P>, buf2: V<P>, count: usize) {
    let l = buf2;
    let r = buf1;
    bezier_subdivide(curve, l, r, buf1, count);

    let l = unsafe { std::slice::from_raw_parts_mut(l.0, l.1) };
    let r = unsafe { std::slice::from_raw_parts_mut(r.0, r.1) };
    for i in 0..(count - 1) {
        l[count + i] = r[i + 1];
    }
    output.push(curve[0]);

    for i in 1..(count - 1) {
        let idx = 2 * i;
        let p = (l[idx - 1] + l[idx] * 2.0 + l[idx + 1]) * 0.25;
        output.push(p);
    }
}

fn bezier_subdivide(curve: &[P], l: V<P>, r: V<P>, subdiv: V<P>, count: usize) {
    let midpoints = unsafe { std::slice::from_raw_parts_mut(subdiv.0, subdiv.1) };
    for i in 0..count {
        midpoints[i] = curve[i];
    }

    let l = unsafe { std::slice::from_raw_parts_mut(l.0, l.1) };
    let r = unsafe { std::slice::from_raw_parts_mut(r.0, r.1) };
    for i in 0..count {
        l[i] = midpoints[0];
        r[count - i - 1] = midpoints[count - i - 1];
        for j in 0..(count - i - 1) {
            midpoints[j] = (midpoints[j] + midpoints[j + 1]) * 0.5;
        }
    }
}

fn vec_to_parts<T>(vec: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(vec);
    (me.as_mut_ptr(), me.len(), me.capacity())
}
