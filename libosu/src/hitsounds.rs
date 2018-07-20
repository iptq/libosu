use std::ops::BitOr;

/// A set of hitsound samples.
///
/// Hitsounds come in sample sets of (normal, soft, drum). In beatmaps, there is a sample set that
/// apply to the entire beatmap as a whole, to timing sections specifically, to individual notes,
/// or even the hitsound additions (whistle, finish, clap).
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct Additions(u32);

impl BitOr for Additions {
    type Output = u32;
    fn bitor(self, other: Self) -> Self::Output {
        return self.0 | other.0;
    }
}
