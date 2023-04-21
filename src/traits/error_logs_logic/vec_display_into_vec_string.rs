pub trait VecDisplayIntoVecString {
    fn vec_display_into_vec_string(self) -> Vec<String>;
}

impl<VecElementGeneric> VecDisplayIntoVecString for Vec<VecElementGeneric>
where
    VecElementGeneric: std::fmt::Display,
{
    fn vec_display_into_vec_string(self) -> Vec<String> {
        self.into_iter()
            .map(|i| i.to_string())
            .collect()
    }
}
