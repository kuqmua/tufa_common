#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoClientOptionsParseOriginErrorNamed<'a> {
    Mongo {
        #[eo_display]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_client_options_parse<'a>(
    config: &impl crate::traits::config_fields::GetMongoUrl
) -> Result<mongodb::options::ClientOptions, Box<crate::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed<'a>>>{
    match mongodb::options::ClientOptions::parse(&config.get_mongo_url()).await {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed::Mongo {
                error: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        )),
        Ok(client_options) => Ok(client_options),
    }
}