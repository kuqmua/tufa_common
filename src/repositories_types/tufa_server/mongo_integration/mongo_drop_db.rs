#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropDbErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_db<'a>(
    config: &'static impl crate::traits::config_fields::GetMongoClient,
    db_name: &'a str,
) -> Result<(), Box<crate::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed<'a>>> {
    if let Err(e) = config.get_mongo_client().database(db_name).drop(None).await {
        return Err(Box::new(
            crate::repositories_types::tufa_server::mongo_integration::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(())
}
