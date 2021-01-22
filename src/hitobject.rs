use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use num::FromPrimitive;

use crate::errors::{ParseError, ParseResult};
use crate::hitsounds::{Additions, SampleInfo, SampleSet};
use crate::math::Point;
use crate::spline::Spline;
use crate::timing::TimestampMillis;

/// Distinguishes between different types of slider splines.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SliderSplineKind {
    /// Linear is the most straightforward, and literally consists of two endpoints.
    Linear,

    /// Bezier is more complex, using control points to create smooth curves.
    Bezier,

    /// Catmull is a deprecated slider spline used mainly in older maps (looks ugly btw).
    Catmull,

    /// Perfect (circle) splines are circles circumscribed around three control points.
    Perfect,
}

impl fmt::Display for SliderSplineKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SliderSplineKind::Linear => 'L',
                SliderSplineKind::Bezier => 'B',
                SliderSplineKind::Catmull => 'C',
                SliderSplineKind::Perfect => 'P',
            }
        )
    }
}

/// Extra information provided by a slider.
#[derive(Clone, Debug)]
pub struct SliderInfo {
    /// The algorithm used to calculate the spline.
    pub kind: SliderSplineKind,

    /// The control points that make up the body of the slider.
    pub control_points: Vec<Point<i32>>,

    /// The number of times this slider should repeat.
    pub num_repeats: u32,

    /// How long this slider is in pixels.
    pub pixel_length: f64,

    /// Hitsounds on each repeat of the slider
    pub edge_additions: Vec<Additions>,

    /// Additions on each repeat of the slider
    pub edge_samplesets: Vec<(SampleSet, SampleSet)>,
}

/// Extra information provided by a spinner.
#[derive(Clone, Debug)]
pub struct SpinnerInfo {
    /// The time at which the slider ends.
    pub end_time: TimestampMillis,
}

/// Distinguishes between different types of hit objects.
#[derive(Clone, Debug)]
pub enum HitObjectKind {
    /// Regular hit circle.
    Circle,

    /// Slider.
    Slider(SliderInfo),

    /// Spinner.
    Spinner(SpinnerInfo),
}

impl HitObjectKind {
    /// Is the given HitObject a hit circle?
    pub fn is_circle(&self) -> bool {
        matches!(self, HitObjectKind::Circle)
    }

    /// Is the given HitObject a slider?
    pub fn is_slider(&self) -> bool {
        matches!(self, HitObjectKind::Slider(_))
    }

    /// Is the given HitObject a spinner?
    pub fn is_spinner(&self) -> bool {
        matches!(self, HitObjectKind::Spinner(_))
    }
}

/// Represents a single hit object.
#[derive(Clone, Debug)]
pub struct HitObject {
    /// The position on the map at which this hit object is located (head for sliders).
    pub pos: Point<i32>,

    /// When this hit object occurs during the map.
    pub start_time: TimestampMillis,

    /// The kind of HitObject this represents (circle, slider, spinner).
    pub kind: HitObjectKind,

    /// Whether or not this object begins a new combo.
    pub new_combo: bool,

    /// The number of combo colors to skip
    pub skip_color: i32,

    /// The hitsound additions attached to this hit object.
    pub additions: Additions,

    /// The sample used to play the hitsound assigned to this hit object.
    pub sample_info: SampleInfo,
}

impl HitObject {
    /// Computes the point at which the hitobject ends
    pub fn end_pos(&self) -> Point<f64> {
        match &self.kind {
            HitObjectKind::Slider(info) => {
                if info.num_repeats % 2 == 0 {
                    self.pos.to_float().unwrap()
                } else {
                    let mut control_points = vec![self.pos];
                    control_points.extend(&info.control_points);
                    let spline = Spline::from_control(
                        info.kind,
                        control_points.as_ref(),
                        Some(info.pixel_length),
                    );
                    spline.end_point()
                }
            }
            _ => self.pos.to_float().unwrap(),
        }
    }
}

impl Ord for HitObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for HitObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HitObject {}

impl PartialEq for HitObject {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time
    }
}

impl FromStr for HitObject {
    type Err = ParseError;

    /// Creates a HitObject from the *.osz format
    fn from_str(input: &str) -> Result<HitObject, Self::Err> {
        let parts = input.split(',').collect::<Vec<_>>();

        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        let timestamp = parts[2].parse::<i32>()?;
        let obj_type = parts[3].parse::<i32>()?;
        let additions_bits = parts[4].parse::<u32>()?;
        let additions = Additions::from_bits(additions_bits).unwrap();

        let start_time = TimestampMillis(timestamp);

        // color is the top 3 bits of the "type" string, since there's a possible of 8 different
        // combo colors max
        let skip_color = (obj_type >> 4) & 0b111;

        let new_combo = (obj_type & 4) == 4;
        let sample_info;
        let kind = match obj_type {
            // hit circle
            o if (o & 1) == 1 => {
                sample_info = if let Some(s) = parts.get(5) {
                    SampleInfo::from_str(s)?
                } else {
                    SampleInfo::default()
                };
                HitObjectKind::Circle
            }

            //slider
            o if (o & 2) == 2 => {
                let mut ctl_parts = parts[5].split('|').collect::<Vec<_>>();
                let num_repeats = parts[6].parse::<u32>()?;
                let slider_type = ctl_parts.remove(0);

                // slider duration = pixelLength / (100.0 * SliderMultiplier) * BeatDuration
                // from the osu wiki
                let pixel_length = parts[7].parse::<f64>()?;

                let edge_additions = if parts.len() > 8 {
                    parts[8]
                        .split('|')
                        .map(|n| n.parse::<u32>().map(|b| Additions::from_bits(b).unwrap()))
                        .collect::<Result<Vec<_>, _>>()?
                } else {
                    vec![Additions::empty()]
                };

                let edge_samplesets = if parts.len() > 9 {
                    parts[9]
                        .split('|')
                        .map(|s| {
                            let s2 = s.split(':').collect::<Vec<_>>();
                            let normal = s2[0].parse::<u32>()?;
                            let additions = s2[1].parse::<u32>()?;
                            Ok((
                                SampleSet::from_u32(normal).unwrap(),
                                SampleSet::from_u32(additions).unwrap(),
                            ))
                        })
                        .collect::<ParseResult<Vec<_>>>()?
                } else {
                    vec![(SampleSet::None, SampleSet::None)]
                };

                sample_info = if parts.len() > 10 {
                    SampleInfo::from_str(parts[10])?
                } else {
                    SampleInfo::default()
                };

                HitObjectKind::Slider(SliderInfo {
                    num_repeats,
                    kind: match slider_type {
                        "L" => SliderSplineKind::Linear,
                        "B" => SliderSplineKind::Bezier,
                        "C" => SliderSplineKind::Catmull,
                        "P" => SliderSplineKind::Perfect,
                        s => return Err(ParseError::InvalidSliderType(s.to_owned())),
                    },
                    control_points: ctl_parts
                        .into_iter()
                        .map(|s| {
                            let p = s.split(':').collect::<Vec<_>>();
                            Point::new(p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap())
                        })
                        .collect(),
                    pixel_length,
                    edge_additions,
                    edge_samplesets,
                })
            }

            // spinner
            o if (o & 8) == 8 => {
                let end_time = parts[5].parse::<i32>()?;
                sample_info = if let Some(s) = parts.get(6) {
                    SampleInfo::from_str(s)?
                } else {
                    SampleInfo::default()
                };
                HitObjectKind::Spinner(SpinnerInfo {
                    end_time: TimestampMillis(end_time),
                })
            }
            o => {
                return Err(ParseError::InvalidObjectType(o));
            }
        };

        let hit_obj = HitObject {
            kind,
            pos: Point::new(x, y),
            new_combo,
            additions,
            skip_color,
            start_time,
            sample_info,
        };

        Ok(hit_obj)
    }
}

impl fmt::Display for HitObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.pos.x, self.pos.y, self.start_time.0)?;

        // object type
        let obj_type = match self.kind {
            HitObjectKind::Circle => 1,
            HitObjectKind::Slider { .. } => 2,
            HitObjectKind::Spinner { .. } => 8,
        } | if self.new_combo { 4 } else { 0 }
            | self.skip_color;
        write!(f, ",{}", obj_type)?;

        // additions
        write!(f, ",{}", self.additions.bits())?;

        match &self.kind {
            HitObjectKind::Circle => {
                // no additional params
            }

            HitObjectKind::Slider(info) => {
                write!(f, ",{}", info.kind)?;
                for point in info.control_points.iter() {
                    write!(f, "|{}:{}", point.x, point.y)?;
                }

                write!(f, ",{}", info.num_repeats)?;
                write!(f, ",{}", info.pixel_length)?;

                write!(f, ",")?;
                for (i, additions) in info.edge_additions.iter().enumerate() {
                    if i > 0 {
                        write!(f, "|")?;
                    }
                    write!(f, "{}", additions.bits())?;
                }

                write!(f, ",")?;
                for (i, (normal_set, addition_set)) in info.edge_samplesets.iter().enumerate() {
                    if i > 0 {
                        write!(f, "|")?;
                    }
                    write!(f, "{}:{}", *normal_set as u8, *addition_set as u8)?;
                }
            }

            HitObjectKind::Spinner(info) => {
                write!(f, ",{}", info.end_time)?;
            }
        }

        // hitsample
        write!(f, ",{}", self.sample_info)?;

        Ok(())
    }
}
