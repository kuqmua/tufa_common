//todo - maybe should exists types of config - config for parsing env file, config builder for checks some logic like its have u8 but its not 42(wrapper under u8)

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,
)]
pub struct ConfigStruct {
    pub server_port: u16,
    pub hmac_secret: String,
    pub base_url: String,
    pub require_ssl: bool,

    pub github_name: String,
    pub github_token: String,

    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,

    pub providers_link_parts_source: crate::server::resource::Resource,

    pub timezone: i32,

    pub redis_ip: String,
    pub redis_port: u16,

    pub mongo_url: String,

    pub mongo_connection_timeout: u64,

    pub mongo_providers_link_parts_db_name: String,
    pub mongo_providers_logs_db_name: String,
    pub mongo_providers_logs_db_collection_handle_second_part: String,
    pub mongo_providers_logs_db_collection_document_field_name_handle: String,

    pub database_url: String,//postgres_url, naming required by sqlx::query::query!

    pub postgres_fourth_handle_url_part: String,
    pub postgres_fifth_handle_url_part: String,
    pub postgres_sixth_handle_url_part: String,

    pub postgres_login: String,
    pub postgres_password: String,
    pub postgres_ip: String, //todo: 4x u8
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_params: String,

    pub postgres_connection_timeout: u64,

    pub path_to_provider_link_parts_folder: String,
    pub log_file_extension: String,

    pub starting_check_link: String, //todo add browser url limit check

    pub is_preparation_enabled: bool,
    pub tracing_type: crate::server::tracing_type::TracingType,
    pub source_place_type: crate::common::source_place_type::SourcePlaceType,
}