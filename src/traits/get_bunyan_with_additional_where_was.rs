use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::get_log_where_was::GetLogWhereWas;
use crate::where_was::WhereWas;

pub trait GetBunyanWithAdditionalWhereWas<T> {
    fn get_bunyan_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        is_bunyan_separated_by_line_enabled: bool,
    ) -> String;
}

impl<T> GetBunyanWithAdditionalWhereWas<Self> for T
where
    Self: GetLogWhereWas,
{
    fn get_bunyan_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        is_bunyan_separated_by_line_enabled: bool,
    ) -> String {
        match (is_bunyan_separated_by_line_enabled, source_place_type) {
            (true, SourcePlaceType::Source) => format!(
                "\n{}{}",
                where_was.file_line_column(),
                self.get_log_where_was(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            (true, SourcePlaceType::Github) => format!(
                "\n{}{}",
                where_was.github_file_line_column(git_info),
                self.get_log_where_was(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            (true, SourcePlaceType::None) => self.get_log_where_was(
                source_place_type,
                git_info,
                is_bunyan_separated_by_line_enabled,
            ),
            (false, SourcePlaceType::Source) => format!(
                "{}, {}",
                where_was.file_line_column(),
                self.get_log_where_was(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            (false, SourcePlaceType::Github) => format!(
                "{}, {}",
                where_was.github_file_line_column(git_info),
                self.get_log_where_was(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            (false, SourcePlaceType::None) => self.get_log_where_was(
                source_place_type,
                git_info,
                is_bunyan_separated_by_line_enabled,
            ),
        }
    }
}
