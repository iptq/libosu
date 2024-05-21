//! Make sure that the timing information is preserved as floats

use std::fs::File;

use anyhow::Result;
use libosu::prelude::*;

const F64_EPSILON: f64 = 0.0001;
const TEST_DATA: &[(&str, &[(usize, f64)])] = &[(
  "tests/files/129891.osu",
  &[
    (0, 270.002700027),
    (1, -100.0),
    (2, -100.0),
    (8, -83.3333333333333),
  ],
)];

#[test]
fn test_preserved_timing_info() -> Result<()> {
  for (path, data) in TEST_DATA {
    let file = File::open(path)?;
    let beatmap = Beatmap::parse(file)?;

    for (idx, val) in *data {
      let tp = beatmap.timing_points.get(*idx).unwrap();
      let s = tp.to_string();
      let parts = s.split(",").collect::<Vec<_>>();
      let actual_val = parts[1].parse::<f64>()?;
      assert!(
        (actual_val - val).abs() < F64_EPSILON,
        "expected {}, got {}",
        val,
        actual_val
      );
    }
  }

  Ok(())
}
