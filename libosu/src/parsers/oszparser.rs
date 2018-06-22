use failure::Error;

use Beatmap;
use HitObject;
use TimingPoint;

pub trait OszParser {
    type Output;
    fn parse(input: &str) -> Result<Self::Output, Error>;
}

impl OszParser for Beatmap {
    type Output = Beatmap;
    fn parse(input: &str) -> Result<Beatmap, Error> {
        let mut section = "Format";
        let mut format = 0;
        for line in input.split(" ") {}
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
