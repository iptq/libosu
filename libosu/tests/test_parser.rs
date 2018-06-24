extern crate libosu;

use std::fs::File;
use std::io::Read;

use libosu::{Beatmap, OszParser};

macro_rules! test_parser {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");
                let beatmap = Beatmap::parse(&contents).expect("couldn't parse");
            }
        )*
    };
}

test_parser!{
    test_parser_774965: 774965,
}
