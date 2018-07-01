extern crate failure;
extern crate libosu;

#[cfg(feature = "gui")]
pub mod gui;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;
use libosu::*;

pub struct Options {
    pub src_file: PathBuf,
    pub dst_files: Vec<PathBuf>,
}

pub fn copy_hitsounds(opt: Options) -> Result<(), Error> {
    let mut f = File::open(opt.src_file)?;
    let src_beatmap;
    {
        let mut src = String::new();
        f.read_to_string(&mut src)?;
        src_beatmap = Beatmap::deserialize(src)?;
    }

    for filename in opt.dst_files {
        let mut f = File::open(filename)?;
        let dst_beatmap;
        {
            let mut src = String::new();
            f.read_to_string(&mut src)?;
            dst_beatmap = Beatmap::deserialize(src)?;
        }

        // copy the hitsounds over
        copy_hitsounds_single(&src_beatmap, &dst_beatmap)?;
    }
    Ok(())
}

pub fn copy_hitsounds_single(src: &Beatmap, dst: &Beatmap) -> Result<(), Error> {
    Ok(())
}
