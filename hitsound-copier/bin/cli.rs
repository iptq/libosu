extern crate hitsound_copier;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
