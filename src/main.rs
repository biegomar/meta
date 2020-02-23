mod all_commands;
mod types;
mod cargo;
mod dotnet;
mod elixir;

use types::App;

fn main() {
    let _matches = App::new("meta")
        .version("0.1.0")
        .author("Marc Biegota <marc.biegota@gmail.com>")
        .about("Meta, a CLI for many. Build with Rust.")
        .subcommands(all_commands::list())
        .get_matches();
}
