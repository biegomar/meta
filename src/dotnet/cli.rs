use crate::types::App;
use crate::types::subcommand;
use crate::dotnet::commands::new;


pub fn sub() -> App {
    subcommand("dotnet")
        .about("Emulates dotnet, the package manager for .NET Core.")
        .subcommands(vec![new::sub()])
}
