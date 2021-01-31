use std::io::Read;
use std::str::FromStr;
use std::{fs::File, time::Instant};

use libosu::{beatmap::Beatmap, hitobject::HitObjectKind, math::Point, spline::Spline};

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
fn test_spline_points() {
    let before = Instant::now();
    for _ in 0..10000 {
        let spline = Spline::from_control(
            libosu::prelude::SliderSplineKind::Bezier,
            &[
                Point { x: 0, y: 0 },
                Point { x: 0, y: 10 },
                Point { x: 20, y: 5 },
            ],
            Some(100.0),
        );
        assert_eq!(
            spline.spline_points,
            include!("spline_points_list.in"),
            "{:#?}\n!=\n{:#?}",
            spline.spline_points,
            include!("spline_points_list.in"),
        );
    }
    let elapsed = before.elapsed();
    panic!("{}ms elapsed", elapsed.as_millis());
}
