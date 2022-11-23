use crate::common::where_was::GitInfoForWhereWas;

pub trait WhereWasTrait {
    fn readable_time(&self, timezone: i32) -> String;
    fn file_line_column(&self) -> String;
    fn github_file_line_column(&self, git_info: &GitInfoForWhereWas) -> String;
}
