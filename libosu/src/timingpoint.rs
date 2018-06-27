// A struct representing a _precise_ location in time, either by absolute timestamp (milliseconds),
// or a tuple (t, m, d, i) where _t_ is the TimingPoint that it's relative to, _m_ is the measure
// number from within this timing section, _d_ is a value representing the meter (for example, 0 =
// 1/1 meter, 1 = 1/2 meter, 3 = 1/4 meter, etc.), and _i_ is the index from the start of the measure.
pub enum TimeLocation<'map> {
    Absolute(u32),
    Relative(&'map TimingPoint, u32, u32),
}

pub struct TimingPoint {
    time: u32,
    mpb: f64,
    inherit: bool,
}
