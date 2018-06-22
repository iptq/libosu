use failure::Error;
use regex::Regex;

use Beatmap;
use HitObject;
use TimingPoint;

lazy_static! {
    static ref SECTION_HEADER_RGX: Regex = Regex::new(r"\[(?P<name>[A-Za-z]+)\]").unwrap();
}

pub trait OszParser {
    type Output;
    fn parse(input: &str) -> Result<Self::Output, Error>;
}

impl OszParser for Beatmap {
    type Output = Beatmap;
    fn parse<'src>(input: &'src str) -> Result<Beatmap, Error> {
        let mut section = "Version".to_owned();
        let mut format = 0;
        for line in input.split(" ") {
            match SECTION_HEADER_RGX.captures(line) {
                Some(captures) => {
                    section = String::from(&captures["name"]);
                    continue;
                }
                None => (),
            }
        }
        Ok(Beatmap {
            format,//
        })
    }
}

impl OszParser for HitObject {
    type Output = HitObject;
    fn parse(input: &str) -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}

impl OszParser for TimingPoint {
    type Output = TimingPoint;
    fn parse(input: &str) -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}
