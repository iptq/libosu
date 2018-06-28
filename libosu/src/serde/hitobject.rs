use failure::Error;

use serde::Deserializer;
use HitObject;
use HitObjectKind;
use Point;

impl<'map> Deserializer<'map> for HitObject {
    type Output = HitObject;
    fn parse(input: &'map str) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let hit_obj = HitObject {
            kind: HitObjectKind::Circle,
            pos: Point(0, 0),
            start_time: 0,
        };

        Ok(hit_obj)
    }
}
