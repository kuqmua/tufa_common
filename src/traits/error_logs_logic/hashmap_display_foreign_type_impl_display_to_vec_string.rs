pub trait HashmapDisplayForeignTypeImplDisplayToVecString<HashMapValueGeneric> {
    fn hashmap_display_foreign_type_impl_display_to_vec_string(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapDisplayForeignTypeImplDisplayToVecString<HashMapValueGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::traits::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_impl_display_to_vec_string(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v))
            .collect()
    }
}
