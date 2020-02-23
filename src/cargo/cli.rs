use crate::types::App;
use crate::types::subcommand;
use crate::cargo::commands::new;

pub fn sub() -> App {
    subcommand("rust")
        .about("Emulates cargo, the package manager for Rust.")
        .subcommands(vec![new::sub()])
}
