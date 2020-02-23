mod all_commands;
mod types;

use clap;
use clap::{Arg};
use types::App;


fn main() {
    let _matches = App::new("meta")
        .version("0.1.0")
        .author("Marc Biegota <marc.biegota@gmail.com>")
        .about("Meta, a CLI for many. Build with Rust.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .subcommands(all_commands::list())
        .get_matches();

    let config = _matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
}
