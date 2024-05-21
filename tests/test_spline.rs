use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use std::{fs::File, time::Instant};

use anyhow::Result;
use libosu::{
  beatmap::Beatmap,
  hitobject::{HitObjectKind, SliderSplineKind},
  math::Point,
  spline::Spline,
};

macro_rules! test_spline {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");

                let beatmap = Beatmap::from_str(&contents).expect("couldn't parse");

                for ho in beatmap.hit_objects.iter() {
                    if let HitObjectKind::Slider(info) = &ho.kind {
                        let mut control_points = vec![ho.pos];
                        control_points.extend(&info.control_points);
                        let spline = Spline::from_control(
                            info.kind,
                            control_points.as_ref(),
                            Some(info.pixel_length),
                        );

                        assert!(spline.spline_points.len() >= 2, "spline for {} is empty!", ho.to_string());
                    }
                }
            }
        )*
    };
}

test_spline! {
    test_spline_774965: 774965,
    test_spline_804683: 804683,
    test_spline_1595588: 1595588,
}

#[test]
fn test_spline_points() -> Result<()> {
  let spline_points_list =
    BufReader::new(File::open("tests/spline_points_list.in")?);
  let mut expected_spline_points = Vec::new();
  for line in spline_points_list.lines() {
    let line = line?;
    let mut parts = line.split(",");
    let x = parts.next().unwrap().parse::<f64>()?;
    let y = parts.next().unwrap().parse::<f64>()?;
    expected_spline_points.push((x, y));
  }

  for _ in 0..10000 {
    let spline = Spline::from_control(
      SliderSplineKind::Bezier,
      &[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 10 },
        Point { x: 20, y: 5 },
      ],
      Some(100.0),
    );

    let expected_points = expected_spline_points.len();
    let actual_points = spline.spline_points.len();
    assert_eq!(
      expected_points, actual_points,
      "expected {} points, got {}",
      expected_points, actual_points
    );

    for (i, (Point { x: ax, y: ay }, (ex, ey))) in spline
      .spline_points
      .iter()
      .zip(expected_spline_points.iter())
      .enumerate()
    {
      assert!(
        (ex - ax).abs() < 0.001,
        "ln{}x: expected {}, got {}",
        i,
        ex,
        ax
      );
      assert!(
        (ey - ay).abs() < 0.001,
        "ln{}y: expected {}, got {}",
        i,
        ex,
        ax
      );
    }
  }

  Ok(())
}
