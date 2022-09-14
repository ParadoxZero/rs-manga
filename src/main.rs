use std::env;

use rs_manga::process_comandline;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    process_comandline(args);
}
