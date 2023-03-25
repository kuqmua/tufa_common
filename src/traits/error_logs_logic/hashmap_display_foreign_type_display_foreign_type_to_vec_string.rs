pub trait HashmapDisplayForeignTypeDisplayForeignTypeToVecString {
    fn hashmap_display_foreign_type_display_foreign_type_to_vec_string(
        self,
    ) -> std::collections::HashMap<String, String>; //todo - key: impl Display, value: impl Display instead?
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashmapDisplayForeignTypeDisplayForeignTypeToVecString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::traits::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_to_vec_string(
        self,
    ) -> std::collections::HashMap<String, String> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v.display_foreign_type()))
            .collect()
    }
}
