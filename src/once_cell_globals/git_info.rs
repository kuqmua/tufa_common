// use crate::project_constants::PROJECT_NAME;
use crate::common::git::git_info_wrapper::GitInformationWrapper;
use once_cell::sync::Lazy;

pub static GIT_INFO: Lazy<GitInformationWrapper> =
    Lazy::new(|| GitInformationWrapper::init("../", "tufa_common"));
