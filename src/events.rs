use crate::math::Point;
use crate::timing::TimeLocation;

/// Beatmap event
pub enum Event {
    /// Background event
    Background(BackgroundEvent),

    /// Video event
    Video(VideoEvent),

    /// Break event
    Break(BreakEvent),

    #[doc(hidden)]
    _NonExhaustive,
}

/// Used in Event::Background
pub struct BackgroundEvent {
    /// Location of the background image relative to the beatmap directory.
    pub filename: String,

    /// Offset in osu!pixels from the center of the screen
    pub offset: Point<i32>,
}

/// Used in Event::Video
pub struct VideoEvent {
    /// The timestamp at which the video starts
    pub start_time: TimeLocation,

    /// Location of the background image relative to the beatmap directory.
    pub filename: String,

    /// Offset in osu!pixels from the center of the screen
    pub offset: Point<i32>,
}

/// Used in Event::Break
pub struct BreakEvent {
    /// The timestamp at which the break starts
    pub start_time: TimeLocation,

    /// The timestamp at which the break ends
    pub end_time: TimeLocation,
}
