// use crate::project_constants::PROJECT_NAME;
use crate::helpers::git::git_info::GitInformationWrapper;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_INFO: GitInformationWrapper =
        GitInformationWrapper::init("../", "tufa_common");
}
