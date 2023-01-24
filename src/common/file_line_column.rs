use generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsFromCrate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, GenerateGetterTraitsForStructFieldsFromCrate, Serialize, Deserialize)] //todo fields implementations
pub struct FileLineColumn {
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
}
