use crate::common::where_was::GitInfoForWhereWas;

pub trait FileLineColumnTrait {
    fn file_line_column(&self) -> String;
    fn github_file_line_column(&self, git_info: &GitInfoForWhereWas) -> String; //theoretically can make it const fn
}
