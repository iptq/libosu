use failure::Error;

mod beatmap;
mod hitobject;
mod osrparser;
mod timing;

pub use self::beatmap::*;
pub use self::hitobject::*;
pub use self::osrparser::*;
pub use self::timing::*;

pub trait OszParser<'src> {
    type Output;
    fn parse(input: &'src str) -> Result<Self::Output, Error>;
}
