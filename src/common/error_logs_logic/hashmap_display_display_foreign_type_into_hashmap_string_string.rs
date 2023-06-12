pub trait HashMapDisplayDisplayForeignTypeIntoHashMapStringString {
    fn hashmap_display_display_foreign_type_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayDisplayForeignTypeIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::common::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String> {
        self.into_iter()
            .map(|(k, v)| (k.to_string(), v.display_foreign_type()))
            .collect()
    }
}
