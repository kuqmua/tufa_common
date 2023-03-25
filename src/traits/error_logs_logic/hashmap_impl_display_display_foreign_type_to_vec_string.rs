pub trait HashmapImplDisplayDisplayForeignTypeToVecString<HashMapKeyGeneric> {
    fn hashmap_impl_display_display_foreign_type_to_vec_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapImplDisplayDisplayForeignTypeToVecString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_impl_display_display_foreign_type_to_vec_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String> {
        self.into_iter()
            .map(|(k, v)| {
                (k, {
                    use crate::traits::display_foreign_type::DisplayForeignType;
                    v.display_foreign_type()
                })
            })
            .collect()
    }
}
