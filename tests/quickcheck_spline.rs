use libosu::{hitobject::SliderSplineKind, math::Point, spline::Spline};
use quickcheck_macros::quickcheck;
use quickcheck::{Arbitrary, Gen, TestResult};

#[derive(Clone, Debug)]
struct OsuPoint(Point<i32>);

impl Arbitrary for OsuPoint {
    fn arbitrary(g: &mut Gen) -> OsuPoint {
        let a = (u32::arbitrary(g) % 512) as i32;
        let b = (u32::arbitrary(g) % 384) as i32;
        OsuPoint(Point(a, b))
    }
}

#[derive(Clone, Debug)]
struct OsuSlider(Vec<Point<i32>>, f64);

impl Arbitrary for OsuSlider {
    fn arbitrary(g: &mut Gen) -> OsuSlider {
        let len = (usize::arbitrary(g) % 20) + 2;
        let mut last: Option<OsuPoint> = None;
        let mut total = 0.0;
        let mut points = vec![];

        for _ in 0..len {
            let curr = OsuPoint::arbitrary(g);
            if let Some(last) = last {
                let dx = (curr.0.0-last.0.0) as f64;
                let dy = (curr.0.1-last.0.1) as f64;
                total += (dx*dx+dy*dy).sqrt();
            }
            points.push(curr.0);
            last = Some(curr)
        }

        let var = loop {
            let v = f64::arbitrary(g);
            if !v.is_nan() {
                break v;
            }
        };
        let b = 3.0;
        let delta = total + (2.0 * b / (1.0 + (-0.2 * var).exp()) - b);
        OsuSlider(points, delta)
    }
}

#[quickcheck]
fn spline_isnt_empty(kind: SliderSplineKind, slider: OsuSlider) -> TestResult {
    let control = slider.0.iter().map(|p| Point(p.0, p.1)).collect::<Vec<_>>();
    let spline = Spline::from_control(kind, control.as_ref(), slider.1);
    TestResult::from_bool(spline.spline_points.len() >= 2)
}
