use std::fs::File;
use std::io::{self, Cursor, Read, Write};
use std::path::Path;

use anyhow::Result;
use libosu::{
    enums::{Mode, Mods},
    replay::{Buttons, Replay, ReplayActionData},
};

#[cfg(feature = "replay-data")]
fn compare_action_data(replay: &Replay, replay2: &Replay) -> Result<()> {
    let action_data = replay.parse_action_data()?;
    let action_data2 = replay2.parse_action_data()?;

    assert_eq!(action_data.frames.len(), action_data2.frames.len());
    assert_eq!(action_data.rng_seed, action_data2.rng_seed);

    for (a, b) in action_data.frames.iter().zip(action_data2.frames.iter()) {
        assert_eq!(a.time, b.time);
        assert!((a.x - b.x).abs() < 0.001);
        assert!((a.y - b.y).abs() < 0.001);
        assert_eq!(a.buttons, b.buttons);
    }
    Ok(())
}

#[test]
fn test_replay_writer() -> Result<()> {
    test_replay_writer_with("tests/files/replay-osu_2058788_3017707256.osr")?;
    test_replay_writer_with("tests/files/replay_with_life.osr")?;
    Ok(())
}

fn test_replay_writer_with(path: impl AsRef<Path>) -> Result<()> {
    let mut osr = File::open(path.as_ref())?;
    let mut contents = Vec::new();
    osr.read_to_end(&mut contents)?;

    let mut curs = Cursor::new(&contents);
    let replay = Replay::parse(&mut curs)?;

    // lzma encoded data will be different, so we'll just re-parse and check contents
    // not the most ideal since this assumes the parsing is correct
    let mut contents2 = Vec::new();
    replay.write(&mut contents2)?;

    let mut curs2 = Cursor::new(&contents2);
    let replay2 = Replay::parse(&mut curs2)?;

    assert_eq!(replay.count_300, replay2.count_300);
    assert_eq!(replay.count_100, replay2.count_100);
    assert_eq!(replay.count_50, replay2.count_50);
    assert_eq!(replay.count_geki, replay2.count_geki);
    assert_eq!(replay.life_graph, replay2.life_graph);

    #[cfg(feature = "replay-data")]
    compare_action_data(&replay, &replay2)?;

    Ok(())
}

#[cfg(feature = "replay-data")]
#[test]
fn test_replay_action_update() -> Result<()> {
    let mut osr = File::open("tests/files/replay-osu_2058788_3017707256.osr").unwrap();
    let replay = Replay::parse(&mut osr).unwrap();
    let actions = replay.parse_action_data().unwrap();

    let mut replay2 = replay.clone();
    replay2.update_action_data(&actions).unwrap();

    compare_action_data(&replay, &replay2)?;
    Ok(())
}

#[test]
fn test_replay_parse_header() -> Result<()> {
    let mut osr = File::open("tests/files/replay-osu_2058788_3017707256.osr")?;
    let header = Replay::parse(&mut osr)?;

    assert_eq!(header.mode, Mode::Osu);
    assert_eq!(header.version, 20200304);
    assert_eq!(
        header.beatmap_hash,
        "4190b795c2847f9eae06a0651493d6e2".to_string()
    );
    assert_eq!(header.player_username, "FGSky".to_string());
    assert_eq!(
        header.replay_hash,
        "e8983dbdb53360e5d19cbe5de5de49a7".to_string()
    );

    assert_eq!(header.count_300, 330);
    assert_eq!(header.count_100, 24);
    assert_eq!(header.count_50, 0);
    assert_eq!(header.count_geki, 87);
    assert_eq!(header.count_katu, 21);
    assert_eq!(header.count_miss, 2);

    assert_eq!(header.score, 7756117);
    assert_eq!(header.max_combo, 527);
    assert_eq!(header.perfect, false);
    assert_eq!(
        header.mods,
        Mods::Flashlight | Mods::Hidden | Mods::DoubleTime | Mods::HardRock
    );

    Ok(())
}

#[cfg(feature = "replay-data")]
#[test]
fn test_seed() -> Result<()> {
    let mut osr = io::BufReader::new(File::open("tests/files/replay-osu_2058788_3017707256.osr")?);
    let replay = Replay::parse(&mut osr)?;

    let actions = replay.parse_action_data()?;
    assert_eq!(actions.rng_seed, Some(16516643));
    Ok(())
}

#[test]
fn test_parse_after_actions() -> Result<()> {
    {
        let mut osr =
            File::open("tests/files/ - nekodex - new beginnings [tutorial] (2020-12-16) Osu.osr")?;
        let replay = Replay::parse(&mut osr)?;
        assert_eq!(replay.score_id, None);
        assert_eq!(replay.target_practice_total_accuracy, None);
    }

    {
        let mut osr = File::open("tests/files/replay-osu_2058788_3017707256.osr")?;
        let replay = Replay::parse(&mut osr)?;
        assert_eq!(replay.score_id, Some(3017707256));
        assert_eq!(replay.target_practice_total_accuracy, None);
    }

    Ok(())
}

#[cfg(feature = "replay-data")]
fn lzma_encode(data: &[u8]) -> Result<Vec<u8>> {
    use xz2::{
        stream::{LzmaOptions, Stream},
        write::XzEncoder,
    };
    let mut buf = Vec::new();
    let opts = LzmaOptions::new_preset(0)?;
    let stream = Stream::new_lzma_encoder(&opts)?;
    {
        let mut xz = XzEncoder::new_stream(&mut buf, stream);
        xz.write_all(data)?;
    }
    Ok(buf)
}

#[cfg(feature = "replay-data")]
#[test]
fn test_replay_action_parser() -> Result<()> {
    let actions_text = "1|32.1|300.734|0,32|500.5123|0|10,-12345|0|0|734243";
    let data = lzma_encode(actions_text.as_bytes())?;
    let actions_reader = Cursor::new(data);
    let action_data = ReplayActionData::parse(actions_reader)?;
    let actions = &action_data.frames;

    assert_eq!(actions.len(), 2);

    assert_eq!(actions[0].time, 1);
    assert_eq!(actions[0].x, 32.1);
    assert_eq!(actions[0].y, 300.734);
    assert_eq!(actions[0].buttons, Buttons::empty());

    assert_eq!(actions[1].time, 32);
    assert_eq!(actions[1].x, 500.5123);
    assert_eq!(actions[1].y, 0.0);
    assert_eq!(actions[1].buttons, Buttons::K2 | Buttons::M2);

    assert_eq!(action_data.rng_seed, Some(734243));
    Ok(())
}

#[test]
fn test_replay_parse() -> Result<()> {
    let mut osr = File::open("tests/files/replay-osu_1816113_2892542031.osr")?;
    let replay = Replay::parse(&mut osr)?;

    assert_eq!(replay.mode, Mode::Osu);
    assert_eq!(replay.version, 20190906);
    assert_eq!(
        replay.beatmap_hash,
        "edd35ab673c5f73029cc8eda6faefe00".to_string()
    );
    assert_eq!(replay.player_username, "Vaxei".to_string());
    assert_eq!(
        replay.replay_hash,
        "139c99f18fc78555cd8f30a963aadf0a".to_string()
    );

    assert_eq!(replay.count_300, 2977);
    assert_eq!(replay.count_100, 38);
    assert_eq!(replay.count_50, 0);
    assert_eq!(replay.count_geki, 605);
    assert_eq!(replay.count_katu, 30);
    assert_eq!(replay.count_miss, 0);

    assert_eq!(replay.score, 364_865_850);
    assert_eq!(replay.max_combo, 4078);
    assert_eq!(replay.perfect, false);
    assert_eq!(replay.mods, Mods::None);

    #[cfg(feature = "replay-data")]
    {
        let action_data = replay.parse_action_data().unwrap();
        assert_eq!(action_data.rng_seed, Some(7364804));
    }

    assert_eq!(replay.score_id, Some(2892542031));
    assert_eq!(replay.target_practice_total_accuracy, None);
    Ok(())
}
