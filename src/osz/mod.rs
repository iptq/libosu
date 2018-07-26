use failure::Error;

mod beatmap;
mod hitobject;
mod timing;

pub use self::beatmap::*;
pub use self::hitobject::*;
pub use self::timing::*;

/// The struct for which this is implement is able to be deserialized from a T.
/// (Basically a marker trait at this point)
pub trait OszDeserializer<T> {
    type Output;
    fn deserialize_osz(input: T) -> Result<Self::Output, Error>;
}

/// The struct for which this is implement is able to be serialized from a T.
/// (Basically a marker trait at this point)
pub trait OszSerializer<T> {
    fn serialize_osz(&self) -> Result<T, Error>;
}

type OsuFormat = String;
