#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum GetByIdHttpResponseVariants {
    Cat(crate::repositories_types::tufa_server::routes::api::cats::Cat),
    //
    Bigserial {
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamedWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    //
    Configuration {
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Database {
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Io {
        io_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Tls {
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Decode {
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
    },
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::FromEnumWithLifetime,
)]
#[type_variants_from_reqwest_response::from_enum_paths_with_lifetime(GetByIdHttpResponseVariants)]
pub enum TryGetByIdErrorHttpResponse<'a> {
    Bigserial {
        #[eo_error_occurence]
        bigserial: crate::server::postgres::bigserial::BigserialTryFromI64ErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        box_dyn_database_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Io {
        #[eo_display]
        io_error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode_box_dyn_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[non_exhaustive] case
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl TryFrom<GetByIdHttpResponseVariants>
    for crate::repositories_types::tufa_server::routes::api::cats::Cat
{
    type Error = TryGetByIdErrorHttpResponseWithSerializeDeserialize;
    fn try_from(
        value: GetByIdHttpResponseVariants,
    ) -> Result<Self, TryGetByIdErrorHttpResponseWithSerializeDeserialize> {
        match value {
            GetByIdHttpResponseVariants::Cat(cat) => Ok(cat),
            //
            GetByIdHttpResponseVariants::Bigserial {
                bigserial,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Bigserial { bigserial, code_occurence }),
            //
            GetByIdHttpResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal,
                project_commit_to_use,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::ProjectCommitExtractorNotEqual { project_commit_not_equal, project_commit_to_use, code_occurence }),
            GetByIdHttpResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::ProjectCommitExtractorToStrConversion { project_commit_to_str_conversion, code_occurence }),
            GetByIdHttpResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::NoProjectCommitExtractorHeader { no_project_commit_header, code_occurence }),
            //
            GetByIdHttpResponseVariants::Configuration {
                configuration_box_dyn_error,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::Configuration {
                    configuration_box_dyn_error,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::Database {
                box_dyn_database_error,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Database {
                box_dyn_database_error,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::Io {
                io_error,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Io {
                io_error,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::Tls {
                box_dyn_error,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Tls {
                box_dyn_error,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::Protocol {
                protocol,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::RowNotFound {
                row_not_found,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::RowNotFound {
                    row_not_found,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::TypeNotFound {
                    type_not_found,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::ColumnIndexOutOfBounds {
                    column_index_out_of_bounds,
                    len,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::ColumnNotFound {
                    column_not_found,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::ColumnDecode {
                    column_decode_index,
                    source_handle,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::Decode {
                decode_box_dyn_error,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Decode {
                decode_box_dyn_error,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::PoolTimedOut {
                    pool_timed_out,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::PoolClosed {
                pool_closed,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::PoolClosed {
                    pool_closed,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::WorkerCrashed {
                    worker_crashed,
                    code_occurence,
                },
            ),
            GetByIdHttpResponseVariants::Migrate {
                migrate,
                code_occurence,
            } => Err(TryGetByIdErrorHttpResponseWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            }),
            GetByIdHttpResponseVariants::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Err(
                TryGetByIdErrorHttpResponseWithSerializeDeserialize::UnexpectedCase {
                    unexpected_case,
                    code_occurence,
                },
            ),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryGetByIdErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        get_by_id: TryGetByIdErrorHttpResponseWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub async fn try_get_by_id<'a>(
    server_location: std::string::String,
    path_parameters: crate::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters,
) -> Result<crate::repositories_types::tufa_server::routes::api::cats::Cat, TryGetByIdErrorNamed<'a>>
{
    // todo maybe path_parameters already must be non zero?
    if let true = path_parameters.id.is_negative() {
        return Err(TryGetByIdErrorNamed::BelowZero {
            below_zero: path_parameters.id,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    let url = format!(
        "{server_location}/api/{}/{}",
        crate::repositories_types::tufa_server::routes::api::cats::CATS,
        path_parameters.id
    );
    // println!("{url}");
    match reqwest::Client::new()
        .get(&url)
        .header(
            crate::common::git::project_git_info::PROJECT_COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .project_commit,
        )
        .send()
        .await
    {
        Ok(response) => match response.json::<GetByIdHttpResponseVariants>().await {
            Ok(variants) => {
                match crate::repositories_types::tufa_server::routes::api::cats::Cat::try_from(
                    variants,
                ) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(TryGetByIdErrorNamed::ExpectedType {
                        get_by_id: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }),
                }
            }
            Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        },
        Err(e) => Err(TryGetByIdErrorNamed::Reqwest {
            reqwest: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
