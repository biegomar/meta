use crate::types::App;
use crate::cargo;

pub fn list() -> Vec<App> {
    vec! [
            cargo::sub(),
    ]
}