use std::collections::BTreeSet;

use failure::Error;

use serde::{Deserializer, OsuFormat, Serializer};
use TimeLocation;
use TimingPoint;
use TimingPointKind;

impl<'map> Deserializer<OsuFormat> for TimingPoint<'map> {
    type Output = TimingPoint<'map>;
    fn deserialize(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",").collect::<Vec<_>>();

        let timestamp = parts[0].parse::<i32>()?;
        let mpb = parts[1].parse::<f64>()?;
        let meter = parts[2].parse::<u32>()?;
        let sample_type = parts[3].parse::<i32>()?;
        let sample_set = parts[4].parse::<i32>()?;
        let volume = parts[5].parse::<i32>()?;
        let inherited = parts[6].parse::<i32>()? > 0;
        let kiai = parts[7].parse::<i32>()? > 0;

        // calculate bpm from mpb
        let bpm = 60_000.0 / mpb;

        let timing_point = TimingPoint {
            kind: TimingPointKind::Uninherited {
                bpm,
                meter,
                children: BTreeSet::new(),
            },
            time: TimeLocation::Absolute(0),
        };

        Ok(timing_point)
    }
}
