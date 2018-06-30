use failure::Error;

mod beatmap;
mod hitobject;
mod timing;

pub use self::beatmap::*;
pub use self::hitobject::*;
pub use self::timing::*;

/// The struct for which this is implement is able to be deserialized from a T.
/// (Basically a marker trait at this point)
pub trait Deserializer<T> {
    type Output;
    fn deserialize(input: T) -> Result<Self::Output, Error>;
}

/// The struct for which this is implement is able to be serialized from a T.
/// (Basically a marker trait at this point)
pub trait Serializer<T> {
    fn serialize(&self) -> Result<T, Error>;
}

type OsuFormat = String;
