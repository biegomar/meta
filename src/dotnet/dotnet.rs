use clap::Arg;
use crate::types::App;
use crate::types::subcommand;

pub fn sub() -> App {
    subcommand("dotnet")
        .arg(Arg::with_name("new")
            .short("n")
            .long("new")
            .value_name("NAME")
            .help("Creates a new .NET Core project.")
            .takes_value(true))
        .about("Emulates dotnet, the package manager for .NET Core.")
}
