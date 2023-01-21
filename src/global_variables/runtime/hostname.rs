//todo - its a duplication, not actually a good decision. think about how make it better
pub static HOSTNAME: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| format!("{:?}", gethostname::gethostname()));
