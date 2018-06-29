use failure::Error;
use regex::Regex;

use serde::{Deserializer, OsuFormat, Serializer};
use Beatmap;
use HitObject;
use Mode;
use TimingPoint;

lazy_static! {
    static ref OSU_FORMAT_VERSION_RGX: Regex =
        Regex::new(r"^osu file format v(?P<version>\d+)$").unwrap();
    static ref SECTION_HEADER_RGX: Regex = Regex::new(r"^\[(?P<name>[A-Za-z]+)\]$").unwrap();
    static ref KEY_VALUE_RGX: Regex =
        Regex::new(r"^(?P<key>[A-Za-z0-9]+)\s*:\s*(?P<value>.+)$").unwrap();
}

/// Macro for matching beatmap keys easier.
macro_rules! kvalue {
    ($captures:ident[$name:ident]: str) => {
        $name = String::from(&$captures["value"]);
    };
    ($captures:ident[$name:ident]: parse($type:ident)) => {
        $name = $captures["value"].parse::<$type>()?;
    };
}

impl<'map> Deserializer<OsuFormat> for Beatmap<'map> {
    type Output = Beatmap<'map>;
    fn deserialize(input: OsuFormat) -> Result<Self::Output, Error> {
        // TODO: actually, replace all the required "default" values with Option<T>s.
        let mut section = "Version".to_owned();
        let mut version = 0;

        let mut audio_filename = String::new();
        let mut audio_leadin = 0;
        let mut preview_time = 0;
        let mut countdown = 0;
        let mut stack_leniency = 0.0;
        let mut mode = 0;
        let mut letterbox_in_breaks = 0;
        let mut widescreen_storyboard = 0;

        let mut bookmarks = vec![];
        let mut distance_spacing = 0.0;
        let mut beat_divisor = 0;
        let mut grid_size = 0;
        let mut timeline_zoom = 0.0;

        let mut title = String::new();
        let mut title_unicode = String::new();
        let mut artist = String::new();
        let mut artist_unicode = String::new();
        let mut creator = String::new();
        let mut difficulty_name = String::new();
        let mut source = String::new();
        let mut tags = vec![];
        let mut beatmap_id = 0;
        let mut beatmap_set_id = -1;

        let mut hit_objects = Vec::new();
        let mut timing_points = Vec::new();

        for line in input.lines() {
            match SECTION_HEADER_RGX.captures(line) {
                Some(captures) => {
                    section = String::from(&captures["name"]);
                    continue;
                }
                None => (),
            }
            // println!("\"{}\" {}", section, line);
            match section.as_ref() {
                "HitObjects" => {
                    let obj = HitObject::deserialize(String::from(line))?;
                    hit_objects.push(obj);
                }
                "Version" => {
                    if let Some(capture) = OSU_FORMAT_VERSION_RGX.captures(line) {
                        version = capture["version"].parse::<u32>()?;
                    }
                }
                _ => if let Some(captures) = KEY_VALUE_RGX.captures(line) {
                    match &captures["key"] {
                        "AudioFilename" => kvalue!(captures[audio_filename]: str),
                        "AudioLeadIn" => kvalue!(captures[audio_leadin]: parse(u32)),
                        "PreviewTime" => kvalue!(captures[preview_time]: parse(u32)),
                        "Countdown" => kvalue!(captures[countdown]: parse(u8)),
                        "StackLeniency" => kvalue!(captures[stack_leniency]: parse(f64)),
                        "Mode" => kvalue!(captures[mode]: parse(u8)),
                        "LetterBoxInBreaks" => kvalue!(captures[letterbox_in_breaks]: parse(u8)),
                        "WidescreenStoryboard" => {
                            kvalue!(captures[widescreen_storyboard]: parse(u8))
                        }

                        "Bookmarks" => {
                            bookmarks = captures["value"]
                                .split(",")
                                .map(|n| n.parse::<i32>().unwrap())
                                .collect()
                        }
                        "DistanceSpacing" => kvalue!(captures[distance_spacing]: parse(f64)),
                        "BeatDivisor" => kvalue!(captures[beat_divisor]: parse(u8)),
                        "GridSize" => kvalue!(captures[grid_size]: parse(u8)),
                        "TimelineZoom" => kvalue!(captures[timeline_zoom]: parse(f64)),

                        "Title" => kvalue!(captures[title]: str),
                        "TitleUnicode" => kvalue!(captures[title_unicode]: str),
                        "Artist" => kvalue!(captures[artist]: str),
                        "ArtistUnicode" => kvalue!(captures[artist_unicode]: str),
                        "Creator" => kvalue!(captures[creator]: str),
                        "Version" => kvalue!(captures[difficulty_name]: str),
                        "Source" => kvalue!(captures[source]: str),
                        "Tags" => {
                            tags = captures["value"].split(" ").map(|s| s.to_owned()).collect()
                        }
                        "BeatmapID" => kvalue!(captures[beatmap_id]: parse(i32)),
                        "BeatmapSetID" => kvalue!(captures[beatmap_set_id]: parse(i32)),

                        _ => (),
                    }
                },
            }
        }
        if version == 0 {
            bail!(
                "Could not find osu! file format version line. Check your beatmap and try again."
            );
        }
        Ok(Beatmap {
            version,
            audio_filename,
            audio_leadin,
            preview_time,
            countdown: countdown > 0,
            stack_leniency,
            mode: match mode {
                0 => Mode::Osu,
                1 => Mode::Taiko,
                2 => Mode::Catch,
                3 => Mode::Mania,
                _ => panic!("Invalid game mode."),
            },
            letterbox_in_breaks: letterbox_in_breaks > 0,
            widescreen_storyboard: widescreen_storyboard > 0,

            bookmarks,
            distance_spacing,
            beat_divisor,
            grid_size,
            timeline_zoom,

            title,
            title_unicode,
            artist,
            artist_unicode,
            creator,
            difficulty_name,
            source,
            tags,
            beatmap_id,
            beatmap_set_id,

            hit_objects,
            timing_points,
        })
    }
}

impl<'map> Serializer<OsuFormat> for Beatmap<'map> {
    fn serialize(&self) -> Result<OsuFormat, Error> {
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
        // SampleSet
        lines.push(format!("StackLeniency: {}", self.stack_leniency));
        lines.push(format!("Mode: {}", self.mode as u32));
        lines.push(format!(
            "LetterBoxInBreaks: {}",
            if self.letterbox_in_breaks { 1 } else { 0 }
        ));
        lines.push(format!(
            "WidescreenStoryboard: {}",
            if self.widescreen_storyboard { 1 } else { 0 }
        ));
        lines.push("".to_string());

        // editor
        lines.push("[Editor]".to_string());

        Ok(lines.join("\n"))
    }
}
