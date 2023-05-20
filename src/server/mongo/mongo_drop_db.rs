#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoDropDbErrorNamed<'a> {
    MongoDB {
        #[eo_display]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn mongo_drop_db<'a>(
    mongo_client: &mongodb::Client,
    db_name: &'a str,
) -> Result<(), Box<crate::server::mongo::mongo_drop_db::MongoDropDbErrorNamed<'a>>> {
    if let Err(e) = mongo_client.database(db_name).drop(None).await {
        return Err(Box::new(
            crate::server::mongo::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ));
    }
    Ok(())
}
