//todo: make it only for Error impl
pub trait GitInfo {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn git_info(&self) -> String;
}
