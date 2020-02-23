use crate::types::App;
use crate::cargo;
use crate::dotnet;
use crate::elixir;

pub fn list() -> Vec<App> {
    vec! [
        cargo::cli::sub(),
        dotnet::cli::sub(),
        elixir::sub(),
    ]
}