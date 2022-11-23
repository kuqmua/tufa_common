use crate::common::git::git_info::GitInformation;
use crate::common::where_was::GitInfoForWhereWas;
use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_git_info::GetGitInfo;
use crate::traits::get_log_where_was::GetLogWhereWas;
use crate::traits::where_was_trait::WhereWasTrait;

pub trait GetLogWithAdditionalWhereWas<T> {
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInfoForWhereWas,
        error: String,
        is_tracing_enabled: bool,
    ) -> String;
}

impl<T> GetLogWithAdditionalWhereWas<Self> for T
where
    Self: GetLogWhereWas, // + GetGitInfo,
{
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInfoForWhereWas,
        error: String,
        is_tracing_enabled: bool,
    ) -> String {
        match (is_tracing_enabled, source_place_type) {
            (false, SourcePlaceType::Source) => format!(
                "{}\n{}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.file_line_column(),
            ),
            (false, SourcePlaceType::Github) => {
                let one =
                    self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error);
                let two = where_was.github_file_line_column(git_info);
                println!("here1 {}", one);
                println!("here2 {}", two);
                format!("{}\n{}", one, two)
            }
            (false, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error)
            }
            (true, SourcePlaceType::Source) => format!(
                "{}, {}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.file_line_column(),
            ),
            (true, SourcePlaceType::Github) => {
                println!("here2");
                format!(
                    "{}, {}",
                    self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                    where_was.github_file_line_column(git_info),
                )
            }
            (true, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error)
            }
        }
    }
}
