pub trait HashmapImplDisplayDisplayForeignTypeIntoHashmapImplDisplayString<HashMapKeyGeneric> {
    fn hashmap_impl_display_display_foreign_type_into_hashmap_impl_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapImplDisplayDisplayForeignTypeIntoHashmapImplDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_impl_display_display_foreign_type_into_hashmap_impl_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String> {
        self.into_iter()
            .map(|(k, v)| (k, v.display_foreign_type()))
            .collect()
    }
}
