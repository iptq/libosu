use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use libosu::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    copy_from: PathBuf,
    copy_to: PathBuf,
    output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut from_beatmap = {
        let mut input_file = File::open(&opt.copy_from)?;
        let mut contents = String::new();
        input_file.read_to_string(&mut contents)?;
        Beatmap::from_osz(&contents)?
    };

    // collect a list of hitsounds from the template beatmap
    #[derive(Debug)]
    struct HitsoundInfo {
        time: TimeLocation,
        sample_set: SampleSet,
        additions_set: SampleSet,
        additions: Additions,
    }

    let mut hitsounds = Vec::new();
    from_beatmap.hit_objects.sort_by_key(|ho| ho.start_time);
    for obj in from_beatmap.hit_objects.iter() {
        let start_time = obj.start_time.clone().as_milliseconds();
        match obj.kind {
            HitObjectKind::Circle => {}
            HitObjectKind::Slider {
                ref repeats,
                ref duration,
                ..
            } => {
                for i in 0..(repeats + 1) {
                    let time = start_time + (i * duration) as i32;
                    // hitsounds.push((time, obj.additions));
                }
            }
            HitObjectKind::Spinner { end_time } => {
                hitsounds.push(HitsoundInfo {
                    time: end_time,
                    sample_set: SampleSet::None,
                    additions_set: SampleSet::None,
                    additions: Additions::empty(),
                });
            }
        }
    }
    println!("hitsounds: {:?}", hitsounds);

    let mut to_beatmap = {
        let mut input_file = File::open(&opt.copy_to)?;
        let mut contents = String::new();
        input_file.read_to_string(&mut contents)?;
        Beatmap::from_osz(&contents)?
    };

    let mut output_file = File::create(&opt.output)?;
    output_file.write_all(to_beatmap.as_osz()?.as_bytes())?;
    Ok(())
}
