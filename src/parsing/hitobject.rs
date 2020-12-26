use num_traits::FromPrimitive;

use crate::parsing::{Error, Result};
use crate::{
    AbsoluteTime, Additions, Beatmap, HitObject, HitObjectKind, Point, SampleInfo, SampleSet,
    SliderSplineKind, TimeLocation,
};

impl HitObject {
    /// Creates a HitObject from the *.osz format
    pub fn from_osz(input: impl AsRef<str>, parent: &Beatmap) -> Result<HitObject> {
        let parts = input.as_ref().split(',').collect::<Vec<_>>();

        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        let timestamp = parts[2].parse::<i32>()?;
        let obj_type = parts[3].parse::<i32>()?;
        let additions_bits = parts[4].parse::<u32>()?;
        let additions = Additions::from_bits(additions_bits).unwrap();

        let start_time = TimeLocation::Absolute(AbsoluteTime::new(timestamp));

        // color is the top 3 bits of the "type" string, since there's a possible of 8 different
        // combo colors max
        let skip_color = (obj_type >> 4) & 0b111;

        let new_combo = (obj_type & 4) == 4;
        let sample_info;
        let kind = match obj_type {
            // hit circle
            o if (o & 1) == 1 => {
                sample_info = parse_hitsample(parts[5])?;
                HitObjectKind::Circle
            }
            //slider
            o if (o & 2) == 2 => {
                let mut ctl_parts = parts[5].split('|').collect::<Vec<_>>();
                let repeats = parts[6].parse::<u32>()?;
                let slider_type = ctl_parts.remove(0);

                // slider duration = pixelLength / (100.0 * SliderMultiplier) * BeatDuration
                // from the osu wiki
                let pixel_length = parts[7].parse::<f64>()?;
                let beat_duration = parent
                    .locate_timing_point(&start_time)
                    .unwrap()
                    .get_beat_duration();
                let duration = (pixel_length as f64 * beat_duration
                    / (100.0 * parent.difficulty.slider_multiplier as f64))
                    as u32;

                let edge_hitsounds = if parts.len() > 8 {
                    parts[8]
                        .split('|')
                        .map(|n| n.parse::<u32>().map(|b| Additions::from_bits(b).unwrap()))
                        .collect::<Result<Vec<_>, _>>()?
                } else {
                    vec![Additions::empty()]
                };

                let edge_additions = if parts.len() > 9 {
                    parts[9]
                        .split('|')
                        .map(|s| {
                            let s2 = s.split(':').collect::<Vec<_>>();
                            let normal = s2[0].parse::<u32>()?;
                            let additions = s2[1].parse::<u32>()?;
                            Ok((
                                SampleSet::from_u32(normal).unwrap(),
                                SampleSet::from_u32(additions).unwrap(),
                            ))
                        })
                        .collect::<Result<Vec<_>>>()?
                } else {
                    vec![(SampleSet::None, SampleSet::None)]
                };

                sample_info = if parts.len() > 10 {
                    parse_hitsample(parts[10])?
                } else {
                    SampleInfo::default()
                };

                HitObjectKind::Slider {
                    kind: match slider_type {
                        "L" => SliderSplineKind::Linear,
                        "B" => SliderSplineKind::Bezier,
                        "C" => SliderSplineKind::Catmull,
                        "P" => SliderSplineKind::Perfect,
                        s => return Err(Error::InvalidSliderType(s.to_owned())),
                    },
                    control: ctl_parts
                        .into_iter()
                        .map(|s| {
                            let p = s.split(':').collect::<Vec<_>>();
                            Point(p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap())
                        })
                        .collect(),
                    repeats,
                    pixel_length,
                    duration,
                    edge_hitsounds,
                    edge_additions,
                }
            }
            // spinner
            o if (o & 8) == 8 => {
                let end_time = parts[5].parse::<i32>()?;
                sample_info = parse_hitsample(parts[6])?;
                HitObjectKind::Spinner {
                    end_time: TimeLocation::Absolute(AbsoluteTime::new(end_time)),
                }
            }
            o => {
                return Err(Error::InvalidObjectType(o));
            }
        };

        let hit_obj = HitObject {
            kind,
            pos: Point(x, y),
            new_combo,
            additions,
            timing_point: None,
            skip_color,
            start_time,
            sample_info,
        };

        Ok(hit_obj)
    }

    /// Serializes this HitObject into the *.osz format.
    pub fn as_osz(&self) -> Result<String> {
        let obj_type = match self.kind {
            HitObjectKind::Circle => 1,
            HitObjectKind::Slider { .. } => 2,
            HitObjectKind::Spinner { .. } => 8,
        } | if self.new_combo { 4 } else { 0 }
            | self.skip_color;

        let hitsample = hitsample_str(&self.sample_info);

        let type_specific = match self.kind {
            HitObjectKind::Slider {
                ref kind,
                ref repeats,
                ref control,
                ref pixel_length,
                ref edge_hitsounds,
                ref edge_additions,
                ..
            } => {
                let edge_hitsounds = edge_hitsounds
                    .iter()
                    .map(|f| f.bits().to_string())
                    .collect::<Vec<_>>()
                    .join("|");
                let edge_additions = edge_additions
                    .iter()
                    .map(|f| format!("{}:{}", f.0 as u32, f.1 as u32))
                    .collect::<Vec<_>>()
                    .join("|");
                format!(
                    "{}|{},{},{},{},{},",
                    match kind {
                        SliderSplineKind::Linear => "L",
                        SliderSplineKind::Bezier => "B",
                        SliderSplineKind::Catmull => "C",
                        SliderSplineKind::Perfect => "P",
                    },
                    control
                        .iter()
                        .map(|point| format!("{}:{}", point.0, point.1))
                        .collect::<Vec<_>>()
                        .join("|"),
                    repeats,
                    pixel_length,
                    edge_hitsounds,
                    edge_additions,
                )
            }
            HitObjectKind::Spinner { ref end_time } => format!("{},", end_time.as_milliseconds()),
            _ => String::new(),
        };

        let line = format!(
            "{},{},{},{},{},{}{}",
            self.pos.0,
            self.pos.1,
            self.start_time.as_milliseconds(),
            obj_type,
            self.additions.bits(),
            type_specific,
            hitsample,
        );
        Ok(line)
    }
}

fn parse_hitsample(line: &str) -> Result<SampleInfo> {
    let extra_parts = line.split(':').collect::<Vec<_>>();
    let sample_set = extra_parts[0].parse::<u32>()?;
    let addition_set = extra_parts[1].parse::<u32>()?;
    let custom_index = extra_parts[2].parse::<i32>()?;
    let sample_volume = extra_parts[3].parse::<i32>()?;
    let filename = extra_parts[4].to_owned();

    // TODO: handle extras field
    let hitsound = SampleInfo {
        addition_set: SampleSet::from_u32(addition_set).unwrap(),
        sample_set: SampleSet::from_u32(sample_set).unwrap(),

        custom_index,
        sample_volume,
        filename,
    };

    Ok(hitsound)
}

fn hitsample_str(hitsound: &SampleInfo) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        hitsound.sample_set as u32,
        hitsound.addition_set as u32,
        hitsound.custom_index,
        hitsound.sample_volume,
        hitsound.filename
    )
}
