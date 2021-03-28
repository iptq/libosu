use std::fs::File;
use std::io;
use std::path::PathBuf;

use anyhow::{bail, Result};
use libosu::prelude::{
    calculate_ppv2, Beatmap, DiffCalc, HitObjectKind, Mode, Mods, PPCalcParams, ScoreVersion,
};
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

    let nobjects = beatmap.hit_objects.len() as u32;
    let mut ncircles = 0;
    let mut nsliders = 0;
    for obj in beatmap.hit_objects.iter() {
        match obj.kind {
            HitObjectKind::Circle => ncircles += 1,
            HitObjectKind::Slider(_) => nsliders += 1,
            _ => {}
        }
    }

    let max_combo = beatmap.max_combo();
    let params = PPCalcParams {
        combo: max_combo,
        n300: nobjects,
        n100: 0,
        n50: 0,
        nmiss: 0,
        mode: Mode::Osu,
        mods,
        score_version: ScoreVersion::V1,
    };
    let pp = calculate_ppv2(
        diff.aim_stars,
        diff.speed_stars,
        beatmap.difficulty.approach_rate as f64,
        beatmap.difficulty.overall_difficulty as f64,
        max_combo,
        nsliders,
        ncircles,
        nobjects,
        params,
    );

    let value = json!({
        "beatmap": {
            "artist": beatmap.artist,
            "title": beatmap.title,
            "mapper": beatmap.creator,
            "diffname": beatmap.difficulty_name,
            "max_combo": max_combo,
        },
        "diff": diff,
        "pp": pp,
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
