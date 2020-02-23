use crate::types::App;
use crate::cargo;
use crate::dotnet;
use crate::elixir;

pub fn list() -> Vec<App> {
    vec! [
            cargo::sub(),
            dotnet::sub(),
            elixir::sub(),
    ]
}