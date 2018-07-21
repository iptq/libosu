use failure::Error;

use osz::*;
use Additions;
use HitObject;
use HitObjectKind;
use Hitsound;
use Point;
use SampleSet;
use SliderSplineKind;
use TimeLocation;

impl<'map> OszDeserializer<OsuFormat> for HitObject<'map> {
    type Output = HitObject<'map>;
    fn deserialize_osz(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",").collect::<Vec<_>>();
        println!("parsing {:?}", input);

        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        let timestamp = parts[2].parse::<i32>()?;
        let obj_type = parts[3].parse::<i32>()?;
        let addition = parts[4].parse::<u32>()?;

        let start_time = TimeLocation::Absolute(timestamp);

        // color is the top 3 bits of the "type" string, since there's a possible of 8 different
        // combo colors max
        let skip_color = (obj_type >> 4) & 0b111;

        let new_combo = (obj_type & 4) == 4;
        let kind = if (obj_type & 1) == 1 {
            HitObjectKind::Circle
        } else if (obj_type & 2) == 2 {
            let mut ctl_parts = parts[5].split("|").collect::<Vec<_>>();
            let repeats = parts[6].parse::<u32>()?;
            let slider_type = ctl_parts.remove(0);

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
                        let p = s.split(":").collect::<Vec<_>>();
                        Point(p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap())
                    })
                    .collect(),
                repeats,
            }
        } else if (obj_type & 8) == 8 {
            let end_time = parts[5].parse::<i32>()?;
            HitObjectKind::Spinner {
                end_time: TimeLocation::Absolute(end_time),
            }
        } else {
            bail!("Invalid object type.")
        };

        // TODO: handle extras field
        let hitsound = Hitsound {
            additions: Additions(addition),
            sample: SampleSet::Normal, // TODO
            time: match &kind {
                &HitObjectKind::Spinner { end_time } => end_time,
                _ => start_time,
            },
        };

        let hit_obj = HitObject {
            kind: kind,
            pos: Point(x, y),
            new_combo,
            hitsound,
            timing_point: None,
            skip_color,
            start_time,
        };

        Ok(hit_obj)
    }
}

impl<'map> OszSerializer<OsuFormat> for HitObject<'map> {
    fn serialize_osz(&self) -> Result<OsuFormat, Error> {
        let obj_type = match &self.kind {
            &HitObjectKind::Circle => 1,
            &HitObjectKind::Slider { .. } => 2,
            &HitObjectKind::Spinner { .. } => 8,
        } | if self.new_combo { 4 } else { 0 } | self.skip_color;

        let mut line = format!(
            "{},{},{},{},{}",
            self.pos.0,
            self.pos.1,
            self.start_time.into_milliseconds(),
            obj_type,
            0,
        );
        match &self.kind {
            &HitObjectKind::Slider {
                ref kind,
                ref repeats,
                ..
            } => {
                line += &format!(
                    ",{}|0:0,{}",
                    match kind {
                        &SliderSplineKind::Linear => "L",
                        &SliderSplineKind::Bezier => "B",
                        &SliderSplineKind::Catmull => "C",
                        &SliderSplineKind::Perfect => "P",
                    },
                    repeats
                );
            }
            &HitObjectKind::Spinner { ref end_time } => {
                line += &format!(",{}", end_time.into_milliseconds());
            }
            _ => (),
        }
        Ok(line)
    }
}
