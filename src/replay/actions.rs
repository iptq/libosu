use std::io::{BufReader, Read};

#[cfg(feature = "replay-data")]
use xz2::{bufread::XzDecoder, stream::Stream};

use super::{ReplayError, ReplayResult};

/// An action by the player while playing the map
#[derive(Debug, Clone)]
pub struct ReplayAction {
    /// The time since the last replay action in milliseconds.
    ///
    /// After osu! version `20130319` if this is the last action in the stream
    /// it may be set `-12345` indicating the `buttons` field holds the RNG seed for this score.
    pub time: i64,

    /// Cursor position X, in the range 0-512
    pub x: f32,

    /// Cursor position Y, in the range 0-384
    pub y: f32,

    /// bitwise combination of keys and mousebuttons pressed.
    pub buttons: Buttons,
}

bitflags! {
    /// The buttons being pressed during a frame of a replay
    pub struct Buttons: u32 {
        /// First mouse button
        const M1 = 1;

        /// Second mouse button
        const M2 = 2;

        /// First keyboard button
        const K1 = 4;

        /// Second keyboard button
        const K2 = 8;

        /// Smoke button
        const SMOKE = 16;
    }
}

// /// An iterator over actions as they are parsed
// ///
// /// use ReplayActionParser::iter() to create this struct
// pub struct ReplayActionParserIter<'a, R: BufRead> {
//     reader: &'a mut R,
//     number_buffer: Vec<u8>,
// }
//
// impl<'a, R: BufRead> ReplayActionParserIter<'a, R> {
//     fn new(reader: &'a mut R) -> ReplayActionParserIter<'a, R> {
//         ReplayActionParserIter {
//             reader,
//             number_buffer: Vec::new(),
//         }
//     }
//
//     fn read_time(&mut self) -> ReplayResult<Option<i64>> {
//         self.number_buffer.clear();
//         self.reader.read_until(b'|', &mut self.number_buffer)?;
//         if self.number_buffer.is_empty() {
//             return Ok(None);
//         }
//         let s = String::from_utf8_lossy(&self.number_buffer[..self.number_buffer.len() - 1]);
//         Ok(Some(i64::from_str(&s)?))
//     }
//
//     fn read_position(&mut self) -> ReplayResult<f32> {
//         self.number_buffer.clear();
//         self.reader.read_until(b'|', &mut self.number_buffer)?;
//         let s = String::from_utf8_lossy(&self.number_buffer[..self.number_buffer.len() - 1]);
//         Ok(f32::from_str(&s)?)
//     }
//
//     fn read_buttons(&mut self) -> ReplayResult<u32> {
//         self.number_buffer.clear();
//         self.reader.read_until(b',', &mut self.number_buffer)?;
//         if self.number_buffer.last() == Some(&b',') {
//             self.number_buffer.pop();
//         }
//
//         let s = String::from_utf8_lossy(&self.number_buffer);
//         Ok(u32::from_str(&s)?)
//     }
// }
//
// impl<'a, R: BufRead> Iterator for ReplayActionParserIter<'a, R> {
//     type Item = ReplayResult<ReplayAction>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let time = match self.read_time() {
//             Ok(Some(x)) => x,
//             Ok(None) => return None,
//             Err(e) => return Some(Err(e)),
//         };
//         let x = match self.read_position() {
//             Ok(x) => x,
//             Err(e) => return Some(Err(e)),
//         };
//         let y = match self.read_position() {
//             Ok(x) => x,
//             Err(e) => return Some(Err(e)),
//         };
//         let buttons = match self.read_buttons() {
//             Ok(x) => x,
//             Err(e) => return Some(Err(e)),
//         };
//
//         Some(Ok(ReplayAction {
//             time,
//             x,
//             y,
//             buttons,
//         }))
//     }
// }

/// A parser for decompressed replay actions
/// to read compressed replay actions see `create_decompressing_replay_action_parser`
#[derive(Debug, Clone)]
pub struct ReplayActionData {
    /// The frames of the replay
    pub frames: Vec<ReplayAction>,

    /// Only for replays from version 20130319 of later, this is the RNG seed used for the score
    pub rng_seed: Option<u32>,
}

impl ReplayActionData {
    #[cfg(feature = "replay-data")]
    /// create a new ReplayActionParser from a BufRead
    pub fn parse(data: impl Read) -> ReplayResult<Self> {
        let lzma_decoder = Stream::new_lzma_decoder(std::u64::MAX)?;
        let data_reader = BufReader::new(data);
        let mut xz_decoder = XzDecoder::new_stream(data_reader, lzma_decoder);

        let mut data = Vec::new();
        xz_decoder.read_to_end(&mut data)?;

        let string = String::from_utf8(data)?;
        let mut frames = string
            .split(',')
            .filter(|action_str| !action_str.trim().is_empty())
            .map(|action_str| {
                let mut parts = action_str.split('|');
                let time = parts.next().unwrap().parse::<i64>()?;
                let x = parts.next().unwrap().parse::<f32>()?;
                let y = parts.next().unwrap().parse::<f32>()?;
                let bits = parts.next().unwrap().parse::<u32>()?;

                let buttons = if time == -12345 {
                    // allow this
                    unsafe { Buttons::from_bits_unchecked(bits) }
                } else {
                    Buttons::from_bits(bits).ok_or(ReplayError::InvalidButtons(bits))?
                };
                Ok(ReplayAction {
                    time,
                    x,
                    y,
                    buttons,
                })
            })
            .collect::<ReplayResult<Vec<_>>>()?;

        let has_seed = matches!(frames.last(), Some(ReplayAction { time: -12345, .. }));
        let rng_seed = if has_seed {
            let last_element = frames.pop().expect("has_seed checked");
            Some(last_element.buttons.bits())
        } else {
            None
        };

        Ok(ReplayActionData { frames, rng_seed })
    }
}
