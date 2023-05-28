pub trait HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString {
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::common::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::common::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v.display_foreign_type()))
            .collect()
    }
}
