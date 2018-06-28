extern crate libosu;

use std::fs::File;
use std::io::Read;

use libosu::{Beatmap, Deserializer, Serializer};

macro_rules! test_parser {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");
                let beatmap = Beatmap::deserialize(contents).expect("couldn't parse");

                // stage 1
                let stage1 = beatmap.serialize().expect("couldn't serialize");
                
                // ok parse again
                let beatmap2 = Beatmap::deserialize(stage1.clone()).expect("couldn't parse");

                // stage 2
                let stage2 = beatmap.serialize().expect("couldn't serialize");

                assert_eq!(stage1, stage2);
            }
        )*
    };
}

test_parser!{
    test_parser_774965: 774965,
    test_parser_804683: 804683,
}
