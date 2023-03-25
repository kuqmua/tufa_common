pub trait VecToStringDisplayForeignType {
    fn vec_to_string_display_foreign_type(&self) -> String;
}

impl<VecElementGeneric> VecToStringDisplayForeignType for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_to_string_display_foreign_type(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&vec_element.display_foreign_type());
                acc
            },
        ))
    }
}
