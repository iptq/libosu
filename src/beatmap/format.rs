use std::fmt;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::str::FromStr;

use num::FromPrimitive;
use regex::Regex;

use crate::data::{GridSize, Mode};
use crate::errors::ParseError;
use crate::events::Event;
use crate::hitobject::HitObject;
use crate::hitsounds::SampleSet;
use crate::timing::TimingPoint;
use crate::{color::Color, timing::Millis};

use super::Beatmap;

lazy_static! {
    static ref OSU_FORMAT_VERSION_RGX: Regex =
        Regex::new(r"^osu file format v(?P<version>\d+)$").expect("compile");
    static ref SECTION_HEADER_RGX: Regex =
        Regex::new(r"^\[(?P<name>[A-Za-z]+)\]$").expect("compile");
    static ref KEY_VALUE_RGX: Regex =
        Regex::new(r"^(?P<key>[A-Za-z0-9]+)\s*:\s*(?P<value>.+)$").expect("compile");
}

/// Macro for matching beatmap keys easier.
macro_rules! kvalue {
    ($line:expr, $captures:ident[$name:expr]: str) => {
        { $name = String::from(&$captures["value"]); }
    };
    ($line:expr, $captures:ident[$name:expr] => str) => {
        String::from(&$captures["value"])
    };
    ($line:expr, $captures:ident[$name:expr]: parse(bool)) => {
        { $name = {
            let val = kvalue!($line, $captures[$name] => parse(u8));
            !(val == 0)
        }; }
    };
    ($line:expr, $captures:ident[$name:expr] => parse($type:ident)) => {
        $captures["value"].parse::<$type>()
            .map_err(|err| BeatmapParseError { line: $line, inner: err.into() })?
    };
    ($line:expr, $captures:ident[$name:expr]: parse($type:ident)) => {
        $name = $captures["value"].parse::<$type>()
            .map_err(|err| BeatmapParseError { line: $line, inner: err.into() })?
    };
}

/// Errors that could occur while parsing beatmaps
#[derive(Debug)]
pub struct BeatmapParseError {
    /// The line number where the error occurred
    pub line: usize,

    /// The kind of error that occurred
    pub inner: ParseError,
}

impl fmt::Display for BeatmapParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error on line {}: {}", self.line, self.inner)
    }
}

impl std::error::Error for BeatmapParseError {}

impl FromStr for Beatmap {
    type Err = BeatmapParseError;

    fn from_str(input: &str) -> Result<Beatmap, Self::Err> {
        let mut curs = Cursor::new(input);
        Beatmap::parse(&mut curs)
    }
}

/// APIs related to parsing and serializing .osu files
impl Beatmap {
    /// Parse a beatmap from any `Read`er
    pub fn parse(reader: impl Read) -> Result<Beatmap, BeatmapParseError> {
        let reader = BufReader::new(reader);

        // TODO: actually, replace all the required "default" values with Option<T>s.
        let mut section = "Version".to_owned();
        let mut beatmap = Beatmap::default();

        for (i, line) in reader.lines().enumerate() {
            let line_no = i + 1;
            let line = line.map_err(|err| BeatmapParseError {
                line: line_no,
                inner: err.into(),
            })?;
            let line = line.as_ref();

            if let Some(captures) = SECTION_HEADER_RGX.captures(line) {
                section = String::from(&captures["name"]);
                continue;
            }

            // skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            match section.as_ref() {
                "Events" => {
                    if line.starts_with("//") {
                        continue;
                    }
                    let evt = Event::from_str(line).map_err(|err| BeatmapParseError {
                        line: line_no,
                        inner: err,
                    })?;
                    beatmap.events.push(evt);
                }
                "HitObjects" => {
                    let obj = HitObject::from_str(line).map_err(|err| BeatmapParseError {
                        line: line_no,
                        inner: err,
                    })?;
                    beatmap.hit_objects.push(obj);
                }
                "TimingPoints" => {
                    let tp = TimingPoint::from_str(line).map_err(|err| BeatmapParseError {
                        line: line_no,
                        inner: err,
                    })?;
                    beatmap.timing_points.push(tp);
                }
                "Version" => {
                    if let Some(capture) = OSU_FORMAT_VERSION_RGX.captures(line) {
                        beatmap.version =
                            capture["version"]
                                .parse::<u32>()
                                .map_err(|err| BeatmapParseError {
                                    line: line_no,
                                    inner: err.into(),
                                })?;
                    }
                }
                "Colours" => {
                    let color = Color::from_str(line).map_err(|err| BeatmapParseError {
                        line: line_no,
                        inner: err,
                    })?;
                    beatmap.colors.push(color);
                }
                _ => {
                    if let Some(captures) = KEY_VALUE_RGX.captures(line) {
                        match &captures["key"] {
                            "AudioFilename" => {
                                kvalue!(line_no, captures[beatmap.audio_filename]: str)
                            }
                            "AudioLeadIn" => {
                                let ms =
                                    kvalue!(line_no, captures[beatmap.audio_leadin] => parse(i32));
                                beatmap.audio_leadin = Millis(ms);
                            }
                            "PreviewTime" => {
                                let ms =
                                    kvalue!(line_no, captures[beatmap.preview_time] => parse(i32));
                                beatmap.preview_time = Millis(ms);
                            }
                            "Countdown" => {
                                kvalue!(line_no, captures[beatmap.countdown]: parse(bool))
                            }
                            "SampleSet" => {
                                beatmap.sample_set = {
                                    let sample_set =
                                        kvalue!(line_no, captures[beatmap.sample_set] => str);
                                    match sample_set.as_ref() {
                                        "None" => SampleSet::Default,
                                        "Normal" => SampleSet::Normal,
                                        "Soft" => SampleSet::Soft,
                                        "Drum" => SampleSet::Drum,
                                        s => {
                                            return Err(BeatmapParseError {
                                                line: line_no,
                                                inner: ParseError::InvalidSampleSetString(
                                                    s.to_owned(),
                                                ),
                                            })
                                        }
                                    }
                                }
                            }
                            "StackLeniency" => {
                                kvalue!(line_no, captures[beatmap.stack_leniency]: parse(f64))
                            }
                            "Mode" => {
                                beatmap.mode = {
                                    let mode = kvalue!(line_no, captures[beatmap.mode]=> parse(u8));
                                    match mode {
                                        0 => Mode::Osu,
                                        1 => Mode::Taiko,
                                        2 => Mode::Catch,
                                        3 => Mode::Mania,
                                        _ => {
                                            return Err(BeatmapParseError {
                                                line: line_no,
                                                inner: ParseError::InvalidGameMode(mode),
                                            })
                                        }
                                    }
                                }
                            }
                            "LetterBoxInBreaks" => {
                                kvalue!(line_no, captures[beatmap.letterbox_in_breaks]: parse(bool))
                            }
                            "WidescreenStoryboard" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.widescreen_storyboard]: parse(bool)
                                )
                            }

                            "Bookmarks" => {
                                beatmap.bookmarks = captures["value"]
                                    .trim()
                                    .split(',')
                                    .filter_map(|s| {
                                        let s = s.trim();
                                        if s.is_empty() {
                                            None
                                        } else {
                                            Some(s)
                                        }
                                    })
                                    .map(|n| {
                                        n.parse::<i32>().map_err(|err| BeatmapParseError {
                                            line: line_no,
                                            inner: err.into(),
                                        })
                                    })
                                    .collect::<Result<Vec<_>, BeatmapParseError>>()?
                            }
                            "DistanceSpacing" => {
                                kvalue!(line_no, captures[beatmap.distance_spacing]: parse(f64))
                            }
                            "BeatDivisor" => {
                                kvalue!(line_no, captures[beatmap.beat_divisor]: parse(u8))
                            }
                            // "GridSize" => kvalue!(captures[beatmap.grid_size]: parse(u8)),
                            "GridSize" => {
                                beatmap.grid_size = {
                                    let grid_size =
                                        kvalue!(line_no, captures[beatmap.grid_size]=> parse(u8));
                                    GridSize::from_u8(grid_size)
                                        .ok_or(ParseError::InvalidGridSize(grid_size))
                                        .map_err(|err| BeatmapParseError {
                                            line: line_no,
                                            inner: err,
                                        })?
                                }
                            }
                            "TimelineZoom" => {
                                kvalue!(line_no, captures[beatmap.timeline_zoom]: parse(f64))
                            }

                            "Title" => kvalue!(line_no, captures[beatmap.title]: str),
                            "TitleUnicode" => {
                                kvalue!(line_no, captures[beatmap.title_unicode]: str)
                            }
                            "Artist" => kvalue!(line_no, captures[beatmap.artist]: str),
                            "ArtistUnicode" => {
                                kvalue!(line_no, captures[beatmap.artist_unicode]: str)
                            }
                            "Creator" => kvalue!(line_no, captures[beatmap.creator]: str),
                            "Version" => kvalue!(line_no, captures[beatmap.difficulty_name]: str),
                            "Source" => kvalue!(line_no, captures[beatmap.source]: str),
                            "Tags" => {
                                beatmap.tags =
                                    captures["value"].split(' ').map(|s| s.to_owned()).collect()
                            }
                            "BeatmapID" => {
                                kvalue!(line_no, captures[beatmap.beatmap_id]: parse(i32))
                            }
                            "BeatmapSetID" => {
                                kvalue!(line_no, captures[beatmap.beatmap_set_id]: parse(i32))
                            }

                            "HPDrainRate" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.hp_drain_rate]: parse(f32)
                                )
                            }
                            "CircleSize" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.circle_size]: parse(f32)
                                )
                            }
                            "OverallDifficulty" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.overall_difficulty]: parse(f32)
                                )
                            }
                            "ApproachRate" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.approach_rate]: parse(f32)
                                )
                            }
                            "SliderMultiplier" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.slider_multiplier]: parse(f64)
                                )
                            }
                            "SliderTickRate" => {
                                kvalue!(
                                    line_no,
                                    captures[beatmap.difficulty.slider_tick_rate]: parse(f64)
                                )
                            }

                            _ => (),
                        }
                    }
                }
            }
        }

        // sort timing points and hit objects
        beatmap.timing_points.sort_by_key(|tp| tp.time);
        beatmap.hit_objects.sort_by_key(|ho| ho.start_time);
        Ok(beatmap)
    }

    /// Write this beatmap to any `Write`r
    pub fn write(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // TODO: write line-by-line
        let beatmap = format!("{}", self);
        w.write_all(beatmap.as_bytes())?;
        Ok(())
    }
}

impl fmt::Display for Beatmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // version
        // TODO: should probably use a fixed version
        writeln!(f, "osu file format v{}", self.version)?;
        writeln!(f)?;

        // general
        writeln!(f, "[General]")?;
        writeln!(f, "AudioFilename: {}", self.audio_filename)?;
        writeln!(f, "AudioLeadIn: {}", self.audio_leadin.0)?;
        writeln!(f, "PreviewTime: {}", self.preview_time.0)?;
        writeln!(f, "Countdown: {}", if self.countdown { 1 } else { 0 })?;
        writeln!(
            f,
            "SampleSet: {}",
            match self.sample_set {
                SampleSet::Default => "None",
                SampleSet::Normal => "Normal",
                SampleSet::Soft => "Soft",
                SampleSet::Drum => "Drum",
            }
        )?;
        writeln!(f, "StackLeniency: {}", self.stack_leniency)?;
        writeln!(f, "Mode: {}", self.mode as u32)?;
        writeln!(
            f,
            "LetterboxInBreaks: {}",
            if self.letterbox_in_breaks { 1 } else { 0 }
        )?;
        writeln!(
            f,
            "WidescreenStoryboard: {}",
            if self.widescreen_storyboard { 1 } else { 0 }
        )?;
        writeln!(f)?;

        // editor
        writeln!(f, "[Editor]")?;
        write!(f, "Bookmarks: ")?;
        for (i, bookmark) in self.bookmarks.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", bookmark)?;
        }
        writeln!(f)?;
        writeln!(f, "DistanceSpacing: {}", self.distance_spacing)?;
        writeln!(f, "BeatDivisor: {}", self.beat_divisor)?;
        writeln!(f, "GridSize: {}", self.grid_size as u8)?;
        writeln!(f, "TimelineZoom: {}", self.timeline_zoom)?;
        writeln!(f)?;

        // metadata
        writeln!(f, "[Metadata]")?;
        writeln!(f, "Title:{}", self.title)?;
        writeln!(f, "TitleUnicode:{}", self.title_unicode)?;
        writeln!(f, "Artist:{}", self.artist)?;
        writeln!(f, "ArtistUnicode:{}", self.artist_unicode)?;
        writeln!(f, "Creator:{}", self.creator)?;
        writeln!(f, "Version:{}", self.difficulty_name)?;
        writeln!(f, "Source:{}", self.source)?;
        writeln!(f, "Tags:{}", self.tags.join(" "))?;
        writeln!(f, "BeatmapID:{}", self.beatmap_id)?;
        writeln!(f, "BeatmapSetID:{}", self.beatmap_set_id)?;
        writeln!(f)?;

        // difficulty
        writeln!(f, "[Difficulty]")?;
        writeln!(f, "HPDrainRate:{}", self.difficulty.hp_drain_rate)?;
        writeln!(f, "CircleSize:{}", self.difficulty.circle_size)?;
        writeln!(
            f,
            "OverallDifficulty:{}",
            self.difficulty.overall_difficulty
        )?;
        writeln!(f, "ApproachRate:{}", self.difficulty.approach_rate)?;
        writeln!(f, "SliderMultiplier:{}", self.difficulty.slider_multiplier)?;
        writeln!(f, "SliderTickRate:{}", self.difficulty.slider_tick_rate)?;

        // events
        writeln!(f, "[Events]")?;
        for event in self.events.iter() {
            writeln!(f, "{}", event)?;
        }
        writeln!(f)?;

        // timing points
        writeln!(f, "[TimingPoints]")?;
        for timing_point in self.timing_points.iter() {
            writeln!(f, "{}", timing_point)?;
        }
        writeln!(f)?;

        // colors
        writeln!(f, "[Colours]")?;
        for (i, color) in self.colors.iter().enumerate() {
            writeln!(f, "Combo{} : {}", i + 1, color)?;
        }
        writeln!(f)?;

        // hit objects
        writeln!(f, "[HitObjects]")?;
        for hit_object in self.hit_objects.iter() {
            writeln!(f, "{}", hit_object)?;
        }
        writeln!(f)?;

        Ok(())
    }
}
