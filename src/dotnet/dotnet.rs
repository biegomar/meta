use crate::types::App;
use crate::types::subcommand;

pub fn sub() -> App {
    subcommand("dotnet")
        .about("Emulates dotnet, the package manager for .NET Core.")
}
