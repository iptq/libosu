use failure::Error;

use serde::{Deserializer, OsuFormat, Serializer};
use TimeLocation;
use TimingPoint;
use TimingPointKind;

impl<'map> Deserializer<OsuFormat> for TimingPoint<'map> {
    type Output = TimingPoint<'map>;
    fn deserialize(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let timing_point = TimingPoint {
            kind: TimingPointKind::Uninherited {
                bpm: 1.0,
                meter: 4,
                children: vec![],
            },
            time: TimeLocation::Absolute(0),
        };

        Ok(timing_point)
    }
}
