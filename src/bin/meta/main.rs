use clap;
use clap::{App};

fn main() {
    let _matches = App::new("meta")
        .version("0.1.0")
        .author("Marc Biegota <marc.biegota@gmail.com>")
        .about("Meta, a CLI for many. Build with Rust.")
        .get_matches();
}
