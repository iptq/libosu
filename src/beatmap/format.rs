use std::fmt;
use std::str::FromStr;

use num::FromPrimitive;
use regex::Regex;

use crate::color::Color;
use crate::enums::{GridSize, Mode};
use crate::errors::ParseError;
use crate::hitobject::HitObject;
use crate::hitsounds::SampleSet;
use crate::timing::TimingPoint;

use super::Beatmap;

lazy_static! {
    static ref OSU_FORMAT_VERSION_RGX: Regex =
        Regex::new(r"^osu file format v(?P<version>\d+)$").unwrap();
    static ref SECTION_HEADER_RGX: Regex = Regex::new(r"^\[(?P<name>[A-Za-z]+)\]$").unwrap();
    static ref KEY_VALUE_RGX: Regex =
        Regex::new(r"^(?P<key>[A-Za-z0-9]+)\s*:\s*(?P<value>.+)$").unwrap();
}

/// Macro for matching beatmap keys easier.
macro_rules! kvalue {
    ($captures:ident[$name:expr]: str) => {
        $name = String::from(&$captures["value"]);
    };
    ($captures:ident[$name:expr] => str) => {
        String::from(&$captures["value"])
    };
    ($captures:ident[$name:expr]: parse(bool)) => {
        $name = {
            let val = kvalue!($captures[$name] => parse(u8));
            !(val == 0)
        };
    };
    ($captures:ident[$name:expr] => parse($type:ident)) => {
        $captures["value"].parse::<$type>()?
    };
    ($captures:ident[$name:expr]: parse($type:ident)) => {
        $name = $captures["value"].parse::<$type>()?;
    };
}

impl FromStr for Beatmap {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Beatmap, Self::Err> {
        // TODO: actually, replace all the required "default" values with Option<T>s.
        let mut section = "Version".to_owned();
        let mut beatmap = Beatmap::default();
        let mut timing_points = Vec::new();

        let mut timing_point_lines = Vec::new();
        let mut hit_object_lines = Vec::new();

        for line in input.lines() {
            if let Some(captures) = SECTION_HEADER_RGX.captures(&line) {
                section = String::from(&captures["name"]);
                continue;
            }
            // println!("\"{}\" {}", section, line);
            //
            if line.trim().is_empty() {
                continue;
            }

            match section.as_ref() {
                "HitObjects" => {
                    hit_object_lines.push(line);
                }
                "TimingPoints" => {
                    timing_point_lines.push(line);
                }
                "Version" => {
                    if let Some(capture) = OSU_FORMAT_VERSION_RGX.captures(&line) {
                        beatmap.version = capture["version"].parse::<u32>()?;
                    }
                }
                "Colours" => {
                    let color = Color::from_str(line)?;
                    beatmap.colors.push(color);
                }
                _ => {
                    if let Some(captures) = KEY_VALUE_RGX.captures(line) {
                        match &captures["key"] {
                            "AudioFilename" => kvalue!(captures[beatmap.audio_filename]: str),
                            "AudioLeadIn" => kvalue!(captures[beatmap.audio_leadin]: parse(u32)),
                            "PreviewTime" => kvalue!(captures[beatmap.preview_time]: parse(u32)),
                            "Countdown" => kvalue!(captures[beatmap.countdown]: parse(bool)),
                            "SampleSet" => {
                                beatmap.sample_set = {
                                    let sample_set = kvalue!(captures[beatmap.sample_set] => str);
                                    match sample_set.as_ref() {
                                        "None" => SampleSet::None,
                                        "Normal" => SampleSet::Normal,
                                        "Soft" => SampleSet::Soft,
                                        "Drum" => SampleSet::Drum,
                                        s => {
                                            return Err(ParseError::InvalidSampleSet(s.to_owned()))
                                        }
                                    }
                                }
                            }
                            "StackLeniency" => {
                                kvalue!(captures[beatmap.stack_leniency]: parse(f64))
                            }
                            "Mode" => {
                                beatmap.mode = {
                                    let mode = kvalue!(captures[beatmap.mode]=> parse(u8));
                                    match mode {
                                        0 => Mode::Osu,
                                        1 => Mode::Taiko,
                                        2 => Mode::Catch,
                                        3 => Mode::Mania,
                                        _ => return Err(ParseError::InvalidGameMode(mode)),
                                    }
                                }
                            }
                            "LetterBoxInBreaks" => {
                                kvalue!(captures[beatmap.letterbox_in_breaks]: parse(bool))
                            }
                            "WidescreenStoryboard" => {
                                kvalue!(captures[beatmap.widescreen_storyboard]: parse(bool))
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
                                    .map(|n| n.parse::<i32>().unwrap())
                                    .collect()
                            }
                            "DistanceSpacing" => {
                                kvalue!(captures[beatmap.distance_spacing]: parse(f64))
                            }
                            "BeatDivisor" => kvalue!(captures[beatmap.beat_divisor]: parse(u8)),
                            // "GridSize" => kvalue!(captures[beatmap.grid_size]: parse(u8)),
                            "GridSize" => {
                                beatmap.grid_size = {
                                    let grid_size =
                                        kvalue!(captures[beatmap.grid_size]=> parse(u8));
                                    GridSize::from_u8(grid_size)
                                        .ok_or(ParseError::InvalidGridSize(grid_size))?
                                }
                            }
                            "TimelineZoom" => kvalue!(captures[beatmap.timeline_zoom]: parse(f64)),

                            "Title" => kvalue!(captures[beatmap.title]: str),
                            "TitleUnicode" => kvalue!(captures[beatmap.title_unicode]: str),
                            "Artist" => kvalue!(captures[beatmap.artist]: str),
                            "ArtistUnicode" => kvalue!(captures[beatmap.artist_unicode]: str),
                            "Creator" => kvalue!(captures[beatmap.creator]: str),
                            "Version" => kvalue!(captures[beatmap.difficulty_name]: str),
                            "Source" => kvalue!(captures[beatmap.source]: str),
                            "Tags" => {
                                beatmap.tags =
                                    captures["value"].split(' ').map(|s| s.to_owned()).collect()
                            }
                            "BeatmapID" => kvalue!(captures[beatmap.beatmap_id]: parse(i32)),
                            "BeatmapSetID" => kvalue!(captures[beatmap.beatmap_set_id]: parse(i32)),

                            "HPDrainRate" => {
                                kvalue!(captures[beatmap.difficulty.hp_drain_rate]: parse(f32))
                            }
                            "CircleSize" => {
                                kvalue!(captures[beatmap.difficulty.circle_size]: parse(f32))
                            }
                            "OverallDifficulty" => {
                                kvalue!(captures[beatmap.difficulty.overall_difficulty]: parse(f32))
                            }
                            "ApproachRate" => {
                                kvalue!(captures[beatmap.difficulty.approach_rate]: parse(f32))
                            }
                            "SliderMultiplier" => {
                                kvalue!(captures[beatmap.difficulty.slider_multiplier]: parse(f64))
                            }
                            "SliderTickRate" => {
                                kvalue!(captures[beatmap.difficulty.slider_tick_rate]: parse(f64))
                            }

                            _ => (),
                        }
                    }
                }
            }
        }

        // if beatmap.version == 0 {
        //     bail!(
        //         "Could not find osu! file format version line. Check your beatmap and try again."
        //     );
        // }

        // parse timing points
        for line in timing_point_lines {
            let tp = TimingPoint::from_str(line)?;
            timing_points.push(tp);
        }

        timing_points.sort();
        for tp in timing_points.into_iter() {
            beatmap.timing_points.push(tp);
        }
        // beatmap.timing_points.sort_by_key(|tp| tp.time);

        for line in hit_object_lines {
            let obj = HitObject::from_str(line)?;
            beatmap.hit_objects.push(obj);
        }
        beatmap.hit_objects.sort_by_key(|ho| ho.start_time);

        // beatmap.associate_hitobjects();
        Ok(beatmap)
    }
}

impl fmt::Display for Beatmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // version
        // TODO: should probably use a fixed version
        writeln!(f, "osu file format v{}\n", self.version)?;

        // general
        writeln!(f, "[General]")?;
        writeln!(f, "AudioFilename: {}", self.audio_filename)?;
        writeln!(f, "AudioLeadIn: {}", self.audio_leadin)?;
        writeln!(f, "PreviewTime: {}", self.preview_time)?;
        writeln!(f, "Countdown: {}", if self.countdown { 1 } else { 0 })?;
        writeln!(
            f,
            "SampleSet: {}",
            match self.sample_set {
                SampleSet::None => "None",
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
        writeln!(f, "")?;

        // editor
        writeln!(f, "[Editor]")?;
        write!(f, "Bookmarks: ")?;
        for (i, bookmark) in self.bookmarks.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", bookmark)?;
        }
        writeln!(f, "DistanceSpacing: {}", self.distance_spacing)?;
        writeln!(f, "BeatDivisor: {}", self.beat_divisor)?;
        writeln!(f, "GridSize: {}", self.grid_size as u8)?;
        writeln!(f, "TimelineZoom: {}", self.timeline_zoom)?;
        writeln!(f, "")?;

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
        writeln!(f, "")?;

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
        writeln!(f, "")?;

        // timing points
        writeln!(f, "[TimingPoints]")?;
        for timing_point in self.timing_points.iter() {
            writeln!(f, "{}", timing_point)?;
        }
        writeln!(f, "")?;

        // colors
        writeln!(f, "[Colours]")?;
        for (i, color) in self.colors.iter().enumerate() {
            writeln!(f, "Combo{} : {}", i + 1, color)?;
        }
        writeln!(f, "")?;

        // hit objects
        writeln!(f, "[HitObjects]")?;
        for hit_object in self.hit_objects.iter() {
            writeln!(f, "{}", hit_object)?;
        }
        writeln!(f, "")?;

        Ok(())
    }
}
