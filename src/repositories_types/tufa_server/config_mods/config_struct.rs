//todo - maybe should exists types of config - config for parsing env file, config builder for checks some logic like its have u8 but its not 42(wrapper under u8)

#[derive(
    Debug,
    Clone,
    init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed,
    Default,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields,
)]
pub struct ConfigStruct {
    pub server_ip: String,
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

    pub mongo_first_handle_url_part: String,
    pub mongo_second_handle_url_part: String,
    pub mongo_third_handle_url_part: String,
    pub mongo_fourth_handle_url_part: String,
    pub mongo_fifth_handle_url_part: String,

    pub mongo_login: String,
    pub mongo_password: String,
    pub mongo_ip: String, //todo: 4x u8
    pub mongo_port: u16,
    pub mongo_params: String,

    pub mongo_connection_timeout: u64,

    pub mongo_providers_link_parts_db_name: String,
    pub mongo_providers_logs_db_name: String,
    pub mongo_providers_logs_db_collection_handle_second_part: String,
    pub mongo_providers_logs_db_collection_document_field_name_handle: String,

    pub is_mongo_initialization_enabled: bool,
    pub is_mongo_initialization_enabled_providers: bool,
    pub is_mongo_initialization_enabled_arxiv: bool,
    pub is_mongo_initialization_enabled_biorxiv: bool,
    pub is_mongo_initialization_enabled_github: bool,
    pub is_mongo_initialization_enabled_habr: bool,
    pub is_mongo_initialization_enabled_medrxiv: bool,
    pub is_mongo_initialization_enabled_reddit: bool,
    pub is_mongo_initialization_enabled_twitter: bool,

    pub is_mongo_write_error_logs_enabled: bool,
    pub is_mongo_write_error_logs_enabled_providers: bool,
    pub is_mongo_write_error_logs_enabled_arxiv: bool,
    pub is_mongo_write_error_logs_enabled_biorxiv: bool,
    pub is_mongo_write_error_logs_enabled_github: bool,
    pub is_mongo_write_error_logs_enabled_habr: bool,
    pub is_mongo_write_error_logs_enabled_medrxiv: bool,
    pub is_mongo_write_error_logs_enabled_reddit: bool,
    pub is_mongo_write_error_logs_enabled_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_github: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_collections_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_github: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_twitter: bool,

    pub is_mongo_link_parts_randomize_order_enabled: bool,
    pub is_mongo_link_parts_randomize_order_enabled_providers: bool,
    pub is_mongo_link_parts_randomize_order_enabled_arxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_biorxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_github: bool,
    pub is_mongo_link_parts_randomize_order_enabled_habr: bool,
    pub is_mongo_link_parts_randomize_order_enabled_medrxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_reddit: bool,
    pub is_mongo_link_parts_randomize_order_enabled_twitter: bool,

    pub postgres_first_handle_url_part: String,
    pub postgres_second_handle_url_part: String,
    pub postgres_third_handle_url_part: String,
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

    pub is_postgres_initialization_enabled: bool,
    pub is_postgres_initialization_enabled_providers: bool,
    pub is_postgres_initialization_enabled_arxiv: bool,
    pub is_postgres_initialization_enabled_biorxiv: bool,
    pub is_postgres_initialization_enabled_github: bool,
    pub is_postgres_initialization_enabled_habr: bool,
    pub is_postgres_initialization_enabled_medrxiv: bool,
    pub is_postgres_initialization_enabled_reddit: bool,
    pub is_postgres_initialization_enabled_twitter: bool,

    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub path_to_provider_link_parts_folder: String,
    pub log_file_extension: String,

    pub is_write_error_logs_in_local_folder_enabled: bool,
    pub is_write_error_logs_in_local_folder_enabled_providers: bool,
    pub is_write_error_logs_in_local_folder_enabled_arxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_biorxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_github: bool,
    pub is_write_error_logs_in_local_folder_enabled_habr: bool,
    pub is_write_error_logs_in_local_folder_enabled_medrxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_reddit: bool,
    pub is_write_error_logs_in_local_folder_enabled_twitter: bool,

    pub is_cleaning_warning_logs_directory_enabled: bool,
    pub is_cleaning_warning_logs_directory_enabled_providers: bool,
    pub is_cleaning_warning_logs_directory_enabled_arxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_biorxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_github: bool,
    pub is_cleaning_warning_logs_directory_enabled_habr: bool,
    pub is_cleaning_warning_logs_directory_enabled_medrxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_reddit: bool,
    pub is_cleaning_warning_logs_directory_enabled_twitter: bool,

    pub starting_check_link: String, //todo add browser url limit check
    pub check_link_arxiv: String,    //todo add browser url limit check
    pub check_link_biorxiv: String,  //todo add browser url limit check
    pub check_link_github: String,   //todo add browser url limit check
    pub check_link_habr: String,     //todo add browser url limit check
    pub check_link_medrxiv: String,  //todo add browser url limit check
    pub check_link_reddit: String,   //todo add browser url limit check
    pub check_link_twitter: String,  //todo add browser url limit check

    pub is_enabled_providers: bool,
    pub is_enabled_arxiv: bool,
    pub is_enabled_biorxiv: bool,
    pub is_enabled_github: bool,
    pub is_enabled_habr: bool,
    pub is_enabled_medrxiv: bool,
    pub is_enabled_reddit: bool,
    pub is_enabled_twitter: bool,

    pub is_dbs_initialization_enabled: bool,
    pub is_dbs_initialization_enabled_providers: bool,
    pub is_dbs_initialization_enabled_arxiv: bool,
    pub is_dbs_initialization_enabled_biorxiv: bool,
    pub is_dbs_initialization_enabled_github: bool,
    pub is_dbs_initialization_enabled_habr: bool,
    pub is_dbs_initialization_enabled_medrxiv: bool,
    pub is_dbs_initialization_enabled_reddit: bool,
    pub is_dbs_initialization_enabled_twitter: bool,

    pub is_links_limit_enabled_providers: bool,
    pub is_links_limit_enabled_arxiv: bool,
    pub is_links_limit_enabled_biorxiv: bool,
    pub is_links_limit_enabled_github: bool,
    pub is_links_limit_enabled_habr: bool,
    pub is_links_limit_enabled_medrxiv: bool,
    pub is_links_limit_enabled_reddit: bool,
    pub is_links_limit_enabled_twitter: bool,

    pub links_limit_providers: usize, //override links limit for providers. this value applied for each provider
    pub links_limit_arxiv: usize,
    pub links_limit_biorxiv: usize,
    pub links_limit_github: usize,
    pub links_limit_habr: usize,
    pub links_limit_medrxiv: usize,
    pub links_limit_reddit: usize,
    pub links_limit_twitter: usize,

    pub is_preparation_enabled: bool,
    pub tracing_type: crate::server::tracing_type::TracingType,
    pub is_parent_tracing_enabled: bool,
    pub source_place_type: crate::common::source_place_type::SourcePlaceType,
    pub is_tracing_time_enabled: bool,
}
