use std::io;

use failure::Error;

use crate::{Grade, Mode, RankedStatus,  UserPermission, read_replay_string, replay::{read_f32le, read_f64le, read_u16le, read_u32le, read_u64le, read_u8}};

#[derive(Debug)]
/// Timing point for beatmap in osu!.db
pub struct OsuDBBeatmapTimingPoint {
    bpm: f64,
    offset: f64,
    is_uninherited: bool,
}

#[derive(Debug)]
/// Beatmap located in osu!.db, they are different from normal beatmaps
pub struct OsuDBBeatmap {
    /// Size in bytes of the beatmap entry. Only present if version is less than 20191106.
    pub size: Option<u32>,

    /// Artist's name. (ASCII)
    pub artist_name: String,
    /// Artist's name. (UTF-8)
    pub artist_name_unicode: String,

    /// Song title. (ASCII)
    pub song_title: String,
    /// Song title. (UTF-8)
    pub song_title_unicode: String,

    /// Creator name.
    pub creator_name: String,
    /// Difficulty name.
    pub difficulty: String,

    /// Name of the audio file.
    pub audio_file_name: String,
    /// MD5 hash of the beatmap.
    pub hash: String,
    /// Name of the .osu file.
    pub beatmap_file_name: String,
    /// Ranked status.
    pub ranked_status: RankedStatus,

    /// How many hitcircles in the beatmap.
    pub hitcircle_count: u16,
    /// How many sliders in the beatmap.
    pub slider_count: u16,
    /// How many spinners in the beatmap.
    pub spinner_count: u16,

    /// When the beatmap was last modified.
    ///
    /// This is value is measured in windows ticks (https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks)
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    pub modification_date: u64,

    /// AR rating.
    pub approach_rate: f32,
    /// CS rating.
    pub circle_size: f32,
    /// HP rating.
    pub hp_drain: f32,
    /// OD rating.
    pub overall_difficulty: f32,

    /// Slider velocity setting of the beatmap.
    pub slider_velocity: f64,
    /// A list of calculated star ratings for different mods for standard. Empty if version less than 20140609.
    pub std_star_rating: Vec<(u32, f64)>,
    /// A list of calculated star ratings for different mods for taiko. Empty if version less than 20140609.
    pub std_taiko_rating: Vec<(u32, f64)>,
    /// A list of calculated star ratings for different mods for ctb. Empty if version less than 20140609.
    pub std_ctb_rating: Vec<(u32, f64)>,
    /// A list of calculated star ratings for different mods for mania. Empty if version less than 20140609.
    pub std_mania_rating: Vec<(u32, f64)>,

    /// The drain time in seconds.
    pub drain_time: u32,
    /// The total time in milliseconds.
    pub total_time: u32,
    /// The preview time point in milliseconds.
    pub preview_time: u32,

    /// Timing points for the beatmap.
    pub timing_points: Vec<OsuDBBeatmapTimingPoint>,

    /// Id of the beatmap.
    pub beatmap_id: u32,
    /// Id of the beatmap set.
    pub beatmap_set_id: u32,
    /// Id of the beatmap thread.
    pub thread_id: u32,

    /// The grade achieved in standard.
    pub std_grade: Grade,
    /// The grade achieved in taiko.
    pub taiko_grade: Grade,
    /// The grade achieved in ctb.
    pub ctb_grade: Grade,
    /// The grade achieved in mania.
    pub mania_grade: Grade,

    /// Local offset.
    pub beatmap_offset: u16,

    /// Stack leniency.
    pub stack_leniency: f32,

    /// Mode of the beatmap.
    pub mode: Mode,

    /// Source.
    pub source: String,
    /// Tags.
    pub tags: String,

    /// Online offset.
    pub online_offset: u16,

    /// Title font.
    pub title_font: String,

    /// Is the beatmap not played.
    pub is_unplayed: bool,

    /// Last time the map was played.
    ///
    /// This is value is measured in windows ticks (https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks)
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    pub last_played: u64,

    /// If the format is osz2.
    pub is_osz2: bool,

    /// Folder name relative to the Songs folder.
    pub folder_name: String,

    /// Last time beatmap was checked to the online repository.
    ///
    /// This is value is measured in windows ticks (https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks)
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    pub last_checked: u64,

    /// Ignore beatmap sounds.
    pub ignore_beatmap_sounds: bool,
    /// Ignore beatmap skin.
    pub ignore_beatmap_skin: bool,
    /// Disable storyboard.
    pub disable_storyboard: bool,
    /// Disable video.
    pub disable_video: bool,
    /// Visual override.
    pub visual_override: bool,

    /// Unknown. Only present if version is less than 20140609.
    pub unknown: Option<u16>,

    /// Last modification time (?).
    pub unknown_modification_date: u32,

    /// Scroll speed for mania.
    pub mania_scrollspeed: u8,
}

#[derive(Debug)]
/// osu!.db object  
pub struct OsuDB {
    /// osu! game mode that this replay was recorded for
    pub version: u32,
    /// The amount of folders?
    pub folder_count: u32,
    /// If the account is unlocked, aka not banned or locked.
    pub account_unlocked: bool,
    /// When the account will be unlocked
    ///
    /// This is value is measured in windows ticks (https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks)
    /// It counts the number of ticks from 12:00:00 midnight, January 1, 0001 to the time this replay was created
    pub unlocked_date: u64,
    /// The player's username
    pub player_name: String,
    /// The amount of beatmaps cached
    pub beatmap_count: u32,
    /// The cached beatmaps     
    pub beatmaps: Vec<OsuDBBeatmap>,
    /// The permissions of the user
    pub permissions: UserPermission,
}

impl OsuDBBeatmap {
    fn read_star_rating(mut reader: impl io::BufRead) -> Result<Vec<(u32, f64)>, Error> {
        let count = read_u32le(&mut reader)?;
        let ratings = (0..count)
            .map(|_| -> Result<(u32, f64), Error> {
                Ok((
                    {
                        assert_eq!(read_u8(&mut reader)?, 0x08);
                        read_u32le(&mut reader)?
                    },
                    {
                        assert_eq!(read_u8(&mut reader)?, 0x0D);
                        read_f64le(&mut reader)?
                    }
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(ratings)
    }

    fn read_timing_points(mut reader: impl io::BufRead) -> Result<Vec<OsuDBBeatmapTimingPoint>, Error> {
        let count = read_u32le(&mut reader)?;
        let points = (0..count)
            .map(|_| {
                Ok(OsuDBBeatmapTimingPoint {
                    bpm: read_f64le(&mut reader)?,
                    offset: read_f64le(&mut reader)?,
                    is_uninherited: read_u8(&mut reader)? > 0,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(points)
    }


    fn parse(mut reader: impl io::BufRead, version: u32) -> Result<OsuDBBeatmap, Error> {
        Ok(OsuDBBeatmap {
            size: if version < 20191106 {
                Some(read_u32le(&mut reader)?)
            } else {
                None
            },
            artist_name: read_replay_string(&mut reader)?,
            artist_name_unicode: read_replay_string(&mut reader)?,
            song_title: read_replay_string(&mut reader)?,
            song_title_unicode: read_replay_string(&mut reader)?,
            creator_name: read_replay_string(&mut reader)?,
            difficulty: read_replay_string(&mut reader)?,
            audio_file_name: read_replay_string(&mut reader)?,
            hash: read_replay_string(&mut reader)?,
            beatmap_file_name: read_replay_string(&mut reader)?,
            ranked_status: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            hitcircle_count: read_u16le(&mut reader)?,
            slider_count: read_u16le(&mut reader)?,
            spinner_count: read_u16le(&mut reader)?,
            modification_date: read_u64le(&mut reader)?,
            approach_rate: read_f32le(&mut reader)?,
            circle_size: read_f32le(&mut reader)?,
            hp_drain: read_f32le(&mut reader)?,
            overall_difficulty: read_f32le(&mut reader)?,
            slider_velocity: read_f64le(&mut reader)?,
            std_star_rating: Self::read_star_rating(&mut reader)?,
            std_taiko_rating: Self::read_star_rating(&mut reader)?,
            std_ctb_rating: Self::read_star_rating(&mut reader)?,
            std_mania_rating: Self::read_star_rating(&mut reader)?,
            drain_time: read_u32le(&mut reader)?,
            total_time: read_u32le(&mut reader)?,
            preview_time: read_u32le(&mut reader)?,
            timing_points: Self::read_timing_points(&mut reader)?,
            beatmap_id: read_u32le(&mut reader)?,
            beatmap_set_id: read_u32le(&mut reader)?,
            thread_id: read_u32le(&mut reader)?,
            std_grade: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            taiko_grade: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            ctb_grade: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            mania_grade: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            beatmap_offset: read_u16le(&mut reader)?,
            stack_leniency: read_f32le(&mut reader)?,
            mode: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
            source: read_replay_string(&mut reader)?,
            tags: read_replay_string(&mut reader)?,
            online_offset: read_u16le(&mut reader)?,
            title_font: read_replay_string(&mut reader)?,
            is_unplayed: read_u8(&mut reader)? > 0,
            last_played: read_u64le(&mut reader)?,
            is_osz2: read_u8(&mut reader)? > 0,
            folder_name: read_replay_string(&mut reader)?,
            last_checked: read_u64le(&mut reader)?,
            ignore_beatmap_sounds: read_u8(&mut reader)? > 0,
            ignore_beatmap_skin: read_u8(&mut reader)? > 0,
            disable_storyboard: read_u8(&mut reader)? > 0,
            disable_video: read_u8(&mut reader)? > 0,
            visual_override: read_u8(&mut reader)? > 0,
            unknown: if version < 20140609 {
                Some(read_u16le(&mut reader)?)
            } else {
                None
            },
            unknown_modification_date: read_u32le(&mut reader)?,
            mania_scrollspeed: read_u8(&mut reader)?,
        })
    }
}

impl OsuDB {
    /// Parse the osu!.db data from a reader.
    pub fn parse(mut reader: impl io::BufRead) -> Result<OsuDB, Error> {
        let version;
        let beatmap_count;
        Ok(OsuDB {
            version: {
                version = read_u32le(&mut reader)?;
                version
            },
            folder_count: read_u32le(&mut reader)?,
            account_unlocked: read_u8(&mut reader)? > 0,
            unlocked_date: read_u64le(&mut reader)?,
            player_name: read_replay_string(&mut reader)?,
            beatmap_count: {
                beatmap_count = read_u32le(&mut reader)?;
                beatmap_count
            },
            beatmaps: (0..beatmap_count)
                .map(|_| OsuDBBeatmap::parse(&mut reader, version))
                .collect::<Result<Vec<_>, Error>>()?,
            permissions: num::FromPrimitive::from_u8(read_u8(&mut reader)?).unwrap(),
        })
    }
}

// Can't provide a proper test at the moment, will have to figure something out.
#[test]
fn test_osudb_parse() {
    use std::fs::File;

    let osr = File::open("C:\\Program Files\\osu!\\osu!.db").unwrap();
    let db = OsuDB::parse(io::BufReader::new(osr)).unwrap();
}
