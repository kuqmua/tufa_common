use crate::common::git::git_info::GitInformation;
use crate::common::where_was::WhereWas;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_log_where_was::GetLogWhereWas;
use crate::traits::where_was_trait::WhereWasTrait;

pub trait GetLogWithAdditionalWhereWas<T> {
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        error: String,
        is_tracing_enabled: bool,
    ) -> String;
}

impl<T> GetLogWithAdditionalWhereWas<Self> for T
where
    Self: GetLogWhereWas,
{
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        error: String,
        is_tracing_enabled: bool,
    ) -> String {
        match (is_tracing_enabled, source_place_type) {
            (false, SourcePlaceType::Source) => format!(
                "{}\n{}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.file_line_column(),
            ),
            (false, SourcePlaceType::Github) => format!(
                "{}\n{}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.github_file_line_column(git_info),
            ),
            (false, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error)
            }
            (true, SourcePlaceType::Source) => format!(
                "{}, {}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.file_line_column(),
            ),
            (true, SourcePlaceType::Github) => format!(
                "{}, {}",
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error),
                where_was.github_file_line_column(git_info),
            ),
            (true, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, git_info, is_tracing_enabled, error)
            }
        }
    }
}
