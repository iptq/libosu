use failure::Error;

use Beatmap;
use HitObject;
use TimingPoint;

pub trait OszParser {
    type Output;
    fn parse() -> Result<Self::Output, Error>;
}

impl OszParser for Beatmap {
    type Output = Beatmap;
    fn parse() -> Result<Beatmap, Error> {
        bail!("shiet");
    }
}

impl OszParser for HitObject {
    type Output = HitObject;
    fn parse() -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}

impl OszParser for TimingPoint {
    type Output = TimingPoint;
    fn parse() -> Result<Self::Output, Error> {
        bail!("shiet");
    }
}
