pub trait VecDisplayForeignTypeToVecString {
    fn vec_display_foreign_type_to_vec_string(self) -> Vec<String>;
}

impl<VecElementGeneric> VecDisplayForeignTypeToVecString for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_display_foreign_type_to_vec_string(self) -> Vec<String> {
        self.into_iter()
            .map(|i| {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type()
            })
            .collect()
    }
}
