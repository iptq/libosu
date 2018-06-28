use failure::Error;

use serde::OszParser;
use TimeLocation;
use TimingPoint;
use TimingPointKind;

impl<'map> OszParser<'map> for TimingPoint<'map> {
    type Output = TimingPoint<'map>;
    fn parse(input: &'map str) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let timing_point = TimingPoint {
            kind: TimingPointKind::Uninherited { bpm: 1.0 },
            time: TimeLocation::Absolute(0),
        };

        Ok(timing_point)
    }
}
