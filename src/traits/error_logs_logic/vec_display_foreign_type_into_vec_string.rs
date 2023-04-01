pub trait VecDisplayForeignTypeIntoVecString {
    fn vec_display_foreign_type_into_vec_string(self) -> Vec<String>;
}

impl<VecElementGeneric> VecDisplayForeignTypeIntoVecString for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_display_foreign_type_into_vec_string(self) -> Vec<String> {
        self.into_iter().map(|i| i.display_foreign_type()).collect()
    }
}
