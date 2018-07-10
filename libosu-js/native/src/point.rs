use libosu;

type OsuPoint = libosu::Point<i32>;

declare_types!{
    pub class Point for OsuPoint {
        init(_) {
            panic!("shiet");
        }
    }
}
