use std::fmt;
use std::str::FromStr;

use crate::errors::ParseError;
use crate::math::Point;
use crate::timing::TimestampMillis;

/// Beatmap event
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    /// Background event
    Background(BackgroundEvent),

    /// Video event
    Video(VideoEvent),

    /// Break event
    Break(BreakEvent),

    /// Storyboard Event (not implemented)
    // TODO: implement storyboard events!
    Storyboard(String),
}

/// Used in Event::Background
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackgroundEvent {
    /// Location of the background image relative to the beatmap directory.
    pub filename: String,

    /// Offset in osu!pixels from the center of the screen
    pub offset: Point<i32>,
}

/// Used in Event::Video
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VideoEvent {
    /// The timestamp at which the video starts
    pub start_time: TimestampMillis,

    /// Location of the background image relative to the beatmap directory.
    pub filename: String,

    /// Offset in osu!pixels from the center of the screen
    pub offset: Point<i32>,
}

/// Used in Event::Break
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakEvent {
    /// The timestamp at which the break starts
    pub start_time: TimestampMillis,

    /// The timestamp at which the break ends
    pub end_time: TimestampMillis,
}

impl FromStr for Event {
    type Err = ParseError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split(',').collect::<Vec<_>>();
        let evt_type = parts[0];

        Ok(match evt_type {
            "0" => {
                let filename = parts[2].trim_matches('"').to_string();
                let offset = if let (Some(x), Some(y)) = (parts.get(3), parts.get(4)) {
                    let x_offset = x.parse::<i32>()?;
                    let y_offset = y.parse::<i32>()?;
                    Point::new(x_offset, y_offset)
                } else {
                    Point::new(0, 0)
                };
                Event::Background(BackgroundEvent {
                    filename,
                    offset,
                })
            }
            "1" | "Video" => {
                let start_time = parts[1].parse::<i32>()?;
                let filename = parts[2].trim_matches('"').to_string();
                let offset = if let (Some(x), Some(y)) = (parts.get(3), parts.get(4)) {
                    let x_offset = x.parse::<i32>()?;
                    let y_offset = y.parse::<i32>()?;
                    Point::new(x_offset, y_offset)
                } else {
                    Point::new(0, 0)
                };
                Event::Video(VideoEvent {
                    start_time: TimestampMillis(start_time),
                    filename,
                    offset,
                })
            }
            "2" | "Break" => {
                let start_time = parts[1].parse::<i32>()?;
                let end_time = parts[2].parse::<i32>()?;
                Event::Break(BreakEvent {
                    start_time: TimestampMillis(start_time),
                    end_time: TimestampMillis(end_time),
                })
            }
            _ => Event::Storyboard(line.to_string()),
        })
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Background(evt) => write!(
                f,
                "0,0,{:?},{},{}",
                evt.filename, evt.offset.x, evt.offset.y
            )?,
            Event::Video(evt) => write!(
                f,
                "1,{},{:?},{},{}",
                evt.start_time, evt.filename, evt.offset.x, evt.offset.y
            )?,
            Event::Break(evt) => write!(f, "2,{},{}", evt.start_time, evt.end_time)?,
            Event::Storyboard(line) => write!(f, "{}", line)?,
        }

        Ok(())
    }
}
