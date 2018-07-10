use std::collections::BTreeSet;

use failure::Error;

use osz::*;
use SampleSet;
use TimeLocation;
use TimingPoint;
use TimingPointKind;

impl<'map> OszDeserializer<OsuFormat> for TimingPoint<'map> {
    type Output = TimingPoint<'map>;
    fn deserialize_osz(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",").collect::<Vec<_>>();

        let timestamp = parts[0].parse::<i32>()?;
        let mpb = parts[1].parse::<f64>()?;
        let meter = parts[2].parse::<u32>()?;
        let sample_set = parts[3].parse::<i32>()?;
        let sample_index = parts[4].parse::<u32>()?;
        let volume = parts[5].parse::<u16>()?;
        let inherited = parts[6].parse::<i32>()? > 0;
        let kiai = parts[7].parse::<i32>()? > 0;

        // calculate bpm from mpb
        let bpm = 60_000.0 / mpb;

        let timing_point = TimingPoint {
            kind: if inherited {
                TimingPointKind::Inherited {
                    parent: None,
                    slider_velocity: 0.0, // TODO: calculate this from mpb
                }
            } else {
                TimingPointKind::Uninherited {
                    bpm,
                    meter,
                    children: BTreeSet::new(),
                }
            },
            kiai,
            sample_set: match sample_set {
                0 => SampleSet::None,
                1 => SampleSet::Normal,
                2 => SampleSet::Soft,
                3 => SampleSet::Drum,
                _ => panic!("Invalid sample set '{}'.", sample_set),
            },
            sample_index,
            volume,
            time: TimeLocation::Absolute(timestamp),
        };

        Ok(timing_point)
    }
}

impl<'map> OszSerializer<OsuFormat> for TimingPoint<'map> {
    fn serialize_osz(&self) -> Result<OsuFormat, Error> {
        let mpb;
        let inherited;
        match &self.kind {
            &TimingPointKind::Inherited { .. } => {
                mpb = 0.0;
                inherited = 1;
            }
            &TimingPointKind::Uninherited { ref bpm, .. } => {
                mpb = 60_000.0 / *bpm;
                inherited = 0;
            }
        };
        let line = format!(
            "{},{},{},{},{},{},{},{}",
            self.time.into_milliseconds(),
            mpb,
            0,
            self.sample_set as i32,
            self.sample_index,
            self.volume,
            inherited,
            if self.kiai { 1 } else { 0 },
        );
        Ok(line)
    }
}
