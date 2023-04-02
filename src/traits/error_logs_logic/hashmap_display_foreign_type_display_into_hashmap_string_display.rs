pub trait HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay<HashMapValueGeneric> {
    fn hashmap_display_foreign_type_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<&'static str, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashmapDisplayForeignTypeDisplayIntoHashMapStringDisplay<HashMapValueGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::traits::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<&'static str, HashMapValueGeneric> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v))
            .collect()
    }
}
