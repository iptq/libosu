use std::ops::BitOr;
use num_derive::FromPrimitive;

/// Integer enumeration of the game's game modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[allow(missing_docs)]
pub enum Mode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

/// Mod listing with their respective bitwise representation.
///
/// This list is ripped directly from the [osu! wiki](https://github.com/ppy/osu-api/wiki).
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Mods {
    None = 0,
    NoFail = 1,
    Easy = 2,
    NoVideo = 4, // Not used anymore, but can be found on old plays like Mesita on b/78239
    Hidden = 8,
    HardRock = 16,
    SuddenDeath = 32,
    DoubleTime = 64,
    Relax = 128,
    HalfTime = 256,
    Nightcore = 512, // Only set along with DoubleTime. i.e: NC only gives 576
    Flashlight = 1024,
    Autoplay = 2048,
    SpunOut = 4096,
    Relax2 = 8192,   // Autopilot?
    Perfect = 16384, // Only set along with SuddenDeath. i.e: PF only gives 16416
    Key4 = 32768,
    Key5 = 65536,
    Key6 = 131072,
    Key7 = 262144,
    Key8 = 524288,
    KeyMod = 1015808,
    FadeIn = 1048576,
    Random = 2097152,
    LastMod = 4194304,
    FreeModAllowed = 2077883,
    Key9 = 16777216,
    Key10 = 33554432,
    Key1 = 67108864,
    Key3 = 134217728,
    Key2 = 268435456,
}

impl BitOr for Mods {
    type Output = u32;
    fn bitor(self, other: Self) -> Self::Output {
        return self as u32 | other as u32;
    }
}

/// Integer enumeration of the user's permission
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        return self as Self::Output | other as Self::Output;
    }
}

/// Integer enumeration of the ranked statuses.
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
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
