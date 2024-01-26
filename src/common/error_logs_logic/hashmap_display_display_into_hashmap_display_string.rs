pub trait HashMapDisplayDisplayIntoHashMapDisplayString<HashMapKeyGeneric> {
    fn hashmap_display_display_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayDisplayIntoHashMapDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String> {
        self.into_iter().map(|(k, v)| (k, v.to_string())).collect()
    }
}
