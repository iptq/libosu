use failure::Error;

use serde::{Deserializer, OsuFormat, Serializer};
use HitObject;
use HitObjectKind;
use Point;
use SliderSplineKind;
use TimeLocation;

impl<'map> Deserializer<OsuFormat> for HitObject<'map> {
    type Output = HitObject<'map>;
    fn deserialize(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",").collect::<Vec<_>>();

        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        let timestamp = parts[2].parse::<i32>()?;
        let obj_type = parts[3].parse::<i32>()?;
        let hitsound = parts[4].parse::<i32>()?;

        let new_combo = (obj_type & 4) == 4;
        let kind = if (obj_type & 1) == 1 {
            HitObjectKind::Circle
        } else if (obj_type & 2) == 2 {
            let mut ctl_parts = parts[5].split("|").collect::<Vec<_>>();
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
            }
        } else if (obj_type & 8) == 8 {
            let end_time = parts[5].parse::<i32>()?;
            HitObjectKind::Spinner {
                end_time: TimeLocation::Absolute(end_time),
            }
        } else {
            bail!("Invalid object type.")
        };

        let hit_obj = HitObject {
            kind: kind,
            pos: Point(x, y),
            new_combo,
            start_time: TimeLocation::Absolute(timestamp),
        };

        Ok(hit_obj)
    }
}
