#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetProvidersLinkPartsErrorNamed<'a> {
    MongoDB {
        #[eo_display_foreign_type]
        mongodb: mongodb::error::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoSuchCollections {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        no_such_collections: std::collections::HashMap<std::string::String, std::string::String>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetDocuments {
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        get_documents: std::collections::HashMap<std::string::String, MongoGetDocumentsAsStringVectorErrorUnnamed<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum MongoGetDocumentsAsStringVectorErrorUnnamed<'a> {
    MongoGetDocumentsAsStringVector(crate::repositories_types::tufa_server::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed<'a>),
}

pub async fn mongo_get_providers_link_parts<'a>(
    config: &'a (
        impl crate::traits::config_fields::GetMongoUrl
        + crate::traits::config_fields::GetMongoProvidersLinkPartsDbName
        + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
        + crate::traits::config_fields::GetIsLinksLimitEnabledProviders
        + crate::traits::config_fields::GetLinksLimitProviders
        + crate::traits::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart

        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledArxiv
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledBiorxiv
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledGithub
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledHabr
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledMedrxiv
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledReddit
        + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledTwitter
        + crate::traits::config_fields::GetIsLinksLimitEnabledArxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledBiorxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledGithub
        + crate::traits::config_fields::GetIsLinksLimitEnabledHabr
        + crate::traits::config_fields::GetIsLinksLimitEnabledMedrxiv
        + crate::traits::config_fields::GetIsLinksLimitEnabledReddit
        + crate::traits::config_fields::GetIsLinksLimitEnabledTwitter
        + crate::traits::config_fields::GetLinksLimitArxiv
        + crate::traits::config_fields::GetLinksLimitBiorxiv
        + crate::traits::config_fields::GetLinksLimitGithub
        + crate::traits::config_fields::GetLinksLimitHabr
        + crate::traits::config_fields::GetLinksLimitMedrxiv
        + crate::traits::config_fields::GetLinksLimitReddit
        + crate::traits::config_fields::GetLinksLimitTwitter

        + crate::traits::config_fields::GetIsEnabledArxiv
        + crate::traits::config_fields::GetIsEnabledBiorxiv
        + crate::traits::config_fields::GetIsEnabledGithub
        + crate::traits::config_fields::GetIsEnabledHabr
        + crate::traits::config_fields::GetIsEnabledMedrxiv
        + crate::traits::config_fields::GetIsEnabledReddit
        + crate::traits::config_fields::GetIsEnabledTwitter

        + crate::traits::config_fields::GetIsEnabledArxiv
        + crate::traits::config_fields::GetIsEnabledBiorxiv
        + crate::traits::config_fields::GetIsEnabledGithub
        + crate::traits::config_fields::GetIsEnabledHabr
        + crate::traits::config_fields::GetIsEnabledMedrxiv
        + crate::traits::config_fields::GetIsEnabledReddit
        + crate::traits::config_fields::GetIsEnabledTwitter
    )
) -> Result<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed<'a>> {
    match mongodb::options::ClientOptions::parse(config.get_mongo_url()).await {
        Err(e) => Err(
            crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        ),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(
                crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            ),
            Ok(client) => {
                let db = client.database(&config.get_mongo_providers_link_parts_db_name());
                match db.list_collection_names(None).await {
                    Err(e) => Err(
                        crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    ),
                    Ok(vec_collection_names) => {
                        let no_collection_error_hashmap = {
                            use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec(config)
                            .into_iter()
                            .filter_map(|pk| {
                                let collection_name = pk.get_mongo_log_collection_name(config);
                                if !vec_collection_names.contains(&collection_name) {
                                    return Some((pk.to_string(), collection_name));
                                }
                                None
                            })
                            .collect::<std::collections::HashMap<String, String>>()
                        };
                        if !no_collection_error_hashmap.is_empty() {
                            return Err(
                                crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::NoSuchCollections {
                                    no_such_collections: no_collection_error_hashmap,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }
                            );
                        }
                        let result_get_documents_hashmap =
                                futures::future::join_all({
                                    use crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                                    crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec(config).iter().map(|pk| async {
                                        (
                                            *pk,
                                            crate::repositories_types::tufa_server::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector(
                                                db.collection::<mongodb::bson::Document>(&pk.get_mongo_log_collection_name(config)),
                                                config.get_mongo_providers_logs_db_collection_document_field_name_handle(),
                                                crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_mongo_provider_link_parts_aggregation(
                                                    pk,
                                                    config
                                                ),
                                            )
                                            .await,
                                        )
                                    })
                                })
                            .await;
                        let mut success_hashmap: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
                            std::collections::HashMap::with_capacity(result_get_documents_hashmap.len());
                        let mut error_hashmap: std::collections::HashMap<
                            String,
                            crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed,
                        > = std::collections::HashMap::new();
                        for (pk, result) in result_get_documents_hashmap.into_iter() {
                            match result {
                                Err(e) => {
                                    error_hashmap.insert(
                                        pk.to_string(), 
                                        crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed::MongoGetDocumentsAsStringVector(*e)
                                    );
                                }
                                Ok(vec) => {
                                    success_hashmap.insert(pk, vec);
                                }
                            }
                        }
                        if !error_hashmap.is_empty() {
                            return Err(
                                crate::repositories_types::tufa_server::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::GetDocuments {
                                    get_documents: error_hashmap,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }
                            );
                        }
                        Ok(success_hashmap)
                    }
                }
            }
        },
    }
}
