use failure::Error;

mod beatmap;
mod hitobject;
mod timing;

pub use self::beatmap::*;
pub use self::hitobject::*;
pub use self::timing::*;

pub trait Deserializer<T> {
    type Output;
    fn deserialize(input: T) -> Result<Self::Output, Error>;
}

pub trait Serializer<T> {
    fn serialize(&self) -> Result<T, Error>;
}

type OsuFormat = String;
