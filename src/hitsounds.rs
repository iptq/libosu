/// A set of hitsound samples.
///
/// Hitsounds come in sample sets of (normal, soft, drum). In beatmaps, there is a sample set that
/// apply to the entire beatmap as a whole, to timing sections specifically, to individual notes,
/// or even the hitsound additions (whistle, finish, clap).
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
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

#[allow(non_upper_case_globals)]
bitflags! {
    /// A representation of hitsound additions.
    pub struct Additions: u32 {
        /// Whistle hitsound
        const WHISTLE = 1 << 1;

        /// Finish (cymbal) hitsound
        const FINISH = 1 << 2;

        /// Clap hitsound
        const CLAP = 1 << 3;
    }
}

/// A hitsound "item" represents a single "hitsound".
#[derive(Clone, Debug)]
pub struct SampleInfo {
    /// The sample (normal/soft/drum) this hitsound uses.
    pub sample_set: SampleSet,
    /// The additions (whistle, finish, clap) attached to this hitsound.
    pub addition_set: SampleSet,

    /// TODO: additional field
    pub custom_index: i32,
    /// TODO: additional field
    pub sample_volume: i32,
    /// TODO: additional field
    pub filename: String,
}

impl Default for SampleInfo {
    fn default() -> SampleInfo {
        SampleInfo {
            sample_set: SampleSet::None,
            addition_set: SampleSet::None,
            custom_index: 0,
            sample_volume: 0,
            filename: "".to_owned(),
        }
    }
}
