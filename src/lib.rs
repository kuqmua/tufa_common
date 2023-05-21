#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#![feature(async_fn_in_trait)]

pub mod global_variables;
pub mod repositories_types;
pub mod client;
pub mod common;
pub mod server;

pub mod dev;
