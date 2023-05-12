#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoClientWithOptionsOriginErrorNamed<'a> {
    Mongo {
        #[eo_display]
        error: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub fn mongo_client_with_options<'a>(client_options: mongodb::options::ClientOptions) -> Result<mongodb::Client, Box<crate::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed<'a>>>{
    match mongodb::Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed::Mongo { 
                error: e, 
                code_occurence: crate::code_occurence_tufa_common!() 
            } 
        )),
        Ok(client) => Ok(client),
    }
}