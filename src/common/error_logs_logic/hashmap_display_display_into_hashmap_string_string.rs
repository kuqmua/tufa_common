pub trait HashMapDisplayDisplayIntoHashMapStringString {
    fn hashmap_display_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayDisplayIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, String> {
        self.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}
