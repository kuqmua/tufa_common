pub trait VecDisplayForeignTypeIntoVecStaticStr {
    fn vec_display_foreign_type_into_vec_static_str(self) -> Vec<&'static str>;
}

impl<VecElementGeneric> VecDisplayForeignTypeIntoVecStaticStr for Vec<VecElementGeneric>
where
    VecElementGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn vec_display_foreign_type_into_vec_static_str(self) -> Vec<&'static str> {
        self.into_iter().map(|i| i.display_foreign_type()).collect()
    }
}
