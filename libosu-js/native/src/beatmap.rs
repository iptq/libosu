use std::marker::PhantomData;

use libosu::Beatmap as OsuBeatmap;
use neon::{self, macro_internal::runtime::raw::Local};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Beatmap(Local);

impl neon::mem::Managed for Beatmap {
    fn to_raw(self) -> Local {
        let Beatmap(raw) = self;
        raw
    }
    fn from_raw(raw: Local) -> Self {
        Beatmap(raw)
    }
}
