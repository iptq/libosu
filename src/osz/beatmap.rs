use failure::Error;
use regex::Regex;

use Beatmap;
use HitObject;
use Mode;
use SampleSet;
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
    ($captures:ident[$name:expr]: str) => {
        $name = String::from(&$captures["value"]);
    };
    ($captures:ident[$name:expr] => str) => {
        String::from(&$captures["value"])
    };
    ($captures:ident[$name:expr]: parse(bool)) => {
        $name = {
            let val = kvalue!($captures[$name] => parse(u8));
            if val == 0 {
                false
            } else {
                true
            }
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
    pub fn deserialize_osz(input: String) -> Result<Beatmap, Error> {
        // TODO: actually, replace all the required "default" values with Option<T>s.
        let mut section = "Version".to_owned();
        let mut beatmap = Beatmap::new();

        for line in input.lines() {
            match SECTION_HEADER_RGX.captures(line) {
                Some(captures) => {
                    section = String::from(&captures["name"]);
                    continue;
                }
                None => (),
            }
            // println!("\"{}\" {}", section, line);
            //
            if line.trim().len() == 0 {
                continue;
            }

            match section.as_ref() {
                "HitObjects" => {
                    let obj = HitObject::deserialize_osz(&beatmap, String::from(line))?;
                    beatmap.hit_objects.push(obj);
                }
                "TimingPoints" => {
                    let tp = TimingPoint::deserialize_osz(String::from(line))?;
                    beatmap.timing_points.push(tp);
                }
                "Version" => {
                    if let Some(capture) = OSU_FORMAT_VERSION_RGX.captures(line) {
                        beatmap.version = capture["version"].parse::<u32>()?;
                    }
                }
                _ => if let Some(captures) = KEY_VALUE_RGX.captures(line) {
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
                                    _ => bail!("Invalid sample set '{}'.", sample_set),
                                }
                            }
                        }
                        "StackLeniency" => kvalue!(captures[beatmap.stack_leniency]: parse(f64)),
                        "Mode" => {
                            beatmap.mode = {
                                let mode = kvalue!(captures[beatmap.mode]=> parse(u8));
                                match mode {
                                    0 => Mode::Osu,
                                    1 => Mode::Taiko,
                                    2 => Mode::Catch,
                                    3 => Mode::Mania,
                                    _ => bail!("Invalid game mode: {}", mode),
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
                                .split(",")
                                .map(|n| n.parse::<i32>().unwrap())
                                .collect()
                        }
                        "DistanceSpacing" => {
                            kvalue!(captures[beatmap.distance_spacing]: parse(f64))
                        }
                        "BeatDivisor" => kvalue!(captures[beatmap.beat_divisor]: parse(u8)),
                        "GridSize" => kvalue!(captures[beatmap.grid_size]: parse(u8)),
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
                                captures["value"].split(" ").map(|s| s.to_owned()).collect()
                        }
                        "BeatmapID" => kvalue!(captures[beatmap.beatmap_id]: parse(i32)),
                        "BeatmapSetID" => kvalue!(captures[beatmap.beatmap_set_id]: parse(i32)),

                        _ => (),
                    }
                },
            }
        }
        if beatmap.version == 0 {
            bail!(
                "Could not find osu! file format version line. Check your beatmap and try again."
            );
        }

        // associate hit objects with timing sections
        beatmap.timing_points.sort_unstable_by(|tp1, tp2| tp1.cmp(tp2));
        beatmap.hit_objects.sort_unstable_by(|o1, o2| o1.start_time.cmp(&o2.start_time));

        beatmap.associate_hitobjects();
        Ok(beatmap)
    }

    pub fn serialize_osz(&self) -> Result<String, Error> {
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
            match &self.sample_set {
                &SampleSet::None => "None",
                &SampleSet::Normal => "Normal",
                &SampleSet::Soft => "Soft",
                &SampleSet::Drum => "Drum",
            }
        ));
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
        lines.push(format!("GridSize: {}", self.grid_size));
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
        lines.push("".to_string());

        // events
        lines.push("[Events]".to_string());
        lines.push("".to_string());

        // timing points
        lines.push("[TimingPoints]".to_string());
        for timing_point in self.timing_points.iter() {
            lines.push(timing_point.serialize_osz()?);
        }
        lines.push("".to_string());

        // colors
        lines.push("[Colours]".to_string());
        lines.push("".to_string());

        // hit objects
        lines.push("[HitObjects]".to_string());
        for hit_object in self.hit_objects.iter() {
            lines.push(hit_object.serialize_osz()?);
        }
        lines.push("".to_string());

        Ok(lines.join("\n"))
    }
}
