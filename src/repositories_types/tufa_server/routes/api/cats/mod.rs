//todo openapi
//todo test if create\update\delete empty array
pub trait GetConfigGetPostgresPool:
    crate::repositories_types::tufa_server::config::config_struct::GetConfig
    + crate::server::routes::helpers::get_postgres_pool::GetPostgresPool
    + crate::common::config::config_fields::GetSourcePlaceType
    + crate::common::config::config_fields::GetTimezone
{
}

pub type DynArcGetConfigGetPostgresPoolSendSync = std::sync::Arc<
    dyn crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
        + Send
        + Sync,
>;

#[derive(
    Debug,
    generate_postgresql_crud::GeneratePostgresqlCrud,
)]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}

//
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    // from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    std::vec::Vec::<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    tvfrr_201_created
)]
pub enum Kekw {
    // #[tvfrr_400_bad_request]
    // FailedToDeserializeQueryString {
    //     #[eo_display_with_serialize_deserialize]
    //     failed_to_deserialize_query_string: std::string::String,
    //     code_occurence: crate::common::code_occurence::CodeOccurence,
    // },
    #[tvfrr_400_bad_request]
    VariantName {
        #[eo_error_occurence]
        variant_field_name: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}