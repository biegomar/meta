use crate::types::App;
use crate::types::subcommand;

pub fn sub() -> App {
    subcommand("rust")
        .about("Emulates cargo, the package manager for Rust.")
}
