pub static GIT_INFO_WITHOUT_LIFETIMES: once_cell::sync::Lazy<
    crate::common::git::git_info::GitInformationWithoutLifetimes,
> = once_cell::sync::Lazy::new(|| {
    crate::global_variables::compile_time::git_info::GIT_INFO.get_runtime_version()
});
