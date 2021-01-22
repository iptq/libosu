use std::fmt;
use std::str::FromStr;

use num::FromPrimitive;

use crate::errors::ParseError;

/// A set of hitsound samples.
///
/// Hitsounds come in sample sets of (normal, soft, drum). In beatmaps, there is a sample set that
/// apply to the entire beatmap as a whole, to timing sections specifically, to individual notes,
/// or even the hitsound additions (whistle, finish, clap).
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl FromStr for SampleInfo {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<SampleInfo, Self::Err> {
        let extra_parts = line.split(':').collect::<Vec<_>>();
        let sample_set = extra_parts[0].parse::<u32>()?;
        let addition_set = extra_parts[1].parse::<u32>()?;
        let custom_index = extra_parts[2].parse::<i32>()?;
        let sample_volume = extra_parts[3].parse::<i32>()?;
        let filename = extra_parts[4].to_owned();

        // TODO: handle extras field
        Ok(SampleInfo {
            addition_set: SampleSet::from_u32(addition_set).unwrap(),
            sample_set: SampleSet::from_u32(sample_set).unwrap(),

            custom_index,
            sample_volume,
            filename,
        })
    }
}

impl fmt::Display for SampleInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.sample_set as u32,
            self.addition_set as u32,
            self.custom_index,
            self.sample_volume,
            self.filename
        )
    }
}
