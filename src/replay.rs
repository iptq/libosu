use std::io::{self};

use anyhow::Result;
use std::str::FromStr;
use xz2::bufread::XzDecoder;
use xz2::stream::Stream;

use crate::{
    Mode,
    read_f64le, read_u16le, read_u32le, read_u64le, read_u8, read_uleb128_string,
};

// write a parser for the life graph
// /// A point in the life graph
// #[derive(Debug, Clone)]
// pub struct LifeGraphPoint {
//     /// The number of milliseconds into the song where the player had this life total, in milliseconds
//     pub time: u32,

//     /// life total, from 0-1
//     pub life: f64,
// }

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
    ///
    /// M1 = 1
    /// M2 = 2
    /// K1 = 4
    /// K2 = 8
    /// Smoke = 16
    /// TODO: what are the keys for other modes?
    pub buttons: u32,
}

/// A replay object.
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
    pub mods: u32,

    /// List of times with an associated player life value
    ///
    /// values are sored in a comma seperated list of "time/life",
    ///   where time is the timestamp in ms and life is a value from 0-1
    /// TODO: write a parser for this
    pub life_graph: String,

    /// Timestamp of the replay in measured in 1/10ths of a millisecond (100 ns)
    ///
    /// This is value is measured in windows ticks (https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks)
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    pub timestamp: u64,

    /// length of the compressed list of replay actions
    pub replay_data_length: u32,

    /// A series of actions made by the player.
    ///
    /// Note that the order is important becuase each replay action only stores the time since the last
    pub actions: Vec<ReplayAction>,

    /// Online ID of this score, if the score was never submitted this value is `0`
    pub score_id: u64,

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
    pub fn parse_header<R: io::Read>(reader: &mut R) -> Result<Replay, Error> {
        Ok(Replay {
            mode: match read_u8(reader)? {
                0 => Mode::Osu,
                1 => Mode::Taiko,
                2 => Mode::Catch,
                3 => Mode::Mania,
                x => bail!(
                    "Unknown game mode ID in replay file: {}, expecting 0, 1, 2, or 3.",
                    x
                ),
            },
            version: read_u32le(reader)?,
            beatmap_hash: read_uleb128_string(reader)?,
            player_username: read_uleb128_string(reader)?,
            replay_hash: read_uleb128_string(reader)?,
            count_300: read_u16le(reader)?,
            count_100: read_u16le(reader)?,
            count_50: read_u16le(reader)?,
            count_geki: read_u16le(reader)?,
            count_katu: read_u16le(reader)?,
            count_miss: read_u16le(reader)?,
            score: read_u32le(reader)?,
            max_combo: read_u16le(reader)?,
            perfect: read_u8(reader)? == 1,
            mods: read_u32le(reader)?,
            life_graph: read_uleb128_string(reader)?,
            timestamp: read_u64le(reader)?,
            replay_data_length: read_u32le(reader)?,

            actions: Vec::new(),
            score_id: 0,
            target_practice_total_accuracy: None,
        })
    }

    fn parse_tail<R: io::Read>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.score_id = read_u64le(reader)?;
        self.target_practice_total_accuracy = if (self.mods & 8388608) != 0 {
            Some(read_f64le(reader)?)
        } else {
            None
        };
        Ok(())
    }

    fn create_action_parser<R: io::BufRead>(
        &self,
        reader: R,
    ) -> Result<ReplayActionParser<io::BufReader<XzDecoder<io::Take<R>>>>, Error> {
        create_decompressing_replay_action_parser(reader.take(self.replay_data_length as u64))
    }

    /// Parse a replay file (.osr)
    pub fn parse<R: io::BufRead>(mut reader: R) -> Result<Replay, Error> {
        let mut replay = Replay::parse_header(&mut reader)?;
        let mut action_parser = replay.create_action_parser(reader)?;
        for action in action_parser.iter() {
            replay.actions.push(action?);
        }

        let mut reader = action_parser
            .into_inner() //  ReplayActionParser -> BufReader
            .into_inner() //  BufReader -> XzDecoder
            .into_inner() //  XzDecoder -> Take
            .into_inner(); // Take -> reader
        replay.parse_tail(&mut reader)?;
        Ok(replay)
    }

    /// Parse a replay file, but skip the player actions
    ///
    /// Using this function can be signficantly easier on memory and CPU
    /// if only score information is needed
    pub fn parse_skip_actions<R: io::Read + io::Seek>(mut reader: R) -> Result<Replay, Error> {
        let mut replay = Replay::parse_header(&mut reader)?;
        reader.seek(io::SeekFrom::Current(replay.replay_data_length as i64))?;
        replay.parse_tail(&mut reader)?;
        Ok(replay)
    }
}

/// An iterator over actions as they are parsed
///
/// use ReplayActionParser::iter() to create this struct
pub struct ReplayActionParserIter<'a, R: io::BufRead> {
    reader: &'a mut R,
    number_buffer: Vec<u8>,
}

impl<'a, R: io::BufRead> ReplayActionParserIter<'a, R> {
    fn new(reader: &'a mut R) -> ReplayActionParserIter<'a, R> {
        ReplayActionParserIter {
            reader,
            number_buffer: Vec::new(),
        }
    }

    fn read_time(&mut self) -> Result<Option<i64>, Error> {
        self.number_buffer.clear();
        self.reader.read_until('|' as u8, &mut self.number_buffer)?;
        if self.number_buffer.len() == 0 {
            return Ok(None);
        }
        let s = String::from_utf8_lossy(&self.number_buffer[..self.number_buffer.len() - 1]);
        Ok(Some(i64::from_str(&s)?))
    }

    fn read_position(&mut self) -> Result<f32, Error> {
        self.number_buffer.clear();
        self.reader.read_until('|' as u8, &mut self.number_buffer)?;
        let s = String::from_utf8_lossy(&self.number_buffer[..self.number_buffer.len() - 1]);
        Ok(f32::from_str(&s)?)
    }

    fn read_buttons(&mut self) -> Result<u32, Error> {
        self.number_buffer.clear();
        self.reader.read_until(',' as u8, &mut self.number_buffer)?;
        if self.number_buffer.last() == Some(&(',' as u8)) {
            self.number_buffer.pop();
        }

        let s = String::from_utf8_lossy(&self.number_buffer);
        Ok(u32::from_str(&s)?)
    }
}

impl<'a, R: io::BufRead> Iterator for ReplayActionParserIter<'a, R> {
    type Item = Result<ReplayAction, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let time = match self.read_time() {
            Ok(Some(x)) => x,
            Ok(None) => return None,
            Err(e) => return Some(Err(e)),
        };
        let x = match self.read_position() {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        let y = match self.read_position() {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        let buttons = match self.read_buttons() {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(ReplayAction {
            time,
            x,
            y,
            buttons,
        }))
    }
}

/// A parser for decompressed replay actions
/// to read compressed replay actions see `create_decompressing_replay_action_parser`
pub struct ReplayActionParser<R: io::BufRead> {
    inner: R,
}

impl<R: io::BufRead> ReplayActionParser<R> {
    /// create a new ReplayActionParser from a BufRead
    pub fn new(reader: R) -> ReplayActionParser<R> {
        ReplayActionParser { inner: reader }
    }

    /// create an iterator over the parsed replay actions
    pub fn iter<'a>(&'a mut self) -> ReplayActionParserIter<'a, R> {
        ReplayActionParserIter::new(&mut self.inner)
    }

    /// unwrap the inner reader
    pub fn into_inner(self) -> R {
        self.inner
    }
}

/// Create a parser for LZMA compressed replay actions
pub fn create_decompressing_replay_action_parser<R: io::BufRead>(
    reader: R,
) -> Result<ReplayActionParser<io::BufReader<XzDecoder<R>>>, Error> {
    Ok(ReplayActionParser::new(io::BufReader::new(
        XzDecoder::new_stream(reader, Stream::new_lzma_decoder(std::u64::MAX)?),
    )))
}

#[test]
fn test_read_uleb128() {
    let mut num = io::Cursor::new([0xE5, 0x8E, 0x26]);
    assert_eq!(read_uleb128(&mut num).unwrap(), 624485u64);
}

#[test]
fn test_read_uleb128_string() {
    let text = "Hello World";
    let mut replay_string = vec![0x0Bu8];
    replay_string.push(text.len() as u8);
    replay_string.extend(text.bytes());

    let mut reader = io::Cursor::new(replay_string);
    assert_eq!(read_uleb128_string(&mut reader).unwrap(), text.to_string());

    let mut reader_empty = io::Cursor::new(vec![0x00]);
    assert_eq!(
        read_uleb128_string(&mut reader_empty).unwrap(),
        String::new()
    );

    let mut reader_zero_length = io::Cursor::new(vec![0x0B, 0x00]);
    assert_eq!(
        read_uleb128_string(&mut reader_zero_length).unwrap(),
        String::new()
    );
}

#[test]
fn test_replay_parse_header() {
    use super::Mods;
    use std::fs::File;

    let mut osr = File::open("tests/files/replay-osu_2058788_3017707256.osr").unwrap();
    let header = Replay::parse_header(&mut osr).unwrap();

    assert_eq!(header.mode, Mode::Osu);
    assert_eq!(header.version, 20200304);
    assert_eq!(
        header.beatmap_hash,
        "4190b795c2847f9eae06a0651493d6e2".to_string()
    );
    assert_eq!(header.player_username, "FGSky".to_string());
    assert_eq!(
        header.replay_hash,
        "e8983dbdb53360e5d19cbe5de5de49a7".to_string()
    );

    assert_eq!(header.count_300, 330);
    assert_eq!(header.count_100, 24);
    assert_eq!(header.count_50, 0);
    assert_eq!(header.count_geki, 87);
    assert_eq!(header.count_katu, 21);
    assert_eq!(header.count_miss, 2);

    assert_eq!(header.score, 7756117);
    assert_eq!(header.max_combo, 527);
    assert_eq!(header.perfect, false);
    assert_eq!(
        header.mods,
        (Mods::Flashlight | Mods::Hidden) | (Mods::DoubleTime | Mods::HardRock)
    );
}

#[test]
fn test_read_replay_actions() {
    use std::fs::File;
    use std::io::Read;

    let mut osr =
        io::BufReader::new(File::open("tests/files/replay-osu_2058788_3017707256.osr").unwrap());
    let replay = Replay::parse_header(&mut osr).unwrap();

    let actions: Vec<_> =
        create_decompressing_replay_action_parser(osr.take(replay.replay_data_length as u64))
            .unwrap()
            .iter()
            .map(|x| match x {
                Ok(v) => v,
                Err(e) => panic!("read replay action error: {}", e),
            })
            .collect();

    let seed_action = actions.last().unwrap();
    assert_eq!(seed_action.time, -12345);
    assert_eq!(seed_action.x, 0.0);
    assert_eq!(seed_action.y, 0.0);
    assert_eq!(seed_action.buttons, 16516643);
}

#[test]
fn test_replay_parse_skip_actions() {
    use std::fs::File;
    {
        let mut osr =
            File::open("tests/files/ - nekodex - new beginnings [tutorial] (2020-12-16) Osu.osr")
                .unwrap();
        let replay = Replay::parse_skip_actions(&mut osr).unwrap();
        assert_eq!(replay.score_id, 0);
        assert_eq!(replay.target_practice_total_accuracy, None);
    }

    {
        let mut osr = File::open("tests/files/replay-osu_2058788_3017707256.osr").unwrap();
        let replay = Replay::parse_skip_actions(&mut osr).unwrap();
        assert_eq!(replay.score_id, 3017707256);
        assert_eq!(replay.target_practice_total_accuracy, None);
    }
}

#[test]
fn test_replay_action_parser() {
    let actions_text = "1|32.1|300.734|0,32|500.5123|0|10,-12345|0|0|734243";

    let reader = io::Cursor::new(actions_text);
    let actions: Vec<_> = ReplayActionParser::new(reader)
        .iter()
        .map(|x| x.unwrap())
        .collect();

    assert_eq!(actions.len(), 3);

    assert_eq!(actions[0].time, 1);
    assert_eq!(actions[0].x, 32.1);
    assert_eq!(actions[0].y, 300.734);
    assert_eq!(actions[0].buttons, 0);

    assert_eq!(actions[1].time, 32);
    assert_eq!(actions[1].x, 500.5123);
    assert_eq!(actions[1].y, 0.0);
    assert_eq!(actions[1].buttons, 10);

    assert_eq!(actions[2].time, -12345);
    assert_eq!(actions[2].x, 0.0);
    assert_eq!(actions[2].y, 0.0);
    assert_eq!(actions[2].buttons, 734243);
}

#[test]
fn test_replay_parse() {
    use std::fs::File;

    let osr = File::open("tests/files/replay-osu_1816113_2892542031.osr").unwrap();
    let replay = Replay::parse(io::BufReader::new(osr)).unwrap();
    dbg!(replay.actions.len());
    assert_eq!(replay.mode, Mode::Osu);
    assert_eq!(replay.version, 20190906);
    assert_eq!(
        replay.beatmap_hash,
        "edd35ab673c5f73029cc8eda6faefe00".to_string()
    );
    assert_eq!(replay.player_username, "Vaxei".to_string());
    assert_eq!(
        replay.replay_hash,
        "139c99f18fc78555cd8f30a963aadf0a".to_string()
    );

    assert_eq!(replay.count_300, 2977);
    assert_eq!(replay.count_100, 38);
    assert_eq!(replay.count_50, 0);
    assert_eq!(replay.count_geki, 605);
    assert_eq!(replay.count_katu, 30);
    assert_eq!(replay.count_miss, 0);

    assert_eq!(replay.score, 364_865_850);
    assert_eq!(replay.max_combo, 4078);
    assert_eq!(replay.perfect, false);
    assert_eq!(replay.mods, 0);

    let seed_action = replay.actions.last().unwrap();
    assert_eq!(seed_action.time, -12345);
    assert_eq!(seed_action.x, 0.0);
    assert_eq!(seed_action.y, 0.0);
    assert_eq!(seed_action.buttons, 7364804);

    assert_eq!(replay.score_id, 2892542031);
    assert_eq!(replay.target_practice_total_accuracy, None);
}
