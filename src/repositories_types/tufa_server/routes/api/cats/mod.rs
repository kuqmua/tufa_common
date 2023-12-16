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
)]//[u8; sds]
#[generate_postgresql_crud::generate_postgresql_crud_additional_http_status_codes_error_variants{
    // vec!
    // [
        enum ProjectCommitExtractorCheckErrorNamed {//crate::server::extractors::project_commit_extractor::
            #[tvfrr_400_bad_request]
            ProjectCommitExtractorNotEqual {
                #[eo_display_with_serialize_deserialize]
                project_commit_not_equal: std::string::String,
                #[eo_display_with_serialize_deserialize]
                project_commit_to_use: std::string::String,
                code_occurence: crate::common::code_occurence::CodeOccurence,
            },
            // #[tvfrr_400_bad_request]
            // ProjectCommitExtractorToStrConversion {
            //     #[eo_display]
            //     project_commit_to_str_conversion: http::header::ToStrError,
            //     code_occurence: crate::common::code_occurence::CodeOccurence,
            // },
            // #[tvfrr_400_bad_request]
            // NoProjectCommitExtractorHeader {
            //     #[eo_display_with_serialize_deserialize]
            //     no_project_commit_header: std::string::String,
            //     code_occurence: crate::common::code_occurence::CodeOccurence,
            // },
        };
        enum ProjectCommitExtractorCheckErrorNamed {
            #[tvfrr_400_bad_request]
            ProjectCommitExtractorNotEqual {
                #[eo_display_with_serialize_deserialize]
                project_commit_not_equal: std::string::String,
                #[eo_display_with_serialize_deserialize]
                project_commit_to_use: std::string::String,
                code_occurence: crate::common::code_occurence::CodeOccurence,
            },
            // #[tvfrr_400_bad_request]
            // ProjectCommitExtractorToStrConversion {
            //     #[eo_display]
            //     project_commit_to_str_conversion: http::header::ToStrError,
            //     code_occurence: crate::common::code_occurence::CodeOccurence,
            // },
        }
    // ]
}]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}