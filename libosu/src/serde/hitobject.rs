use failure::Error;

use serde::{Deserializer, OsuFormat, Serializer};
use HitObject;
use HitObjectKind;
use Point;
use TimeLocation;

impl<'map> Deserializer<OsuFormat> for HitObject<'map> {
    type Output = HitObject<'map>;
    fn deserialize(input: OsuFormat) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let hit_obj = HitObject {
            kind: HitObjectKind::Circle,
            pos: Point(0, 0),
            start_time: TimeLocation::Absolute(0),
        };

        Ok(hit_obj)
    }
}
