use crate::common::where_was::WhereWas;
use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_git_info::GetGitInfo;
use crate::traits::where_was_methods::WhereWasMethods;

pub trait GetLogWithAdditionalWhereWas<SelfGeneric> {
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        error: String,
        log_type: LogType,
    ) -> String;
}

impl<SelfGeneric> GetLogWithAdditionalWhereWas<Self> for SelfGeneric
where
    Self: crate::traits::get_log_where_was::GetLogWhereWas, // + GetGitInfo,
{
    fn get_log_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        error: String,
        log_type: LogType,
    ) -> String {
        match (log_type.clone(), source_place_type) {
            (LogType::Tracing, SourcePlaceType::Source) => format!(
                "{}, {}",
                self.get_log_where_was(source_place_type, log_type, error),
                where_was.file_line_column(),
            ),
            (LogType::Tracing, SourcePlaceType::Github) => format!(
                "{}, {}",
                self.get_log_where_was(source_place_type, log_type, error),
                where_was.github_file_line_column(&where_was.git_info),
            ),
            (LogType::Tracing, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, log_type, error)
            }
            (LogType::Stack, SourcePlaceType::Source) => format!(
                "{}\n{}",
                self.get_log_where_was(source_place_type, log_type, error),
                where_was.file_line_column(),
            ),
            (LogType::Stack, SourcePlaceType::Github) => format!(
                "{}\n{}",
                self.get_log_where_was(source_place_type, log_type, error),
                where_was.github_file_line_column(&where_was.git_info)
            ),
            (LogType::Stack, SourcePlaceType::None) => {
                self.get_log_where_was(source_place_type, log_type, error)
            }
            (LogType::None, SourcePlaceType::Source) => String::from(""),
            (LogType::None, SourcePlaceType::Github) => String::from(""),
            (LogType::None, SourcePlaceType::None) => String::from(""),
        }
    }
}
