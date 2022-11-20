use crate::common::git::git_info::GitInformation;
use crate::common::where_was::WhereWasOriginOrWrapper;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_where_was_one_or_many::GetWhereWasOriginOrWrapper;
use crate::traits::where_was_trait::WhereWasTrait;

pub trait GetBunyanWhereWas {
    fn get_bunyan_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        error: String,
    ) -> String;
}

impl<T> GetBunyanWhereWas for T
where
    T: GetWhereWasOriginOrWrapper,
{
    fn get_bunyan_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        error: String,
    ) -> String {
        match self.get_where_was_one_or_many() {
            WhereWasOriginOrWrapper::One(where_was_with_addition) => match source_place_type {
                SourcePlaceType::Source => where_was_with_addition.where_was.file_line_column(),
                SourcePlaceType::Github => where_was_with_addition
                    .where_was
                    .github_file_line_column(git_info),
                SourcePlaceType::None => String::from(""), //todo - is it wrong?
            },
            WhereWasOriginOrWrapper::Many(vec_where_was_with_addition) => {
                let mut formatted_into_string_vec = vec_where_was_with_addition
                    .iter()
                    .enumerate()
                    // .rev()
                    .map(|(number, where_was_with_addition)| match number == 0 {
                        true => format!(
                            "{} {}",
                            where_was_with_addition
                                .get_file_line_column(source_place_type, git_info),
                            error
                        ),
                        false => format!(
                            "\n{}",
                            where_was_with_addition
                                .get_file_line_column(source_place_type, git_info)
                        ),
                    }) //maybe here use \n
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                if !formatted_into_string_vec.is_empty() {
                    formatted_into_string_vec.pop();
                    formatted_into_string_vec.pop();
                }
                formatted_into_string_vec
            }
            WhereWasOriginOrWrapper::None => String::from(""), //todo - not a good decision
        }
    }
}
