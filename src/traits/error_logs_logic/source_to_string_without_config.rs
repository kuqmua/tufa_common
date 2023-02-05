// pub trait SourceToStringWithoutConfig {
//     fn source_to_string_without_config(&self) -> String;
// }
//
pub trait SourceToStringWithoutConfigLifetime<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String;
}
