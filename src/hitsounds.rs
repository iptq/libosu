use std::ops::BitOr;

use crate::TimeLocation;

/// A set of hitsound samples.
///
/// Hitsounds come in sample sets of (normal, soft, drum). In beatmaps, there is a sample set that
/// apply to the entire beatmap as a whole, to timing sections specifically, to individual notes,
/// or even the hitsound additions (whistle, finish, clap).
#[derive(Clone, Debug)]
pub enum SampleSet {
    /// No sample set used. (TODO: wtf?)
    None = 0,
    /// Normal sample set.
    Normal = 1,
    /// Soft sample set.
    Soft = 2,
    /// Drum sample set.
    Drum = 3,
}

/// A representation of hitsound additions.
#[derive(Clone, Debug)]
pub struct Additions(pub u32);

/// A hitsound "item" represents a single "hitsound".
#[derive(Clone, Debug)]
pub struct Hitsound {
    /// The time at which this hitsound occurs.
    pub time: TimeLocation,
    /// The sample (normal/soft/drum) this hitsound uses.
    pub sample: SampleSet,
    /// The additions (whistle, finish, clap) attached to this hitsound.
    pub additions: Additions,

    /// TODO: additional field
    pub sample_set: i32,
    /// TODO: additional field
    pub addition_set: i32,
    /// TODO: additional field
    pub custom_index: i32,
    /// TODO: additional field
    pub sample_volume: i32,
    /// TODO: additional field
    pub filename: String,
}

impl BitOr for Additions {
    type Output = u32;
    fn bitor(self, other: Self) -> Self::Output {
        self.0 | other.0
    }
}
