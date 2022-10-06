use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::get_bunyan_where_was::GetBunyanWhereWas;
use crate::traits::get_json_where_was::GetJsonWhereWas;
use crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;

pub trait GetLogWhereWas {
    fn get_log_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        separation_by_line: bool,
        error: String,
    ) -> String;
}

impl<T> GetLogWhereWas for T
where
    T: GetWhereWasOneOrMany + GetJsonWhereWas + GetBunyanWhereWas,
{
    fn get_log_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        separation_by_line: bool,
        error: String,
    ) -> String {
        match separation_by_line {
            true => self.get_json_where_was(source_place_type, git_info, error),
            false => self.get_bunyan_where_was(source_place_type, git_info, error),
        }
    }
}
