pub trait HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringDisplayForeignType<
    HashMapValueGeneric,
>
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric>
    HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringDisplayForeignType<
        HashMapValueGeneric,
    > for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric:
        crate::common::display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: crate::common::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v))
            .collect()
    }
}
