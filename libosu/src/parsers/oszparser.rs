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

pub trait OszParser<'src> {
    type Output;
    fn parse(input: &'src str) -> Result<Self::Output, Error>;
}

impl<'map> OszParser<'map> for Beatmap<'map> {
    type Output = Beatmap<'map>;
    fn parse(input: &'map str) -> Result<Beatmap, Error> {
        let mut section = "Version".to_owned();
        let mut version = 0;
        let mut audio_filename = String::new();
        let mut hit_objects = Vec::new();
        let mut timing_points = Vec::new();
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
                    hit_objects.push(obj);
                }
                _ => (),
            }
        }
        Ok(Beatmap {
            version,
            audio_filename,
            hit_objects,
            timing_points,
        })
    }
}

impl<'map> OszParser<'map> for HitObject {
    type Output = HitObject;
    fn parse(input: &'map str) -> Result<Self::Output, Error> {
        let parts = input.split(",");

        let obj = HitObject {
            kind: HitObjectKind::Circle,
            pos: Point(0, 0),
            start_time: 0,
        };
        Ok(obj)
    }
}

impl<'map> OszParser<'map> for TimingPoint<'map> {
    type Output = TimingPoint<'map>;
    fn parse(input: &'map str) -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}
