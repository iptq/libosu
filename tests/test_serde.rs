extern crate libosu;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::io::{Read, Write};

use libosu::*;

macro_rules! test_serde {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");
                let beatmap = Beatmap::deserialize_osz(&contents).expect("couldn't parse");

                // stage 1
                let stage1 = beatmap.serialize_osz().expect("couldn't serialize");

                // let mut file = File::create(format!("tests/out/{}.stage1.osu", $id)).expect("couldn't open file");
                // file.write_all(stage1.as_bytes()).expect("couldn't write");
                // eprintln!("STAGE 2 --------------------");

                // ok parse again
                let beatmap1 = Beatmap::deserialize_osz(stage1.clone()).expect("couldn't parse");

                // stage 2
                let stage2 = beatmap1.serialize_osz().expect("couldn't serialize");

                println!("{}", stage2);
                println!("{:?}", beatmap);

                assert_eq!(stage1, stage2);
                // panic!();

                json!(beatmap).to_string();
            }
        )*
    };
}

test_serde!{
    test_parser_774965: 774965,
    test_parser_804683: 804683,
}
