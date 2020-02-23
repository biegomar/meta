use crate::types::App;
use crate::cargo;
use crate::dotnet;

pub fn list() -> Vec<App> {
    vec! [
            cargo::sub(),
            dotnet::sub(),
    ]
}