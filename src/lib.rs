#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod client;
pub mod common;
pub mod global_variables;
pub mod repositories_types;
pub mod server;
pub mod proc_macros;

pub mod dev;
