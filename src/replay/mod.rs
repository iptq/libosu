//! Data structures for reading and writing replays
//!
//! The main focus of this module is the [`Replay`][self::Replay] struct, which contains functions
//! for opening and writing *.osr files.
//!
//! Read more about the *.osr data format on the [OSU wiki][1].
//!
//! Simple Example
//! --------------
//!
//! ```no_run
//! # use std::{fs::File, io::Read};
//! # use libosu::replay::Replay;
//! # #[cfg(feature = "replay-data")]
//! # fn invisible(mut reader: impl Read) -> anyhow::Result<()> {
//! #
//! let replay = Replay::parse(&mut reader)?;
//! let action_data = replay.parse_action_data()?;
//! for frame in action_data.frames.iter() {
//!     println!("time={} x={} y={} btns={:?}", frame.time, frame.x, frame.y, frame.buttons);
//! }
//! println!("seed: {:?}", action_data.rng_seed);
//!
//! # Ok(())
//! # }
//! ```
//!
//! Note that the frames of the actual replay (called `action_data` in libosu) is stored in
//! compressed form at all times, so in order to actually change the action data, you will want to
//! call [`Replay::update_action_data`][Replay::update_action_data] in order to write the changed
//! action data _back_ into the replay:
//!
//! ```no_run
//! # use libosu::replay::{Replay, ReplayActionData};
//! # #[cfg(feature = "replay-data")]
//! # fn invisible(mut action_data: ReplayActionData, mut replay: Replay) -> anyhow::Result<()> {
//! #
//! // assuming these were declared as mut instead
//! action_data.frames[0].x = 5.0;
//! replay.update_action_data(&action_data);
//!
//! # Ok(())
//! # }
//! ```
//!
//! Then you can write this back into a file:
//!
//! ```no_run
//! # use std::fs::File;
//! # use libosu::replay::Replay;
//! # fn invisible(replay: Replay) -> anyhow::Result<()> {
//! #
//! let mut output = File::create("output.osr")?;
//! replay.write(&mut output)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! [1]: https://osu.ppy.sh/wiki/en/osu%21_File_Formats/Osr_%28file_format%29

mod actions;

use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::db::{ReadBytesOsu, WriteBytesOsu};
use crate::data::{Mode, Mods};

pub use self::actions::{Buttons, ReplayAction, ReplayActionData};

/// Result type for Replay processing
pub type ReplayResult<T, E = ReplayError> = std::result::Result<T, E>;

/// Errors that could occur while processing replays
// TODO: track positions as well
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ReplayError {
    #[cfg(feature = "replay-data")]
    #[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
    #[error("error creating lzma decoder: {0}")]
    LzmaCreate(#[from] xz2::stream::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("error decoding utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("error parsing int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("error parsing float: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("binary data error: {0}")]
    Binary(#[from] crate::db::binary::Error),

    #[error("missing field in life graph")]
    LifeGraphMissing,

    #[error("unexpected mods: {0}")]
    UnexpectedMods(u32),

    #[error("invalid mode: {0}")]
    InvalidMode(u8),

    #[error("invalid buttons: {0}")]
    InvalidButtons(u32),
}

/// A replay object.
///
/// See the [module documentation][crate::replay] for examples of using this struct.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Replay {
    /// osu! game mode that this replay was recorded for
    ///
    /// Note the mode field may change the meaning of some of the `count_*` fields
    /// becuase some osu! modes score slightly differently from standard
    pub mode: Mode,

    /// osu! game version this replay was recorded on
    pub version: u32,

    /// MD5 hash for the beatmap this replay's map
    pub beatmap_hash: String,

    /// The username of player
    pub player_username: String,

    /// MD5 hash for this replay
    pub replay_hash: String,

    /// The number of 300s the player scored in this map
    pub count_300: u16,

    /// The number of 100s the player scored, or 150s in taiko
    pub count_100: u16,

    /// The number of 50s the player scored, or small fruit in Catch the Beat
    pub count_50: u16,

    /// The number of gekis the player scored in standard, or max 300s in Mania
    pub count_geki: u16,

    /// The number of katus the player scored in standard, or 200s in Mania
    pub count_katu: u16,

    /// The number of misses
    pub count_miss: u16,

    /// total score as displayed on the score report
    pub score: u32,

    /// max combo as displayed on the score report
    pub max_combo: u16,

    /// true if the player has no _misses_, _slider breaks_, or _early finished sliders_.
    pub perfect: bool,

    /// mod values mods or'd together
    pub mods: Mods,

    /// List of integer timestamps with an associated player life value.
    ///
    /// 0 = No health, 1 = Full health
    pub life_graph: Vec<(i32, f64)>,

    /// Timestamp of the replay in measured in 1/10ths of a millisecond (100 ns)
    ///
    /// This is value is measured in [windows ticks][1]
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    ///
    /// [1]: https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks
    pub timestamp: u64,

    /// The action data contained in this replay.
    ///
    /// If the feature `replay-data` is enabled, the function `parse_action_data` can be used to
    /// decompress and parse the data that is contained in this replay.
    pub action_data: Vec<u8>,

    /// Online ID of this score, if submitted.
    pub score_id: Option<u64>,

    /// Only present if `self.mods` includes Target Practice (bit 23)
    /// Total accuracy of all hits.
    /// Divide this value by the number of targets to find the actual accuracy
    pub target_practice_total_accuracy: Option<f64>,
}

impl Replay {
    /// Parse the replay header from a replay file.
    ///
    /// The returned struct will be missing the `action`, `score_id`, and `target_practice_total_accuracy` fields.
    /// If you need `score_id` or `target_practice_total_accuracy`, but don't want to parse actions use the `parse_skip_actions` function instead.
    pub fn parse<R: Read>(reader: &mut R) -> ReplayResult<Replay> {
        let mode = match reader.read_u8()? {
            0 => Mode::Osu,
            1 => Mode::Taiko,
            2 => Mode::Catch,
            3 => Mode::Mania,
            x => return Err(ReplayError::InvalidMode(x)),
        };
        let version = reader.read_u32::<LittleEndian>()?;
        let beatmap_hash = reader.read_uleb128_string()?;
        let player_username = reader.read_uleb128_string()?;
        let replay_hash = reader.read_uleb128_string()?;
        let count_300 = reader.read_u16::<LittleEndian>()?;
        let count_100 = reader.read_u16::<LittleEndian>()?;
        let count_50 = reader.read_u16::<LittleEndian>()?;
        let count_geki = reader.read_u16::<LittleEndian>()?;
        let count_katu = reader.read_u16::<LittleEndian>()?;
        let count_miss = reader.read_u16::<LittleEndian>()?;
        let score = reader.read_u32::<LittleEndian>()?;
        let max_combo = reader.read_u16::<LittleEndian>()?;
        let perfect = reader.read_u8()? == 1;
        let mods_value = reader.read_u32::<LittleEndian>()?;
        let mods = Mods::from_bits(mods_value).ok_or(ReplayError::UnexpectedMods(mods_value))?;
        let life_graph = reader
            .read_uleb128_string()?
            .split(',')
            .filter_map(|frame| {
                if frame.is_empty() {
                    None
                } else {
                    Some(frame.split('|'))
                }
            })
            .map(|mut frame| {
                Ok((
                    frame
                        .next()
                        .ok_or(ReplayError::LifeGraphMissing)?
                        .parse::<i32>()?,
                    frame
                        .next()
                        .ok_or(ReplayError::LifeGraphMissing)?
                        .parse::<f64>()?,
                ))
            })
            .collect::<ReplayResult<Vec<_>>>()?;
        let timestamp = reader.read_u64::<LittleEndian>()?;
        let replay_data_length = reader.read_u32::<LittleEndian>()?;

        let mut action_data = vec![0; replay_data_length as usize];
        reader.read_exact(&mut action_data)?;

        let score_id = match reader.read_u64::<LittleEndian>()? {
            0 => None,
            v => Some(v),
        };
        let target_practice_total_accuracy = if mods.contains(Mods::TargetPractice) {
            Some(reader.read_f64::<LittleEndian>()?)
        } else {
            None
        };

        Ok(Replay {
            mode,
            version,
            beatmap_hash,
            player_username,
            replay_hash,
            count_300,
            count_100,
            count_50,
            count_geki,
            count_katu,
            count_miss,
            score,
            max_combo,
            perfect,
            mods,

            life_graph,
            timestamp,

            action_data,
            score_id,
            target_practice_total_accuracy,
        })
    }

    /// Writes this replay to the given writer
    pub fn write<W: Write>(&self, mut w: W) -> ReplayResult<()> {
        w.write_u8(self.mode as u8)?;
        w.write_u32::<LittleEndian>(self.version)?;
        w.write_uleb128_string(&self.beatmap_hash)?;
        w.write_uleb128_string(&self.player_username)?;
        w.write_uleb128_string(&self.replay_hash)?;
        w.write_u16::<LittleEndian>(self.count_300)?;
        w.write_u16::<LittleEndian>(self.count_100)?;
        w.write_u16::<LittleEndian>(self.count_50)?;
        w.write_u16::<LittleEndian>(self.count_geki)?;
        w.write_u16::<LittleEndian>(self.count_katu)?;
        w.write_u16::<LittleEndian>(self.count_miss)?;
        w.write_u32::<LittleEndian>(self.score)?;
        w.write_u16::<LittleEndian>(self.max_combo)?;
        w.write_u8(if self.perfect { 0 } else { 1 })?;
        w.write_u32::<LittleEndian>(self.mods.bits())?;
        w.write_uleb128_string(
            &self
                .life_graph
                .iter()
                .map(|(time, life)| format!("{}|{}", time, life))
                .collect::<Vec<_>>()
                .join(","),
        )?;
        w.write_u64::<LittleEndian>(self.timestamp)?;
        w.write_u32::<LittleEndian>(self.action_data.len() as u32)?;
        w.write_all(&self.action_data)?;
        w.write_u64::<LittleEndian>(self.score_id.unwrap_or(0))?;
        if let Some(acc) = self.target_practice_total_accuracy {
            w.write_f64::<LittleEndian>(acc)?;
        }
        Ok(())
    }

    #[cfg(feature = "replay-data")]
    #[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
    /// Updates the Replay object with action data
    pub fn update_action_data(&mut self, action_data: &ReplayActionData) -> ReplayResult<()> {
        use xz2::{
            stream::{LzmaOptions, Stream},
            write::XzEncoder,
        };

        // clear everything in the data first
        // NOTE: this doesn't change the capacity
        self.action_data.clear();

        // write thru the encoder
        // TODO: presets? options?
        let opts = LzmaOptions::new_preset(0)?;
        let stream = Stream::new_lzma_encoder(&opts)?;
        {
            let mut xz = XzEncoder::new_stream(&mut self.action_data, stream);
            for (i, frame) in action_data.frames.iter().enumerate() {
                if i > 0 {
                    xz.write_all(&[b','])?;
                }

                let this_frame = format!(
                    "{}|{}|{}|{}",
                    frame.time.0,
                    frame.x,
                    frame.y,
                    frame.buttons.bits()
                );
                xz.write_all(this_frame.as_bytes())?;
            }

            if let Some(seed) = action_data.rng_seed {
                let this_frame = format!(",-12345|0|0|{}", seed);
                xz.write_all(this_frame.as_bytes())?;
            }
        }

        Ok(())
    }

    #[cfg(feature = "replay-data")]
    #[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
    /// Parse and retrieve the actions in the replay
    pub fn parse_action_data(&self) -> ReplayResult<ReplayActionData> {
        use std::io::Cursor;
        let cursor = Cursor::new(&self.action_data);
        ReplayActionData::parse(cursor)
    }
}
