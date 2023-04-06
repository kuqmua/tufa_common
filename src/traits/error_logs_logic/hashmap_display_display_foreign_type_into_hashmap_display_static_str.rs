pub trait HashmapDisplayDisplayForeignTypeIntoHashmapDisplayStaticStr<HashMapKeyGeneric> {
    fn hashmap_display_display_foreign_type_into_hashmap_display_static_str(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapDisplayDisplayForeignTypeIntoHashmapDisplayStaticStr<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_into_hashmap_display_static_str(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, String> {
        self.into_iter()
            .map(|(k, v)| (k, v.display_foreign_type().to_string()))
            .collect()
    }
}
