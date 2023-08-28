#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum PostgresLike {
    ZeroOrMoreCharacters, //Percent sign ( %) matches any sequence of zero or more characters.
    AnySingleCharacter,   //Underscore sign ( _)  matches any single character.
}
