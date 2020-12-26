use anyhow::Result;

use crate::{
    AbsoluteTime, Additions, Beatmap, HitObject, HitObjectKind, Hitsound, Point, SampleSet,
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
        let addition = parts[4].parse::<u32>()?;

        let start_time = TimeLocation::Absolute(AbsoluteTime::new(timestamp));
        let extras;

        // color is the top 3 bits of the "type" string, since there's a possible of 8 different
        // combo colors max
        let skip_color = (obj_type >> 4) & 0b111;

        let new_combo = (obj_type & 4) == 4;
        let kind = if (obj_type & 1) == 1 {
            extras = parts[5];
            HitObjectKind::Circle
        } else if (obj_type & 2) == 2 {
            let mut ctl_parts = parts[5].split('|').collect::<Vec<_>>();
            let repeats = parts[6].parse::<u32>()?;
            let slider_type = ctl_parts.remove(0);

            extras = if parts.len() < 11 {
                "0:0:0:0:"
            } else {
                parts[10]
            };

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

            HitObjectKind::Slider {
                kind: match slider_type {
                    "L" => SliderSplineKind::Linear,
                    "B" => SliderSplineKind::Bezier,
                    "C" => SliderSplineKind::Catmull,
                    "P" => SliderSplineKind::Perfect,
                    _ => bail!("Invalid slider type."),
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
            }
        } else if (obj_type & 8) == 8 {
            let end_time = parts[5].parse::<i32>()?;
            extras = parts[6];
            HitObjectKind::Spinner {
                end_time: TimeLocation::Absolute(AbsoluteTime::new(end_time)),
            }
        } else {
            bail!("Invalid object type.")
        };

        let extra_parts = extras.split(':').collect::<Vec<_>>();
        let sample_set = extra_parts[0].parse::<i32>()?;
        let addition_set = extra_parts[1].parse::<i32>()?;
        let custom_index = extra_parts[2].parse::<i32>()?;
        let sample_volume = extra_parts[3].parse::<i32>()?;
        let filename = extra_parts[4].to_owned();

        // TODO: handle extras field
        let hitsound = Hitsound {
            additions: Additions(addition),
            sample: SampleSet::Normal, // TODO
            time: match kind {
                HitObjectKind::Spinner { ref end_time } => end_time.clone(),
                _ => start_time.clone(),
            },

            sample_set,
            addition_set,
            custom_index,
            sample_volume,
            filename,
        };

        let hit_obj = HitObject {
            kind,
            pos: Point(x, y),
            new_combo,
            hitsound,
            timing_point: None,
            skip_color,
            start_time,
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

        let hitsound = self.hitsound.additions.0;
        let extras = format!(
            "{}:{}:{}:{}:{}",
            self.hitsound.sample_set,
            self.hitsound.addition_set,
            self.hitsound.custom_index,
            self.hitsound.sample_volume,
            self.hitsound.filename
        );

        let type_specific = match &self.kind {
            HitObjectKind::Slider {
                ref kind,
                ref repeats,
                ref control,
                ref pixel_length,
                ..
            } => {
                let edge_hitsounds = "0";
                let edge_additions = "0:0";
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
            &HitObjectKind::Spinner { ref end_time } => format!("{},", end_time.as_milliseconds()),
            _ => String::new(),
        };

        let line = format!(
            "{},{},{},{},{},{}{}",
            self.pos.0,
            self.pos.1,
            self.start_time.as_milliseconds(),
            obj_type,
            hitsound,
            type_specific,
            extras
        );
        Ok(line)
    }
}
