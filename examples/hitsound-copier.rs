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

    // just to be sure
    from_beatmap.hit_objects.sort_by_key(|ho| ho.start_time);

    // collect a list of hitsounds from the template beatmap
    let mut hitsounds = collect_hitsounds(&from_beatmap);
    hitsounds.sort_by_key(|hs| hs.time);

    let mut to_beatmap = {
        let mut input_file = File::open(&opt.copy_to)?;
        let mut contents = String::new();
        input_file.read_to_string(&mut contents)?;
        Beatmap::from_osz(&contents)?
    };

    // write the hitsounds to the beatmap object
    write_hitsounds(&hitsounds, &mut to_beatmap);

    let mut output_file = File::create(&opt.output)?;
    output_file.write_all(to_beatmap.as_osz()?.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct HitsoundInfo {
    time: TimeLocation,
    sample_set: SampleSet,
    additions_set: SampleSet,
    additions: Additions,
}

fn collect_hitsounds(beatmap: &Beatmap) -> Vec<HitsoundInfo> {
    let mut hitsounds = Vec::new();
    for (ho, tp) in beatmap.double_iter() {
        let start_time = ho.start_time;
        let sample_set = if let SampleSet::None = ho.sample_info.sample_set {
            // default to the timing point's sample set
            tp.sample_set
        } else {
            ho.sample_info.sample_set
        };

        let additions_set = if let SampleSet::None = ho.sample_info.addition_set {
            // default to the regular sample set
            sample_set
        } else {
            ho.sample_info.addition_set
        };

        match &ho.kind {
            // circles get 1 hitsound
            HitObjectKind::Circle => {
                hitsounds.push(HitsoundInfo {
                    time: start_time,
                    sample_set,
                    additions_set,
                    additions: ho.additions,
                });
            }
            // sliders are a mess
            HitObjectKind::Slider {
                edge_additions,
                edge_samplesets,
                ..
            } => {
                let duration = beatmap.get_slider_duration(ho).unwrap();
                let mut time = ho.start_time.0 as f64;

                // add a hitsound for each slider repeat (called "edge")
                for (additions, (normal_set, addition_set)) in
                    edge_additions.iter().zip(edge_samplesets.iter())
                {
                    let edge_sample_set = if let SampleSet::None = normal_set {
                        // default to the hit object's sample set
                        sample_set
                    } else {
                        *normal_set
                    };
                    let edge_addition_set = if let SampleSet::None = addition_set {
                        // default to the edge sample set
                        edge_sample_set
                    } else {
                        *addition_set
                    };
                    hitsounds.push(HitsoundInfo {
                        time: TimeLocation(time as i32),
                        sample_set: edge_sample_set,
                        additions_set: edge_addition_set,
                        additions: *additions,
                    });
                    time += duration;
                }
            }
            // spinners get 1 hitsound at the end
            HitObjectKind::Spinner { end_time } => {
                hitsounds.push(HitsoundInfo {
                    time: *end_time,
                    sample_set,
                    additions_set,
                    additions: ho.additions,
                });
            }
        }
    }
    hitsounds
}

fn write_hitsounds(hitsounds: &Vec<HitsoundInfo>, beatmap: &mut Beatmap) {
    // generate a mapping of hitsound to hitobject
    let mut iter = beatmap.hit_objects.iter().enumerate().peekable();
    let mut index_map = Vec::new();
    let mut slider_map = Vec::new();
    'outer: for (hs_idx, hitsound) in hitsounds.iter().enumerate() {
        let (ho_idx, ho) = loop {
            let (ho_idx, ho) = match iter.peek() {
                Some((ho_idx, ho)) => (*ho_idx, ho),
                None => break 'outer,
            };

            let ho_end_time = beatmap.get_hitobject_end_time(ho);
            if ho_end_time >= hitsound.time {
                break (ho_idx, ho);
            }

            // advance hit-object iterator
            iter.next();
        };

        // this is probably a circle!
        if ho.start_time == hitsound.time {
            if let HitObjectKind::Circle = ho.kind {
                index_map.push((hs_idx, ho_idx));
            } else if let HitObjectKind::Slider { .. } = ho.kind {
                index_map.push((hs_idx, ho_idx));
            }
        } else if ho.start_time < hitsound.time {
            if let HitObjectKind::Spinner { end_time } = ho.kind {
                if end_time == hitsound.time {
                    index_map.push((hs_idx, ho_idx));
                }
            } else if let HitObjectKind::Slider { num_repeats, .. } = ho.kind {
                let time_diff = (hitsound.time.0 - ho.start_time.0) as f64;
                let duration = beatmap.get_slider_duration(ho).unwrap();
                let num_repeats_approx = time_diff / duration;
                let num_repeats_rounded = num_repeats_approx.round();
                if num_repeats_rounded as u32 > num_repeats {
                    continue;
                }
                let percent_diff = (num_repeats_rounded - num_repeats_approx).abs();
                if percent_diff < 0.05 {
                    let num_repeats = num_repeats_rounded as usize;
                    slider_map.push((hs_idx, ho_idx, num_repeats));
                }
            }
        }
    }

    for (hs_idx, ho_idx) in index_map {
        let hitsound = hitsounds.get(hs_idx).unwrap();
        let mut hit_object = beatmap.hit_objects.get_mut(ho_idx).unwrap();

        hit_object.additions = hitsound.additions;
        hit_object.sample_info.sample_set = hitsound.sample_set;
        hit_object.sample_info.addition_set = hitsound.additions_set;
    }

    for (hs_idx, ho_idx, e_idx) in slider_map {
        let hitsound = hitsounds.get(hs_idx).unwrap();
        let hit_object = beatmap.hit_objects.get_mut(ho_idx).unwrap();
        if let HitObjectKind::Slider {
            ref mut edge_additions,
            ref mut edge_samplesets,
            ..
        } = hit_object.kind
        {
            while edge_additions.len() <= e_idx {
                edge_additions.push(Additions::empty());
            }
            edge_additions[e_idx] = hitsound.additions;

            while edge_samplesets.len() <= e_idx {
                edge_samplesets.push((SampleSet::None, SampleSet::None));
            }
            edge_samplesets[e_idx] = (hitsound.sample_set, hitsound.additions_set);
        }
    }
}
