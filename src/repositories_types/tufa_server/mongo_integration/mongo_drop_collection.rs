#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropCollectionErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}