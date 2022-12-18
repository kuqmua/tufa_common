pub static GIT_INFO_WITHOUT_LIFETIMES: once_cell::sync::Lazy<
    crate::common::git::git_info::GitInformationWithoutLifetimes,
> = once_cell::sync::Lazy::new(|| {
    crate::global_variables::compile_time::git_info::GIT_INFO.get_runtime_version()
});

// pub static GIT_INFO_WITHOUT_LIFETIMES_UNDER_ARC: once_cell::sync::Lazy<
//     std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes>,
// > = once_cell::sync::Lazy::new(|| {
//     std::sync::Arc::new(
//         crate::global_variables::compile_time::git_info::GIT_INFO.get_runtime_version(),
//     )
// });
