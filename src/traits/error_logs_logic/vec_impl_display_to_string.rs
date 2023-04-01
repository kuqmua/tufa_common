pub trait VecImplDisplayToString {
    fn vec_impl_display_to_string(&self) -> String;
}

impl<VecElementGeneric> VecImplDisplayToString for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn vec_impl_display_to_string(&self) -> String {
        crate::traits::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            String::from(""),
            |mut acc, vec_element| {
                acc.push_str(&format!(" {vec_element}\n"));
                acc
            },
        ))
    }
}
