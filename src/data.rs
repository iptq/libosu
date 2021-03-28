#![allow(non_upper_case_globals)] // for the Mods bitflags

use std::ops::BitOr;

/// Integer enumeration of the game's game modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mode {
    /// osu!standard
    #[cfg_attr(feature = "serde", serde(rename = "osu"))]
    Osu = 0,

    /// Taiko
    #[cfg_attr(feature = "serde", serde(rename = "taiko"))]
    Taiko = 1,

    /// Catch the beat
    #[cfg_attr(feature = "serde", serde(rename = "fruits"))]
    Catch = 2,

    /// osu!mania
    #[cfg_attr(feature = "serde", serde(rename = "mania"))]
    Mania = 3,
}

/// Last saved grid size for editor
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridSize {
    /// Tiny grid size (4 osu!px)
    Tiny = 4,

    /// Small grid size (8 osu!px)
    Small = 8,

    /// Medium grid size (16 osu!px)
    Medium = 16,

    /// Large grid size (32 osu!px)
    Large = 32,
}

bitflags! {
    /// Mod listing with their respective bitwise representation.
    ///
    /// This list is ripped directly from the [osu! wiki](https://github.com/ppy/osu-api/wiki).
    #[derive(Default)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Mods: u32 {
        /// No selected mods
        const None = 0;

        /// No-Fail (NF) Makes the player incapabale of failing beatmaps, even if their life drops to zero.
        const NoFail = 1;

        /// Easy (EZ) halves the all difficulty settings for a beatmap, and gives the player 2
        /// extra lives in modes other than taiko.
        const Easy = 2;

        /// Touch Device (TD) is a marker for a score that has been determined to be set on a
        /// touchscreen device. This value used to be No Video (NV), for when video is disabled.
        const TouchDevice = 4;

        /// Hidden (HD) removes approach circles and causes hit objects to fade after appearing.
        const Hidden = 8;

        /// Hard Rock (HR) increases CS by 30% and all other difficulty settings by 40%.
        const HardRock = 16;

        /// Sudden Death (SD) will cause the player to fail after missing a hit object or slider tick.
        const SuddenDeath = 32;

        /// Double Time (DT) increasing the overall speed to 150%.
        const DoubleTime = 64;

        /// Relax (RL) removes the need to tap hitobjects in standard, color judgement in taiko and allows free movement at any speed in catch.
        const Relax = 128;

        /// Half Time (HT) decreases the overall speed to 75%.
        const HalfTime = 256;

        /// Nightcore (NC) has the same effect as doubletime, but increases the pitch and adds a drum tick in the background. Nightcore will only be set alongside doubletime.
        const Nightcore = 512;

        /// Flashlight (FL) limits the visible area of the beatmap.
        const Flashlight = 1024;

        /// Autoplay plays the beatmap automatically.
        const Autoplay = 2048;

        /// SpunOut (SO) spins spinners automatically in osu!standard.
        const SpunOut = 4096;

        /// Autopilot (AP, Relex2) will automatically move the players cursor (osu!standard only).
        const Relax2 = 8192;

        /// Perfect (PF) will fail the player if they miss or a score under 300 on a hit object, only set along with SuddenDeath.
        const Perfect = 16384;

        /// 4Key (4K, xK) forces maps converted into osu!mania to use 4 keys.
        const Key4 = 32768;

        /// 5Key (5K, xK) forces maps converted into osu!mania to use 5 keys.
        const Key5 = 65536;

        /// 6Key (6K, xK) forces maps converted into osu!mania to use 6 keys.
        const Key6 = 131072;

        /// 7Key (7K, xK) forces maps converted into osu!mania to use 7 keys.
        const Key7 = 262144;

        /// 8Key (8K, xK) forces maps converted into osu!mania to use 8 keys.
        const Key8 = 1015808;

        /// Fade In (FI) causes notes start invisible and fade in as they approach the judgement bar, only set along with Hidden (osu!mania only).
        const FadeIn = 1048576;

        /// Random (RD) randomizes note placement (osu!mania only).
        const Random = 2097152;

        /// Cinema (CM, LastMod) only plays the video or storyboard, without any gameplay. Hitsounds will still be heard.
        const LastMod = 4194304;

        /// Target Practice (TP) removes all mapped hitobjects and replaces them with a consistent set of target (osu!standard Cutting Edge only).
        const TargetPractice = 8388608;

        /// 9Key (9K, xK) forces maps converted into osu!mania to use 9 keys.
        const Key9 = 16777216;

        /// 10Key (10K, xK) forces maps converted into osu!mania to use 10 keys, it has been removed from the game.
        const Key10 = 33554432;

        /// 1Key (1K, xK) forces maps converted into osu!mania to use 1 key.
        const Key1 = 67108864;

        /// 3Key (3K, xK) forces maps converted into osu!mania to use 3 keys.
        const Key3 = 134217728;

        /// 2Key (2K, xK) forces maps converted into osu!mania to use 2 keys.
        const Key2 = 268435456;

        /// Bits of Key4, Key5, Key6, Key7, and Key8.
        const KeyMod = Self::Key4.bits | Self::Key5.bits | Self::Key6.bits | Self::Key7.bits |Self::Key8.bits;

        /// Mods allowed to be chosen when FreeMod is enabled in multiplayer.
        const FreeModAllowed = Self::NoFail.bits | Self::Easy.bits | Self::Hidden.bits | Self::HardRock.bits | Self::SuddenDeath.bits | Self::Flashlight.bits | Self::FadeIn.bits | Self::Relax.bits | Self::Relax2.bits | Self::SpunOut.bits | Self::KeyMod.bits;

    }
}

impl Mods {
    /// Attempts to parse mods from a string, delimiter is what goes between mods
    ///
    /// ```
    /// # use libosu::prelude::Mods;
    /// assert_eq!(Mods::parse_from_str("+HD", ""), Some(Mods::Hidden));
    /// assert_eq!(Mods::parse_from_str("+EZDT", ","), None);
    /// assert_eq!(Mods::parse_from_str("+EZ,DT", ","), Some(Mods::Easy | Mods::DoubleTime));
    /// assert_eq!(Mods::parse_from_str("4K|FI|RD", "|"), Some(Mods::Key4 | Mods::FadeIn | Mods::Random));
    /// ```
    pub fn parse_from_str(s: impl AsRef<str>, delimiter: impl AsRef<str>) -> Option<Self> {
        // convert string to uppercase
        let s = s.as_ref().to_uppercase();

        // trim off starting +
        let s = s.trim_start_matches('+');

        let mut mods = Mods::None;
        let delim = delimiter.as_ref();
        let mut it = s.chars().peekable();
        loop {
            let ch1 = match it.next() {
                Some(c) => c,
                None => return None,
            };
            let ch2 = match it.next() {
                Some(c) => c,
                None => return None,
            };

            let thismod = match (ch1, ch2) {
                ('N', 'F') => Mods::NoFail,
                ('E', 'Z') => Mods::Easy,
                ('T', 'D') | ('N', 'V') => Mods::TouchDevice,
                ('H', 'D') => Mods::Hidden,
                ('H', 'R') => Mods::HardRock,
                ('S', 'D') => Mods::SuddenDeath,
                ('D', 'T') => Mods::DoubleTime,
                ('R', 'X') | ('R', 'L') => Mods::Relax,
                ('H', 'T') => Mods::HalfTime,
                ('N', 'C') => Mods::Nightcore,
                ('F', 'L') => Mods::Flashlight,
                ('A', 'U') => Mods::Autoplay,
                ('S', 'O') => Mods::SpunOut,
                ('A', 'P') => Mods::Relax2,
                ('P', 'F') => Mods::Perfect,
                ('4', 'K') => Mods::Key4,
                ('5', 'K') => Mods::Key5,
                ('6', 'K') => Mods::Key6,
                ('7', 'K') => Mods::Key7,
                ('8', 'K') => Mods::Key8,
                ('F', 'I') => Mods::FadeIn,
                ('R', 'D') => Mods::Random,
                ('C', 'M') => Mods::LastMod,
                ('T', 'P') => Mods::TargetPractice,
                ('9', 'K') => Mods::Key9,
                ('1', 'K') => Mods::Key1,
                ('3', 'K') => Mods::Key3,
                ('2', 'K') => Mods::Key2,
                _ => return None,
            };
            mods |= thismod;

            if it.peek().is_none() {
                break;
            }

            for ce in delim.chars() {
                match it.next() {
                    Some(c) if c == ce => {}
                    _ => return None,
                }
            }
        }

        Some(mods)
    }
}

/// Integer enumeration of the user's permission
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum UserPermission {
    None = 0,
    Normal = 1,
    Moderator = 2,
    Supporter = 4,
    Friend = 8,
    Peppy = 16,
    WorldCupStaff = 32,
}

impl BitOr for UserPermission {
    type Output = u8;
    fn bitor(self, other: Self) -> Self::Output {
        self as Self::Output | other as Self::Output
    }
}

/// Integer enumeration of the ranked statuses.
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum RankedStatus {
    Unknown,
    Unsubmitted,
    Unranked,
    Unused,
    Ranked,
    Approved,
    Qualified,
    Loved,
}

/// Rank grades
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum Grade {
    SS,
    SH,
    SSH,
    S,
    A,
    B,
    C,
    D,
    F,
    None,
}
