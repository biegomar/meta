use crate::types::App;
use crate::cargo;
use crate::dotnet::cli;
use crate::elixir;

pub fn list() -> Vec<App> {
    vec! [
        cargo::sub(),
        cli::sub(),
        elixir::sub(),
    ]
}