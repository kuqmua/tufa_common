use crate::common::git::git_info::GitInformation;
use crate::common::where_was::WhereWasOriginOrWrapper;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_where_was_one_or_many::GetWhereWasOriginOrWrapper;

pub trait GetBunyanWhereWas {
    fn get_bunyan_where_was(&self, source_place_type: &SourcePlaceType, error: String) -> String;
}

impl<SelfGeneric> GetBunyanWhereWas for SelfGeneric
where
    SelfGeneric: GetWhereWasOriginOrWrapper,
{
    fn get_bunyan_where_was(&self, source_place_type: &SourcePlaceType, error: String) -> String {
        match self.get_where_was_one_or_many() {
            WhereWasOriginOrWrapper::One(where_was_with_addition) => format!(
                "{} {}",
                where_was_with_addition.get_file_line_column(source_place_type),
                error
            ),
            WhereWasOriginOrWrapper::Many(vec_where_was_with_addition) => {
                let formatted_into_string_vec = vec_where_was_with_addition
                    .iter()
                    .enumerate()
                    // .rev()
                    .map(|(number, where_was_with_addition)| match number == 0 {
                        true => format!(
                            "{} {}",
                            where_was_with_addition.get_file_line_column(source_place_type),
                            error
                        ),
                        false => format!(
                            "\n{}",
                            where_was_with_addition.get_file_line_column(source_place_type)
                        ),
                    }) //maybe here use \n
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                formatted_into_string_vec
            }
            WhereWasOriginOrWrapper::None => error, //todo - not a good decision
        }
    }
}
