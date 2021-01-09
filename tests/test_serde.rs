use std::fs::File;
use std::io::Read;

use libosu::beatmap::Beatmap;

macro_rules! test_serde {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");

                let beatmap = Beatmap::from_osz(&contents).expect("couldn't parse");
                let reexported = beatmap.as_osz().expect("couldn't serialize");
                let beatmap2 = Beatmap::from_osz(&reexported).expect("couldn't parse");

                assert_eq!(beatmap, beatmap2);
            }
        )*
    };
}

test_serde! {
    test_parser_774965: 774965,
    test_parser_804683: 804683,
}
