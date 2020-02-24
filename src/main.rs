mod all_commands;
mod cargo;
mod dotnet;
mod elixir;
mod types;

use types::App;

fn main() {
    let _matches = App::new("meta")
        .version("0.1.0")
        .author("Marc Biegota <marc.biegota@gmail.com>")
        .about("Meta, a CLI for many. Built with Rust.")
        .subcommands(all_commands::list())
        .get_matches();

    if let Some(matches) = _matches.subcommand_matches("dotnet") {
        if matches.is_present("new") {
            println!("Setting up C# project...");
        }
    }

    if let Some(matches) = _matches.subcommand_matches("rust") {
        if matches.is_present("new") {
            println!("Setting up Rust project...");
        }
    }

    if let Some(matches) = _matches.subcommand_matches("elixir") {
        if matches.is_present("new") {
            println!("Setting up Elixir project...");
        }
    }
}
