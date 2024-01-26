pub trait HashMapDisplayForeignTypeDisplayIntoHashMapStringString {
    fn hashmap_display_foreign_type_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayForeignTypeDisplayIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::common::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v.to_string()))
            .collect()
    }
}
