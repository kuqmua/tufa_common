#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]
#![feature(async_fn_in_trait)]

pub mod client;
pub mod common;
pub mod global_variables;
pub mod repositories_types;
pub mod server;

pub mod dev;

check_specific_dependency_version_usage::check_specific_dependency_version_usage!(tufa_server);
