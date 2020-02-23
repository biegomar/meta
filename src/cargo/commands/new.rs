use crate::types::App;
use crate::types::subcommand;

pub fn sub() -> App {
    subcommand("new")
        .about("Creates a new Rust project.")
}
