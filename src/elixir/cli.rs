use crate::types::App;
use crate::types::subcommand;
use crate::elixir::commands::new;

pub fn sub() -> App {
    subcommand("elixir")
        .about("Emulates mix, the build tool for Elixir.")
        .subcommands(vec![new::sub()])
}
