use failure::Error;

use Beatmap;

pub struct OszParser;

impl OszParser {
    pub fn parse() -> Result<Beatmap, Error> {
        bail!("shiet");
    }
}
