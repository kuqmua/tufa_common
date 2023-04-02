pub trait HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString<HashMapKeyGeneric> {
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, &'static str>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapDisplayDisplayForeignTypeIntoHashmapDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, &'static str> {
        self.into_iter()
            .map(|(k, v)| (k, v.display_foreign_type()))
            .collect()
    }
}
