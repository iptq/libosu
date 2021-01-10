use regex::Regex;
use num::FromPrimitive;

use crate::beatmap::Beatmap;
use crate::color::Color;
use crate::enums::{Mode, GridSize};
use crate::hitobject::HitObject;
use crate::hitsounds::SampleSet;
use crate::parsing::{Error, Result};
use crate::timing::{TimingPoint, TimingPointKind};

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

impl Beatmap {
    /// Creates a Beatmap from the *.osz format
    pub fn from_osz(input: impl AsRef<str>) -> Result<Beatmap> {
        // TODO: actually, replace all the required "default" values with Option<T>s.
        let mut section = "Version".to_owned();
        let mut beatmap = Beatmap::default();
        let mut timing_points = Vec::new();

        let mut timing_point_lines = Vec::new();
        let mut hit_object_lines = Vec::new();

        for line in input.as_ref().lines() {
            if let Some(captures) = SECTION_HEADER_RGX.captures(line) {
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
                    if let Some(capture) = OSU_FORMAT_VERSION_RGX.captures(line) {
                        beatmap.version = capture["version"].parse::<u32>()?;
                    }
                }
                "Colours" => {
                    let color = parse_color(line)?;
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
                                        s => return Err(Error::InvalidSampleSet(s.to_owned())),
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
                                        _ => return Err(Error::InvalidGameMode(mode)),
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
                                    let grid_size = kvalue!(captures[beatmap.grid_size]=> parse(u8));
                                    GridSize::from_u8(grid_size).ok_or_else(|| Error::InvalidGridSize(grid_size))?
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
        let mut prev = None;
        for line in timing_point_lines {
            let tp = TimingPoint::from_osz(line, &prev)?;
            if let TimingPointKind::Uninherited { .. } = tp.kind {
                prev = Some(tp.clone());
            }
            timing_points.push(tp);
        }

        // set their parents now
        timing_points.sort();
        for tp in timing_points.into_iter() {
            beatmap.timing_points.push(tp);
        }
        // beatmap.timing_points.sort_by_key(|tp| tp.time);

        for line in hit_object_lines {
            let obj = HitObject::from_osz(line)?;
            beatmap.hit_objects.push(obj);
        }
        beatmap.hit_objects.sort_by_key(|ho| ho.start_time);

        // beatmap.associate_hitobjects();
        Ok(beatmap)
    }

    /// Serializes this Beatmap into the *.osz format.
    pub fn as_osz(&self) -> Result<String> {
        let mut lines = vec![];

        // version
        // TODO: should probably use a fixed version
        lines.push(format!("osu file format v{}", self.version));
        lines.push("".to_string()); // new line

        // general
        lines.push("[General]".to_string());
        lines.push(format!("AudioFilename: {}", self.audio_filename));
        lines.push(format!("AudioLeadIn: {}", self.audio_leadin));
        lines.push(format!("PreviewTime: {}", self.preview_time));
        lines.push(format!("Countdown: {}", if self.countdown { 1 } else { 0 }));
        lines.push(format!(
            "SampleSet: {}",
            match self.sample_set {
                SampleSet::None => "None",
                SampleSet::Normal => "Normal",
                SampleSet::Soft => "Soft",
                SampleSet::Drum => "Drum",
            }
        ));
        lines.push(format!("StackLeniency: {}", self.stack_leniency));
        lines.push(format!("Mode: {}", self.mode as u32));
        lines.push(format!(
            "LetterboxInBreaks: {}",
            if self.letterbox_in_breaks { 1 } else { 0 }
        ));
        lines.push(format!(
            "WidescreenStoryboard: {}",
            if self.widescreen_storyboard { 1 } else { 0 }
        ));
        lines.push("".to_string());

        // editor
        lines.push("[Editor]".to_string());
        lines.push(format!(
            "Bookmarks: {}",
            self.bookmarks
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
        lines.push(format!("DistanceSpacing: {}", self.distance_spacing));
        lines.push(format!("BeatDivisor: {}", self.beat_divisor));
        lines.push(format!("GridSize: {}", self.grid_size as u8));
        lines.push(format!("TimelineZoom: {}", self.timeline_zoom));
        lines.push("".to_string());

        // metadata
        lines.push("[Metadata]".to_string());
        lines.push(format!("Title:{}", self.title));
        lines.push(format!("TitleUnicode:{}", self.title_unicode));
        lines.push(format!("Artist:{}", self.artist));
        lines.push(format!("ArtistUnicode:{}", self.artist_unicode));
        lines.push(format!("Creator:{}", self.creator));
        lines.push(format!("Version:{}", self.difficulty_name));
        lines.push(format!("Source:{}", self.source));
        lines.push(format!("Tags:{}", self.tags.join(" ")));
        lines.push(format!("BeatmapID:{}", self.beatmap_id));
        lines.push(format!("BeatmapSetID:{}", self.beatmap_set_id));
        lines.push("".to_string());

        // difficulty
        lines.push("[Difficulty]".to_string());
        lines.push(format!("HPDrainRate:{}", self.difficulty.hp_drain_rate));
        lines.push(format!("CircleSize:{}", self.difficulty.circle_size));
        lines.push(format!(
            "OverallDifficulty:{}",
            self.difficulty.overall_difficulty
        ));
        lines.push(format!("ApproachRate:{}", self.difficulty.approach_rate));
        lines.push(format!(
            "SliderMultiplier:{}",
            self.difficulty.slider_multiplier
        ));
        lines.push(format!(
            "SliderTickRate:{}",
            self.difficulty.slider_tick_rate
        ));

        // events
        lines.push("[Events]".to_string());
        lines.push("".to_string());

        // timing points
        lines.push("[TimingPoints]".to_string());
        for timing_point in self.timing_points.iter() {
            lines.push(timing_point.as_osz()?);
        }
        lines.push("".to_string());

        // colors
        lines.push("[Colours]".to_string());
        for (i, color) in self.colors.iter().enumerate() {
            lines.push(format!("Combo{} : {}", i + 1, color_str(&color)));
        }
        lines.push("".to_string());

        // hit objects
        lines.push("[HitObjects]".to_string());
        for hit_object in self.hit_objects.iter() {
            lines.push(hit_object.as_osz()?);
        }
        lines.push("".to_string());

        Ok(lines.join("\n"))
    }
}

fn parse_color(line: &str) -> Result<Color> {
    let mut s = line.split(" : ");
    s.next();
    let s = s.next().unwrap().split(',').collect::<Vec<_>>();
    let red = s[0].parse::<u8>()?;
    let green = s[1].parse::<u8>()?;
    let blue = s[2].parse::<u8>()?;
    Ok(Color { red, green, blue })
}

fn color_str(color: &Color) -> String {
    format!("{},{},{}", color.red, color.green, color.blue)
}
