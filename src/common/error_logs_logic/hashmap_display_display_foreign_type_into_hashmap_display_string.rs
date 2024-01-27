pub trait HashMapDisplayDisplayForeignTypeIntoHashMapDisplayString<HashMapKeyGeneric> {
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayDisplayForeignTypeIntoHashMapDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: error_occurence_lib::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String> {
        self.into_iter()
            .map(|(k, v)| (k, v.display_foreign_type()))
            .collect()
    }
}
