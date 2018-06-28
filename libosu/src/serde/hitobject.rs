use failure::Error;

use serde::Deserializer;
use HitObject;
use HitObjectKind;
use Point;
use TimeLocation;

impl<'map> Deserializer<'map> for HitObject<'map> {
    type Output = HitObject<'map>;
    fn parse(input: &'map str) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let hit_obj = HitObject {
            kind: HitObjectKind::Circle,
            pos: Point(0, 0),
            start_time: TimeLocation::Absolute(0),
        };

        Ok(hit_obj)
    }
}
