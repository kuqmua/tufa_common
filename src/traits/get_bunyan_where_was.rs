use crate::config::source_place_type::SourcePlaceType;
use crate::helpers::git::git_info::GitInformation;
use crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;
use crate::where_was::WhereWasOneOrMany;

pub trait GetBunyanWhereWas {
    fn get_bunyan_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        separation_by_line: bool,
    ) -> String;
}

impl<T> GetBunyanWhereWas for T
where
    T: GetWhereWasOneOrMany,
{
    fn get_bunyan_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
        separation_by_line: bool,
    ) -> String {
        match separation_by_line {
            true => match self.get_where_was_one_or_many() {
                WhereWasOneOrMany::One(where_was_with_addition) => {
                    where_was_with_addition.where_was.file_line_column()
                }
                WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                    let mut formatted_into_string_vec = vec_where_was_with_addition
                        .iter()
                        .rev()
                        .map(|where_was_with_addition| {
                            format!(
                                "\n{}",
                                where_was_with_addition
                                    .get_file_line_column(source_place_type, git_info)
                            )
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
            },
            false => match self.get_where_was_one_or_many() {
                WhereWasOneOrMany::One(where_was_with_addition) => {
                    where_was_with_addition.where_was.file_line_column()
                }
                WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                    let mut formatted_into_string_vec = vec_where_was_with_addition
                        .iter()
                        .rev()
                        .map(|where_was_with_addition| {
                            format!(
                                "{}, ",
                                where_was_with_addition
                                    .get_file_line_column(source_place_type, git_info)
                            )
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
            },
        }
    }
}
