use libosu::{hitobject::SliderSplineKind, math::Point, spline::Spline};
use quickcheck::{Arbitrary, Gen, TestResult};
use quickcheck_macros::quickcheck;

#[derive(Clone, Debug)]
struct OsuPoint(Point<i32>);

fn nonnan(g: &mut Gen) -> f64 {
    loop {
        let v = f64::arbitrary(g);
        if !v.is_nan() {
            break v;
        }
    }
}

fn sigmoid_clamp(midpoint: f64, amp: f64, inp: f64) -> f64 {
    midpoint + (2.0 * amp / (1.0 + (-0.2 * inp).exp()) - amp)
}

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
        let len = (usize::arbitrary(g) % 500) + 2;
        let mut last: Option<OsuPoint> = None;
        let mut total = 0.0;
        let mut points = vec![];

        for _ in 0..len {
            let curr = OsuPoint::arbitrary(g);
            if let Some(last) = last {
                let dx = (curr.0 .0 - last.0 .0) as f64;
                let dy = (curr.0 .1 - last.0 .1) as f64;
                total += (dx * dx + dy * dy).sqrt();
            }
            points.push(curr.0);
            last = Some(curr)
        }

        let var = nonnan(g);
        let res = sigmoid_clamp(total, 3.0, var);
        OsuSlider(points, res)
    }
}

#[derive(Clone, Debug)]
struct Nonnan(f64);

impl Arbitrary for Nonnan {
    fn arbitrary(g: &mut Gen) -> Nonnan {
        Nonnan(nonnan(g))
    }
}

#[quickcheck]
fn spline_isnt_empty(kind: SliderSplineKind, slider: OsuSlider) -> TestResult {
    let control = slider.0.iter().map(|p| Point(p.0, p.1)).collect::<Vec<_>>();
    let spline = Spline::from_control(kind, control.as_ref(), slider.1);
    TestResult::from_bool(spline.spline_points.len() >= 2)
}

#[quickcheck]
fn point_at_length(kind: SliderSplineKind, slider: OsuSlider, len: Nonnan) -> TestResult {
    let control = slider.0.iter().map(|p| Point(p.0, p.1)).collect::<Vec<_>>();
    let spline = Spline::from_control(kind, control.as_ref(), slider.1);
    let len = sigmoid_clamp(slider.1 / 2.0, slider.1 / 2.0, len.0);
    let point = spline.point_at_length(len);
    TestResult::from_bool(true)
}
