#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod config_mods;
pub mod global_variables;
pub mod json_example;
pub mod traits;

pub mod client;
pub mod common;
pub mod server;

pub mod dev;
pub mod dev_with_deserialize;
