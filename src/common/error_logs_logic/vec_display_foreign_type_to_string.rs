pub trait VecDisplayForeignTypeToString {
    fn vec_display_foreign_type_to_string(&self) -> String;
}

impl<VecElementGeneric> VecDisplayForeignTypeToString for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::common::display_foreign_type::DisplayForeignType,
{
    fn vec_display_foreign_type_to_string(&self) -> String {
        crate::common::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&format!(" {}\n", vec_element.display_foreign_type()));
                acc
            },
        ))
    }
}