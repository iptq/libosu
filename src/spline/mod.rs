mod bezier;

use ordered_float::NotNan;

use crate::float::compare_eq_f64;
use crate::hitobject::SliderSplineKind;
use crate::math::{Math, Point};

use self::bezier::{create_singlebezier, P};

const CATMULL_DETAIL: usize = 50;

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
    let mut kind = kind;
    let mut control_points = control_points.to_vec();

    // no matter what, if there's 2 control points, it's linear
    if control_points.len() == 2 {
      kind = SliderSplineKind::Linear;
    }

    // if there's 3 points but the 3 points occur on the same line then it's also linear
    // we can also safely remove the middle one
    // TODO: if the 3 points coincide then this gets fucked
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
        let mut idx = 0;
        let mut whole = Vec::new();
        let mut cumul_length = 0.0;
        let mut last_circ: Option<P> = None;

        let mut check_push = |whole: &mut Vec<P>, point: P| -> bool {
          let result;
          if let Some(circ) = last_circ {
            let distance = circ.distance(point);
            let total_len = cumul_length + distance;
            if let Some(pixel_length) = pixel_length {
              if total_len < pixel_length {
                whole.push(point);
                cumul_length += circ.distance(point);
                last_circ = Some(point);
                result = true;
              } else {
                let push_amt = pixel_length - cumul_length;
                let new_end = Math::point_on_line(circ, point, push_amt);
                whole.push(new_end);
                last_circ = Some(new_end);
                result = false;
              }
            } else {
              whole.push(point);
              cumul_length += circ.distance(point);
              last_circ = Some(point);
              result = true;
            }
          } else {
            whole.push(point);
            last_circ = Some(point);
            result = true;
          }
          result
        };

        // TODO: hack to allow breaks
        #[allow(clippy::never_loop)]
        'outer: loop {
          // split the curve by red-anchors
          for i in 1..points.len() {
            if compare_eq_f64(points[i].x, points[i - 1].x)
              && compare_eq_f64(points[i].y, points[i - 1].y)
            {
              let mut spline = Vec::new();
              create_singlebezier(&mut spline, &points[idx..i]);

              // check if it's equal to the last thing that was added to whole
              if let Some(last) = whole.last() {
                if spline[0] != *last && !check_push(&mut whole, spline[0]) {
                  break 'outer;
                }
              } else if !check_push(&mut whole, spline[0]) {
                break 'outer;
              }

              // add points, making sure no 2 are the same
              for points in spline.windows(2) {
                if points[0] != points[1] && !check_push(&mut whole, points[1])
                {
                  break 'outer;
                }
              }

              idx = i;
              continue;
            }
          }

          let mut spline = Vec::new();
          create_singlebezier(&mut spline, &points[idx..]);

          if let Some(last) = whole.last() {
            if spline[0] != *last && !check_push(&mut whole, spline[0]) {
              break 'outer;
            }
          } else if !check_push(&mut whole, spline[0]) {
            break 'outer;
          }
          for points in spline.windows(2) {
            if points[0] != points[1] && !check_push(&mut whole, points[1]) {
              break 'outer;
            }
          }
          break;
        }
        whole
      }
      SliderSplineKind::Catmull => {
        let mut path =
          Vec::with_capacity((points.len() - 1) * CATMULL_DETAIL * 2);

        for j in 0..points.len() - 1 {
          let v1 = match j {
            n if n > 0 => points[j - 1],
            _ => points[0],
          };

          let v2 = points[j];

          let v3 = match j + 1 {
            n if n < points.len() => points[j + 1],
            _ => v2 + (v2 - v1),
          };

          let v4 = match j + 2 {
            n if n < points.len() => points[j + 2],
            _ => v3 + (v3 - v2),
          };

          for c in 0..CATMULL_DETAIL {
            path.push(Math::catmull_find_point(
              v1,
              v2,
              v3,
              v4,
              c as f64 / CATMULL_DETAIL as f64,
            ));
            path.push(Math::catmull_find_point(
              v1,
              v2,
              v3,
              v4,
              (c + 1) as f64 / CATMULL_DETAIL as f64,
            ));
          }
        }

        path
      }
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
  pub fn angle_at_length(&self, length: f64) -> f64 {
    // dumb way of calculating
    // TODO: is it worth it using a different algorithm?
    let mut cands = vec![];
    const EPSILON: f64 = 0.001;
    cands.push(self.point_at_length(length - EPSILON));
    cands.push(self.point_at_length(length));
    cands.push(self.point_at_length(length + EPSILON));
    cands.dedup();

    match cands.as_slice() {
      &[a, b, ..] => (a.y - b.y).atan2(a.x - b.x),
      _ => panic!("uhhhhh"),
    }
  }

  /// Calculate the point at which the slider ball would be after it has traveled a distance of
  /// `length` into the slider.
  pub fn point_at_length(&self, length: f64) -> P {
    let length_notnan = NotNan::new(length).unwrap();
    match self.cumulative_lengths.binary_search(&length_notnan) {
      Ok(idx) => self.spline_points[idx],

      Err(idx) => {
        // if it's out of bounds, just return the bounds
        let n = self.spline_points.len();
        if idx == 0 && self.spline_points.len() > 2 {
          return self.spline_points[0];
        } else if idx == n {
          return self.spline_points[n - 1];
        }

        // there's no point at this length, give us an approximation based on the two points
        // around it instead
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

#[cfg(test)]
mod tests {
  use proptest::collection::vec;
  use proptest::option;
  use proptest::prelude::*;
  use proptest::proptest;

  use super::Spline;
  use crate::hitobject::SliderSplineKind;
  use crate::math::Point;

  proptest! {
      #![proptest_config(ProptestConfig {
          cases: 2,
          verbose: 2,
          timeout: 3000,
          ..ProptestConfig::default()
      })]
      #[test]
      fn doesnt_crash(
          kind: SliderSplineKind,
          pixel_length in option::of(any::<f64>()),
          control in vec((0..512, 0..384), 10))
      {
          let control = control.into_iter().map(|(x, y)| Point::new(x, y)).collect::<Vec<_>>();
          Spline::from_control(kind, &control, pixel_length);
      }
  }
}
