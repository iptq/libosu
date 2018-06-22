use failure::Error;

use Beatmap;

pub struct OszParser;

impl OszParser {
    pub fn parse<'a>() -> Result<Beatmap<'a>, Error> {
        bail!("shiet");
    }
}
