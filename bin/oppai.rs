use std::fs::File;
use std::io;
use std::path::PathBuf;

use anyhow::{bail, Result};
use libosu::prelude::{Beatmap, DiffCalc, Mods, PPCalcParams};
use serde_json::json;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "m", long = "mods", parse(try_from_str = parse_mods))]
    mods: Option<Mods>,
    file: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let beatmap = {
        let file = File::open(opts.file)?;
        Beatmap::parse(file)?
    };

    let mods = opts.mods.unwrap_or(Mods::None);

    let diff_calc = DiffCalc::new(&beatmap);
    let diff = diff_calc.calc(mods, None)?;

    let value = json!({
        "beatmap": {
            "artist": beatmap.artist,
            "title": beatmap.title,
            "mapper": beatmap.creator,
            "diffname": beatmap.difficulty_name,
        },
        "diff": diff,
    });

    let stdout = io::stdout();
    serde_json::to_writer_pretty(stdout, &value)?;
    Ok(())
}

// try parsing with both "" and ","
fn parse_mods(s: impl AsRef<str>) -> Result<Mods> {
    let s = s.as_ref();
    if let Some(v) = Mods::parse_from_str(s, "") {
        return Ok(v);
    }
    if let Some(v) = Mods::parse_from_str(s, ",") {
        return Ok(v);
    }
    bail!("could not parse mods")
}
