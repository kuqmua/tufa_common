// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum MongoGetProvidersLinkPartsErrorNamed<'a> {
//     MongoDB {
//         #[eo_display_foreign_type]
//         mongodb: mongodb::error::Error,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
//     CollectionIsNotEmpty {
//         #[eo_display_with_serialize_deserialize]
//         collection_is_not_empty: u64,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }