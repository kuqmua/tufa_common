use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::get_bunyan_where_was::GetBunyanWhereWas;
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
    Self: GetBunyanWhereWas,
{
    fn get_bunyan_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        is_bunyan_separated_by_line_enabled: bool,
    ) -> String {
        let separator = match is_bunyan_separated_by_line_enabled {
            true => "\n",
            false => ", ",
        };
        match source_place_type {
            SourcePlaceType::Source => format!(
                "{}{}{}",
                where_was.file_line_column(),
                separator,
                self.get_bunyan_where_was_separated(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            SourcePlaceType::Github => format!(
                "{}{}{}",
                where_was.github_file_line_column(git_info),
                separator,
                self.get_bunyan_where_was_separated(
                    source_place_type,
                    git_info,
                    is_bunyan_separated_by_line_enabled
                )
            ),
            SourcePlaceType::None => self.get_bunyan_where_was_separated(
                source_place_type,
                git_info,
                is_bunyan_separated_by_line_enabled,
            ),
        }
    }
}
