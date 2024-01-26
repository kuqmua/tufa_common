pub trait VecDisplayToString {
    fn vec_display_to_string(&self) -> std::string::String;
}

impl<VecElementGeneric> VecDisplayToString for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn vec_display_to_string(&self) -> std::string::String {
        crate::common::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            std::string::String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&format!(" {vec_element}\n"));
                acc
            },
        ))
    }
}
