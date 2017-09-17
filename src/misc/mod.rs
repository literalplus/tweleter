//! This module contains a few constants and utilities that are either shared by other modules or
//! that do not naturally belong to any other module.

pub fn get_version_str() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?");
}

pub mod banner;
