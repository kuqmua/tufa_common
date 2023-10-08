//todo openapi
//todo test if create\update\delete empty array
pub const ROUTE_NAME: &str = "cats";
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
    serde_derive::Serialize,
    serde_derive::Deserialize,
    generate_postgresql_crud::GeneratePostgresqlCrud,
)]
pub struct Cat {
    #[generate_postgresql_crud_id]
    pub id: i64, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    pub name: String,
    pub color: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatOrderByWrapper(
    #[serde(deserialize_with = "deserialize_cat_order_by")]
    pub  crate::server::postgres::order_by::OrderBy<CatColumn>,
);

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for CatOrderByWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.unwrap_or_default();
        format!("column={column},order={order}")
    }
}

fn deserialize_cat_order_by<'de, D>(
    deserializer: D,
) -> Result<crate::server::postgres::order_by::OrderBy<CatColumn>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    };
    let split_inner_url_parameters_symbol = ',';
    let default_message = "Invalid CatOrderBy:";
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_column) => match {
                                use std::str::FromStr;
                                CatColumn::from_str(possible_column)
                            } {
                                Ok(column) => column,
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {column_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                )));
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_column) => match {
                            use std::str::FromStr;
                            CatColumn::from_str(possible_column)
                        } {
                            Ok(column) => column,
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {column_equal_str} failed to offset_slice.get(0..)"
                            )));
                        }
                    },
                },
                None => {
                    return Err(serde::de::Error::custom(&format!(
                        "{default_message} {column_equal_str} failed to string_deserialized.get(offset..)"
                    )));
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {column_equal_str} index overflow"
                )));
            }
        },
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} {column_equal_str} not found"
            )));
        }
    };
    let order = match string_deserialized.find(order_equal_str) {
        Some(index) => match index.checked_add(order_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_order) => match {
                                use std::str::FromStr;
                                crate::server::postgres::order::Order::from_str(possible_order)
                            } {
                                Ok(order) => Some(order),
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {order_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                )));
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_order) => match {
                            use std::str::FromStr;
                            crate::server::postgres::order::Order::from_str(possible_order)
                        } {
                            Ok(order) => Some(order),
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {order_equal_str} failed to offset_slice.get(0..)"
                            )));
                        }
                    },
                },
                None => {
                    return Err(serde::de::Error::custom(&format!(
                        "{default_message} {order_equal_str} failed to string_deserialized.get(offset..)"
                    )));
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {order_equal_str} index overflow"
                )));
            }
        },
        None => None,
    };
    Ok(crate::server::postgres::order_by::OrderBy { column, order })
}

//////////////////

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_201_created
)]
pub enum TryCreateBatch {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_201_created
)]
pub enum TryCreate {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDeleteById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
///////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDeleteWithBody {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryDelete {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    DeleteQueryTryFromUrlEncoding {
        #[eo_error_occurence]
        checked_add: DeleteQueryTryFromUrlEncodingErrorNamed,//todo renaming?
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoQueryParameters {
        #[eo_display_with_serialize_deserialize]
        no_query_parameters: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CommitFailed {
        #[eo_display]
        commit_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    DeleteAndRollbackFailed {
        #[eo_display]
        delete_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    crate::repositories_types::tufa_server::routes::api::cats::CatOptions,
    tvfrr_200_ok
)]
pub enum TryReadById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    Vec::<crate::repositories_types::tufa_server::routes::api::cats::CatOptions>,
    tvfrr_200_ok
)]
pub enum TryReadWithBody {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    Vec::<crate::repositories_types::tufa_server::routes::api::cats::CatOptions>,
    tvfrr_200_ok
)]
pub enum TryRead {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //todo - no parameters case?
}

/////////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryUpdateById {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

/////////
#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    from_sqlx_postgres_error::FromSqlxPostgresError,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponse,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute(
    (),
    tvfrr_200_ok
)]
pub enum TryUpdate {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    #[tvfrr_500_internal_server_error]
    BindQuery {
        #[eo_error_occurence]
        checked_add: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    CommitFailed {
        #[eo_display]
        commit_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    //todo what status code should return if non_existing_primary_keys = 400, but transaction rollback failed = 500
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: Vec<i64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    UpdateAndRollbackFailed {
        #[eo_display]
        update_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
//////
