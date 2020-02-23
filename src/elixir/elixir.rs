use crate::types::App;
use crate::types::subcommand;

pub fn sub() -> App {
    subcommand("elixir")
        .about("Emulates mix, the build tool for Elixir.")
}
