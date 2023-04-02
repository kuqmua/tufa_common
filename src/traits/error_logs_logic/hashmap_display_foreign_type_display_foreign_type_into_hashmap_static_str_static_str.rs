pub trait HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStaticStrStaticStr {
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str(
        self,
    ) -> std::collections::HashMap<&'static str, &'static str>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStaticStrStaticStr
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::traits::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::traits::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str(
        self,
    ) -> std::collections::HashMap<&'static str, &'static str> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v.display_foreign_type()))
            .collect()
    }
}
