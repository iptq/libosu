use crate::math::Point;

pub type P = Point<f64>;

fn subdivide(
  control_points: &[P],
  l: &mut [P],
  r: &mut [P],
  midpoints_buf: &mut [P],
) {
  let count = control_points.len();
  midpoints_buf.copy_from_slice(control_points);

  for i in 0..count {
    l[i] = midpoints_buf[0];
    r[count - i - 1] = midpoints_buf[count - i - 1];

    for j in 0..count - i - 1 {
      midpoints_buf[j] =
        (midpoints_buf[j] + midpoints_buf[j + 1]) / P::new(2.0, 2.0);
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

  subdivide(control_points, l_buf, r_buf, midpoints_buf);

  l_buf[count..(count * 2) - 1].clone_from_slice(&r_buf[1..count]);

  output.push(control_points[0]);

  for i in 1..count - 1 {
    let index = 2 * i;
    let p =
      (l_buf[index] * P::new(2.0, 2.0) + l_buf[index - 1] + l_buf[index + 1])
        * P::new(0.25, 0.25);
    output.push(p);
  }
}

fn is_flat_enough(control_points: &[P], tolerance_sq: f64) -> bool {
  for i in 1..control_points.len() - 1 {
    if (control_points[i - 1] - control_points[i] * P::new(2.0, 2.0)
      + control_points[i + 1])
      .length_squared()
      > tolerance_sq
    {
      return false;
    }
  }

  true
}

/// The bezier algorithm as implemented by osu.
///
/// This seems to be an iterative version of [De Casteljau's algorithm][1], splitting curves in
/// half until they're "flat enough" as evaluated by [`is_flat_enough`], and then lerp'd.
///
/// [1]:
pub fn create_singlebezier(output: &mut Vec<P>, control_points: &[P]) {
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
