//! Lightweight HTTP library

pub mod common;
pub mod server;

use common::CARGO_TOML;
use kern::meta::{init_name, init_version, name as get_name, version as get_version};

/// Get lhi version string
pub fn version() -> &'static str {
    match get_version() {
        "" => init_version(CARGO_TOML),
        version => version,
    }
}

/// Get lhi name string
pub fn name() -> &'static str {
    match get_name() {
        "" => init_name(CARGO_TOML),
        name => name,
    }
}

// TODO add tests
