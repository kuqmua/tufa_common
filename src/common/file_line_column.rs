use generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsFromCrate;

#[derive(Debug, Clone, GenerateGetterTraitsForStructFieldsFromCrate)] //todo fields implementations
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}
