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
    Default = 0,

    /// Normal sample set.
    Normal = 1,

    /// Soft sample set.
    Soft = 2,

    /// Drum sample set.
    Drum = 3,
}

bitflags! {
    /// A representation of hitsound additions.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[allow(non_upper_case_globals)]
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

    /// The index of the sample filename to use
    pub custom_index: i32,

    /// Volume (from 5 to 100)
    pub sample_volume: i32,

    /// TODO: additional field
    /// (does this even have any effect lol)
    pub filename: String,
}

impl Default for SampleInfo {
    fn default() -> SampleInfo {
        SampleInfo {
            sample_set: SampleSet::Default,
            addition_set: SampleSet::Default,
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
        let custom_index = match extra_parts.get(2) {
            Some(v) => v.parse::<i32>()?,
            None => 0,
        };
        let sample_volume = match extra_parts.get(3) {
            Some(v) => v.parse::<i32>()?,
            None => 0,
        };
        let filename = match extra_parts.get(4) {
            Some(v) => (*v).to_owned(),
            None => String::new(),
        };

        Ok(SampleInfo {
            addition_set: SampleSet::from_u32(addition_set)
                .ok_or(ParseError::InvalidSampleSet(addition_set))?,
            sample_set: SampleSet::from_u32(sample_set)
                .ok_or(ParseError::InvalidSampleSet(sample_set))?,

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
