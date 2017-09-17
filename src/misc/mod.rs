//! This module contains a few constants and utilities that are either shared by other modules or
//! that do not naturally belong to any other module.

pub const VERSION_STR: &str = option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?");

pub mod banner;
