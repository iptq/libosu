use ordered_float::NotNan;

use crate::hitobject::SliderSplineKind;
use crate::math::{Math, Point};

/// Represents a spline, a set of points that represents the actual shape of a slider, generated
/// from the control points.
#[derive(Clone, Debug)]
pub struct Spline {
    /// The actual points
    pub spline_points: Vec<P>,

    /// The cumulative lengths over the points. The indices correspond to the spline_points field
    pub cumulative_lengths: Vec<NotNan<f64>>,
}

impl Spline {
    /// Create a new spline from the control points of a slider.
    ///
    /// Pixel length gives the length in osu!pixels that the slider should be. If it's not given,
    /// the full slider will be rendered.
    pub fn from_control(
        kind: SliderSplineKind,
        control_points: &[Point<i32>],
        pixel_length: Option<f64>,
    ) -> Self {
        // no matter what, if there's 2 control points, it's linear
        let mut kind = kind;
        let mut control_points = control_points.to_vec();
        if control_points.len() == 2 {
            kind = SliderSplineKind::Linear;
        }
        if control_points.len() == 3
            && Math::is_line(
                control_points[0].to_float::<f64>().unwrap(),
                control_points[1].to_float::<f64>().unwrap(),
                control_points[2].to_float::<f64>().unwrap(),
            )
        {
            kind = SliderSplineKind::Linear;
            control_points.remove(1);
        }

        let points = control_points
            .iter()
            .map(|p| Point::new(p.x as f64, p.y as f64))
            .collect::<Vec<_>>();
        let spline_points = match kind {
            SliderSplineKind::Linear => {
                let start = points[0];
                let end = if let Some(pixel_length) = pixel_length {
                    Math::point_on_line(points[0], points[1], pixel_length)
                } else {
                    points[1]
                };
                vec![start, end]
            }
            SliderSplineKind::Perfect => {
                let (p1, p2, p3) = (points[0], points[1], points[2]);
                let (center, radius) = Math::circumcircle(p1, p2, p3);

                // find the t-values of the start and end of the slider
                let t0 = (center.y - p1.y).atan2(p1.x - center.x);
                let mut mid = (center.y - p2.y).atan2(p2.x - center.x);
                let mut t1 = (center.y - p3.y).atan2(p3.x - center.x);

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

                let diff = (t1 - t0).abs();
                let pixel_length = pixel_length.unwrap_or(radius * diff);

                // circumference is 2 * pi * r, slider length over circumference is length/(2 * pi * r)
                let direction_unit = (t1 - t0) / (t1 - t0).abs();
                let new_t1 = t0 + direction_unit * (pixel_length / radius);

                let mut t = t0;
                let mut c = Vec::new();
                loop {
                    if !((new_t1 >= t0 && t < new_t1) || (new_t1 < t0 && t > new_t1)) {
                        break;
                    }

                    let rel = Point::new(t.cos() * radius, -t.sin() * radius);
                    c.push(center + rel);

                    t += (new_t1 - t0) / pixel_length;
                }
                c
            }
            SliderSplineKind::Bezier => {
                let mut output = Vec::new();
                let mut last_index = 0;
                let mut i = 0;
                while i < points.len() {
                    let multipart_segment = i < points.len() - 2 && (points[i] == points[i + 1]);
                    if multipart_segment || i == points.len() - 1 {
                        let sub = &points[last_index..i + 1];
                        if sub.len() == 2 {
                            output.push(points[0]);
                            output.push(points[1]);
                        } else {
                            create_singlebezier(&mut output, sub);
                        }
                        if multipart_segment {
                            i += 1;
                        }
                        last_index = i;
                    }
                    i += 1;
                }
                output
            }
            _ => todo!(),
        };

        let mut cumulative_lengths = Vec::with_capacity(spline_points.len());
        let mut curr = 0.0;
        // using NotNan here because these need to be binary-searched over
        // and f64 isn't Ord
        cumulative_lengths.push(NotNan::new(curr).unwrap());
        for points in spline_points.windows(2) {
            let dist = points[0].distance(points[1]);
            curr += dist;
            cumulative_lengths.push(NotNan::new(curr).unwrap());
        }

        Spline {
            spline_points,
            cumulative_lengths,
        }
    }

    /// Truncate the length of the spline irreversibly
    pub fn truncate(&mut self, to_length: f64) {
        debug!("truncating to {} pixels", to_length);

        let mut limit_idx = None;
        for (i, cumul_length) in self.cumulative_lengths.iter().enumerate() {
            if cumul_length.into_inner() > to_length {
                limit_idx = Some(i);
                break;
            }
        }

        let limit_idx = match limit_idx {
            Some(v) if v > 0 => v,
            _ => return,
        };

        let prev_idx = limit_idx - 1;
        let a = self.spline_points[prev_idx];
        let b = self.spline_points[limit_idx];
        let a_len = self.cumulative_lengths[prev_idx];
        debug!("a={:?} (a_len={}) b={:?}", a, b, a_len);
        let remain = to_length - a_len.into_inner();
        let mid = Math::point_on_line(a, b, remain);
        debug!("remain={:?} mid={:?}", remain, mid);

        self.spline_points[limit_idx] = mid;
        self.cumulative_lengths[limit_idx] = NotNan::new(to_length).unwrap();
        debug!("spline_points[{}] = {:?}", limit_idx, mid);
        debug!("cumulative_lengths[{}] = {:?}", limit_idx, to_length);

        self.spline_points.truncate(limit_idx + 1);
        self.cumulative_lengths.truncate(limit_idx + 1);
        debug!("truncated to len {}", limit_idx + 1);
    }

    /// Return the pixel length of this spline
    pub fn pixel_length(&self) -> f64 {
        self.cumulative_lengths.last().unwrap().into_inner()
    }

    /// Return the endpoint of this spline
    pub fn end_point(&self) -> P {
        self.spline_points.last().cloned().unwrap()
    }

    /// Calculate the angle at the given length on the slider
    fn angle_at_length(&self, length: f64) -> P {
        let _length_notnan = NotNan::new(length).unwrap();
        // match self.cumulative_lengths.binary_search(&length_notnan) {
        //     Ok(_) => {}
        //     Err(_) => {}
        // }
        todo!()
    }

    /// Calculate the point at which the slider ball would be after it has traveled a distance of
    /// `length` into the slider.
    pub fn point_at_length(&self, length: f64) -> P {
        let length_notnan = NotNan::new(length).unwrap();
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
                (p2 - p1) * P::new(proportion, proportion) + p1
            }
        }
    }
}

type P = Point<f64>;

fn subdivide(control_points: &[P], l: &mut [P], r: &mut [P], midpoints_buf: &mut [P]) {
    let count = control_points.len();
    midpoints_buf.copy_from_slice(control_points);

    for i in 0..count {
        l[i] = midpoints_buf[0];
        r[count - i - 1] = midpoints_buf[count - i - 1];

        for j in 0..count - i - 1 {
            midpoints_buf[j] = (midpoints_buf[j] + midpoints_buf[j + 1]) / P::new(2.0, 2.0);
        }
    }
}

fn approximate(
    control_points: &[P],
    output: &mut Vec<P>,
    l_buf: &mut [P],
    r_buf: &mut [P],
    midpoints_buf: &mut [P],
) {
    let count = control_points.len();

    subdivide(&control_points, l_buf, r_buf, midpoints_buf);

    l_buf[count..(count * 2) - 1].clone_from_slice(&r_buf[1..count]);

    output.push(control_points[0]);

    for i in 1..count - 1 {
        let index = 2 * i;
        let p = (l_buf[index] * P::new(2.0, 2.0) + l_buf[index - 1] + l_buf[index + 1])
            * P::new(0.25, 0.25);
        output.push(p);
    }
}

fn is_flat_enough(control_points: &[P], tolerance_sq: f64) -> bool {
    for i in 1..control_points.len() - 1 {
        if (control_points[i - 1] - control_points[i] * P::new(2.0, 2.0) + control_points[i + 1])
            .length_squared()
            > tolerance_sq
        {
            return false;
        }
    }

    true
}

fn create_singlebezier(output: &mut Vec<P>, control_points: &[P]) {
    let count = control_points.len();
    const TOLERANCE: f64 = 0.25;
    const TOLERANCE_SQ: f64 = TOLERANCE * TOLERANCE;

    if count == 0 {
        return;
    }

    let mut to_flatten: Vec<Vec<P>> = Vec::new();
    let mut free_buffers: Vec<Vec<P>> = Vec::new();

    let last_control_point = control_points[count - 1];
    to_flatten.push(control_points.to_vec());

    let mut left_child = vec![P::new(0.0, 0.0); count * 2 - 1];

    let mut l_buf = vec![P::new(0.0, 0.0); count * 2 - 1];
    let mut r_buf = vec![P::new(0.0, 0.0); count];
    let mut midpoints_buf = vec![P::new(0.0, 0.0); count];

    while !to_flatten.is_empty() {
        let mut parent = to_flatten.pop().unwrap();
        if is_flat_enough(&parent, TOLERANCE_SQ) {
            approximate(
                &parent,
                output,
                &mut l_buf[..count * 2 - 1],
                &mut r_buf[..count],
                &mut midpoints_buf[..count],
            );
            free_buffers.push(parent);
            continue;
        }

        let mut right_child = free_buffers
            .pop()
            .unwrap_or_else(|| vec![P::new(0.0, 0.0); count]);

        subdivide(
            &parent,
            &mut left_child,
            &mut right_child,
            &mut midpoints_buf[..count],
        );

        // We re-use the buffer of the parent for one of the children, so that we save one allocation per iteration.
        parent[..count].clone_from_slice(&left_child[..count]);

        to_flatten.push(right_child);
        to_flatten.push(parent);
    }

    output.push(last_control_point);
}
