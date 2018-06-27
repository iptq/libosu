use failure::Error;
use regex::Regex;

use Beatmap;
use HitObject;
use HitObjectKind;
use Point;
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
        let mut Version = 0;
        let mut AudioFilename = String::new();
        let mut HitObjects = Vec::new();
        let mut TimingPoints = Vec::new();
        for line in input.lines() {
            match SECTION_HEADER_RGX.captures(line) {
                Some(captures) => {
                    section = String::from(&captures["name"]);
                    continue;
                }
                None => (),
            }
            println!("\"{}\" {}", section, line);
            match section.as_ref() {
                "HitObjects" => {
                    let obj = HitObject::parse(line)?;
                    HitObjects.push(obj);
                }
                _ => (),
            }
        }
        Ok(Beatmap {
            Version,
            AudioFilename,
            HitObjects,
            TimingPoints,
        })
    }
}

impl OszParser for HitObject {
    type Output = HitObject;
    fn parse<'src>(input: &'src str) -> Result<Self::Output, Error> {
        let obj = HitObject {
            kind: HitObjectKind::Circle,
            pos: Point(0, 0),
            start_time: 0,
        };
        Ok(obj)
    }
}

impl OszParser for TimingPoint {
    type Output = TimingPoint;
    fn parse<'src>(input: &'src str) -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}
