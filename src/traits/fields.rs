use crate::config_mods::log_type::LogType;
use crate::{
    config_mods::{source_place_type::SourcePlaceType, tracing_type::TracingType},
    server::resource::Resource,
};

//Config
pub trait GetServerIp {
    fn get_server_ip(&self) -> &String;
}
pub trait GetServerPort {
    fn get_server_port(&self) -> &u16;
}
pub trait GetHmacSecret {
    fn get_hmac_secret(&self) -> &String;
}
pub trait GetBaseUrl {
    fn get_base_url(&self) -> &String;
}
pub trait GetRequireSsl {
    fn get_require_ssl(&self) -> &bool;
}
pub trait GetGithubName {
    fn get_github_name(&self) -> &String;
}
pub trait GetGithubToken {
    fn get_github_token(&self) -> &String;
}
pub trait GetRedditUserAgent {
    fn get_reddit_user_agent(&self) -> &String;
}
pub trait GetRedditClientId {
    fn get_reddit_client_id(&self) -> &String;
}
pub trait GetRedditClientSecret {
    fn get_reddit_client_secret(&self) -> &String;
}
pub trait GetRedditUsername {
    fn get_reddit_username(&self) -> &String;
}
pub trait GetRedditPassword {
    fn get_reddit_password(&self) -> &String;
}
pub trait GetProvidersLinkPartsSource {
    fn get_providers_link_parts_source(&self) -> &Resource;
}
pub trait GetTimezone {
    fn get_timezone(&self) -> &i32;
}
pub trait GetRedisIp {
    fn get_redis_ip(&self) -> &String;
}
pub trait GetRedisPort {
    fn get_redis_port(&self) -> &u16;
}
pub trait GetMongoFirstHandleUrlPart {
    fn get_mongo_first_handle_url_part(&self) -> &String;
}
pub trait GetMongoSecondHandleUrlPart {
    fn get_mongo_second_handle_url_part(&self) -> &String;
}
pub trait GetMongoThirdHandleUrlPart {
    fn get_mongo_third_handle_url_part(&self) -> &String;
}
pub trait GetMongoFourthHandleUrlPart {
    fn get_mongo_fourth_handle_url_part(&self) -> &String;
}
pub trait GetMongoFifthHandleUrlPart {
    fn get_mongo_fifth_handle_url_part(&self) -> &String;
}
pub trait GetMongoLogin {
    fn get_mongo_login(&self) -> &String;
}
pub trait GetMongoPassword {
    fn get_mongo_password(&self) -> &String;
}
pub trait GetMongoIp {
    fn get_mongo_ip(&self) -> &String;
}
pub trait GetMongoPort {
    fn get_mongo_port(&self) -> &u16;
}
pub trait GetMongoParams {
    fn get_mongo_params(&self) -> &String;
}
pub trait GetMongoConnectionTimeout {
    fn get_mongo_connection_timeout(&self) -> &u64;
}
pub trait GetMongoProvidersLinkPartsDbName {
    fn get_mongo_providers_link_parts_db_name(&self) -> &String;
}
pub trait GetMongoProvidersLogsDbName {
    fn get_mongo_providers_logs_db_name(&self) -> &String;
}
pub trait GetMongoProvidersLogsDbCollectionHandleSecondPart {
    fn get_mongo_providers_logs_db_collection_handle_second_part(&self) -> &String;
}
pub trait GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle {
    fn get_mongo_providers_logs_db_collection_document_field_name_handle(&self) -> &String;
}
pub trait GetIsMongoInitializationEnabled {
    fn get_is_mongo_initialization_enabled(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledProviders {
    fn get_is_mongo_initialization_enabled_providers(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledArxiv {
    fn get_is_mongo_initialization_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledBiorxiv {
    fn get_is_mongo_initialization_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledGithub {
    fn get_is_mongo_initialization_enabled_github(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledHabr {
    fn get_is_mongo_initialization_enabled_habr(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledMedrxiv {
    fn get_is_mongo_initialization_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledReddit {
    fn get_is_mongo_initialization_enabled_reddit(&self) -> &bool;
}
pub trait GetIsMongoInitializationEnabledTwitter {
    fn get_is_mongo_initialization_enabled_twitter(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabled {
    fn get_is_mongo_write_error_logs_enabled(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledProviders {
    fn get_is_mongo_write_error_logs_enabled_providers(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledArxiv {
    fn get_is_mongo_write_error_logs_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledBiorxiv {
    fn get_is_mongo_write_error_logs_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledGithub {
    fn get_is_mongo_write_error_logs_enabled_github(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledHabr {
    fn get_is_mongo_write_error_logs_enabled_habr(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledMedrxiv {
    fn get_is_mongo_write_error_logs_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledReddit {
    fn get_is_mongo_write_error_logs_enabled_reddit(&self) -> &bool;
}
pub trait GetIsMongoWriteErrorLogsEnabledTwitter {
    fn get_is_mongo_write_error_logs_enabled_twitter(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabled {
    fn get_is_mongo_cleaning_warning_logs_db_enabled(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledProviders {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_providers(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledArxiv {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledBiorxiv {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledGithub {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_github(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledHabr {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_habr(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledMedrxiv {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledReddit {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_reddit(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbEnabledTwitter {
    fn get_is_mongo_cleaning_warning_logs_db_enabled_twitter(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabled {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledProviders {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_providers(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledArxiv {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledBiorxiv {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledGithub {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_github(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledHabr {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_habr(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledMedrxiv {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledReddit {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_reddit(&self) -> &bool;
}
pub trait GetIsMongoCleaningWarningLogsDbCollectionsEnabledTwitter {
    fn get_is_mongo_cleaning_warning_logs_db_collections_enabled_twitter(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabled {
    fn get_is_mongo_link_parts_randomize_order_enabled(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledProviders {
    fn get_is_mongo_link_parts_randomize_order_enabled_providers(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledArxiv {
    fn get_is_mongo_link_parts_randomize_order_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledBiorxiv {
    fn get_is_mongo_link_parts_randomize_order_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledGithub {
    fn get_is_mongo_link_parts_randomize_order_enabled_github(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledHabr {
    fn get_is_mongo_link_parts_randomize_order_enabled_habr(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledMedrxiv {
    fn get_is_mongo_link_parts_randomize_order_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledReddit {
    fn get_is_mongo_link_parts_randomize_order_enabled_reddit(&self) -> &bool;
}
pub trait GetIsMongoLinkPartsRandomizeOrderEnabledTwitter {
    fn get_is_mongo_link_parts_randomize_order_enabled_twitter(&self) -> &bool;
}
pub trait GetPostgresFirstHandleUrlPart {
    fn get_postgres_first_handle_url_part(&self) -> &String;
}
pub trait GetPostgresSecondHandleUrlPart {
    fn get_postgres_second_handle_url_part(&self) -> &String;
}
pub trait GetPostgresThirdHandleUrlPart {
    fn get_postgres_third_handle_url_part(&self) -> &String;
}
pub trait GetPostgresFourthHandleUrlPart {
    fn get_postgres_fourth_handle_url_part(&self) -> &String;
}
pub trait GetPostgresFifthHandleUrlPart {
    fn get_postgres_fifth_handle_url_part(&self) -> &String;
}
pub trait GetPostgresSixthHandleUrlPart {
    fn get_postgres_sixth_handle_url_part(&self) -> &String;
}
pub trait GetPostgresLogin {
    fn get_postgres_login(&self) -> &String;
}
pub trait GetPostgresPassword {
    fn get_postgres_password(&self) -> &String;
}
pub trait GetPostgresIp {
    fn get_postgres_ip(&self) -> &String;
}
pub trait GetPostgresPort {
    fn get_postgres_port(&self) -> &u16;
}
pub trait GetPostgresDb {
    fn get_postgres_db(&self) -> &String;
}
pub trait GetPostgresParams {
    fn get_postgres_params(&self) -> &String;
}
pub trait GetPostgresConnectionTimeout {
    fn get_postgres_connection_timeout(&self) -> &u64;
}
pub trait GetIsPostgresInitializationEnabled {
    fn get_is_postgres_initialization_enabled(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledProviders {
    fn get_is_postgres_initialization_enabled_providers(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledArxiv {
    fn get_is_postgres_initialization_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledBiorxiv {
    fn get_is_postgres_initialization_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledGithub {
    fn get_is_postgres_initialization_enabled_github(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledHabr {
    fn get_is_postgres_initialization_enabled_habr(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledMedrxiv {
    fn get_is_postgres_initialization_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledReddit {
    fn get_is_postgres_initialization_enabled_reddit(&self) -> &bool;
}
pub trait GetIsPostgresInitializationEnabledTwitter {
    fn get_is_postgres_initialization_enabled_twitter(&self) -> &bool;
}
pub trait GetWarningLogsDirectoryName {
    fn get_warning_logs_directory_name(&self) -> &String;
}
pub trait GetUnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir {
    fn get_unhandled_success_handled_success_are_there_items_initialized_posts_dir(
        &self,
    ) -> &String;
}
pub trait GetPathToProviderLinkPartsFolder {
    fn get_path_to_provider_link_parts_folder(&self) -> &String;
}
pub trait GetLogFileExtension {
    fn get_log_file_extension(&self) -> &String;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabled {
    fn get_is_write_error_logs_in_local_folder_enabled(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledProviders {
    fn get_is_write_error_logs_in_local_folder_enabled_providers(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledArxiv {
    fn get_is_write_error_logs_in_local_folder_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledBiorxiv {
    fn get_is_write_error_logs_in_local_folder_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledGithub {
    fn get_is_write_error_logs_in_local_folder_enabled_github(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledHabr {
    fn get_is_write_error_logs_in_local_folder_enabled_habr(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledMedrxiv {
    fn get_is_write_error_logs_in_local_folder_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledReddit {
    fn get_is_write_error_logs_in_local_folder_enabled_reddit(&self) -> &bool;
}
pub trait GetIsWriteErrorLogsInLocalFolderEnabledTwitter {
    fn get_is_write_error_logs_in_local_folder_enabled_twitter(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabled {
    //todo change it
    fn get_is_cleaning_warning_logs_directory_enabled(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledProviders {
    fn get_is_cleaning_warning_logs_directory_enabled_providers(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledArxiv {
    fn get_is_cleaning_warning_logs_directory_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledBiorxiv {
    fn get_is_cleaning_warning_logs_directory_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledGithub {
    fn get_is_cleaning_warning_logs_directory_enabled_github(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledHabr {
    fn get_is_cleaning_warning_logs_directory_enabled_habr(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledMedrxiv {
    fn get_is_cleaning_warning_logs_directory_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledReddit {
    fn get_is_cleaning_warning_logs_directory_enabled_reddit(&self) -> &bool;
}
pub trait GetIsCleaningWarningLogsDirectoryEnabledTwitter {
    fn get_is_cleaning_warning_logs_directory_enabled_twitter(&self) -> &bool;
}
pub trait GetStartingCheckLink {
    fn get_starting_check_link(&self) -> &String;
}
pub trait GetCheckLinkArxiv {
    fn get_check_link_arxiv(&self) -> &String;
}
pub trait GetCheckLinkBiorxiv {
    fn get_check_link_biorxiv(&self) -> &String;
}
pub trait GetCheckLinkGithub {
    fn get_check_link_github(&self) -> &String;
}
pub trait GetCheckLinkHabr {
    fn get_check_link_habr(&self) -> &String;
}
pub trait GetCheckLinkMedrxiv {
    fn get_check_link_medrxiv(&self) -> &String;
}
pub trait GetCheckLinkReddit {
    fn get_check_link_reddit(&self) -> &String;
}
pub trait GetCheckLinkTwitter {
    fn get_check_link_twitter(&self) -> &String;
}
pub trait GetIsEnabledProviders {
    fn get_is_enabled_providers(&self) -> &bool;
}
pub trait GetIsEnabledArxiv {
    fn get_is_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsEnabledBiorxiv {
    fn get_is_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsEnabledGithub {
    fn get_is_enabled_github(&self) -> &bool;
}
pub trait GetIsEnabledHabr {
    fn get_is_enabled_habr(&self) -> &bool;
}
pub trait GetIsEnabledMedrxiv {
    fn get_is_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsEnabledReddit {
    fn get_is_enabled_reddit(&self) -> &bool;
}
pub trait GetIsEnabledTwitter {
    fn get_is_enabled_twitter(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabled {
    fn get_is_dbs_initialization_enabled(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledProviders {
    fn get_is_dbs_initialization_enabled_providers(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledArxiv {
    fn get_is_dbs_initialization_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledBiorxiv {
    fn get_is_dbs_initialization_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledGithub {
    fn get_is_dbs_initialization_enabled_github(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledHabr {
    fn get_is_dbs_initialization_enabled_habr(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledMedrxiv {
    fn get_is_dbs_initialization_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledReddit {
    fn get_is_dbs_initialization_enabled_reddit(&self) -> &bool;
}
pub trait GetIsDbsInitializationEnabledTwitter {
    fn get_is_dbs_initialization_enabled_twitter(&self) -> &bool;
}
pub trait GetIsPrintsEnabled {
    fn get_is_prints_enabled(&self) -> &bool;
}
pub trait GetIsPrintsEnabledProviders {
    fn get_is_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsPrintsEnabledArxiv {
    fn get_is_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsPrintsEnabledBiorxiv {
    fn get_is_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsPrintsEnabledGithub {
    fn get_is_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsPrintsEnabledHabr {
    fn get_is_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsPrintsEnabledMedrxiv {
    fn get_is_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsPrintsEnabledReddit {
    fn get_is_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsPrintsEnabledTwitter {
    fn get_is_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabled {
    fn get_is_warning_high_prints_enabled(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledProviders {
    fn get_is_warning_high_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledArxiv {
    fn get_is_warning_high_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledBiorxiv {
    fn get_is_warning_high_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledGithub {
    fn get_is_warning_high_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledHabr {
    fn get_is_warning_high_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledMedrxiv {
    fn get_is_warning_high_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledReddit {
    fn get_is_warning_high_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsWarningHighPrintsEnabledTwitter {
    fn get_is_warning_high_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabled {
    fn get_is_warning_low_prints_enabled(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledProviders {
    fn get_is_warning_low_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledArxiv {
    fn get_is_warning_low_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledBiorxiv {
    fn get_is_warning_low_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledGithub {
    fn get_is_warning_low_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledHabr {
    fn get_is_warning_low_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledMedrxiv {
    fn get_is_warning_low_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledReddit {
    fn get_is_warning_low_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsWarningLowPrintsEnabledTwitter {
    fn get_is_warning_low_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabled {
    fn get_is_success_prints_enabled(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledProviders {
    fn get_is_success_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledArxiv {
    fn get_is_success_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledBiorxiv {
    fn get_is_success_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledGithub {
    fn get_is_success_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledHabr {
    fn get_is_success_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledMedrxiv {
    fn get_is_success_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledReddit {
    fn get_is_success_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsSuccessPrintsEnabledTwitter {
    fn get_is_success_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabled {
    fn get_is_partial_success_prints_enabled(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledProviders {
    fn get_is_partial_success_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledArxiv {
    fn get_is_partial_success_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledBiorxiv {
    fn get_is_partial_success_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledGithub {
    fn get_is_partial_success_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledHabr {
    fn get_is_partial_success_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledMedrxiv {
    fn get_is_partial_success_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledReddit {
    fn get_is_partial_success_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsPartialSuccessPrintsEnabledTwitter {
    fn get_is_partial_success_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabled {
    fn get_is_error_prints_enabled(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledProviders {
    fn get_is_error_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledArxiv {
    fn get_is_error_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledBiorxiv {
    fn get_is_error_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledGithub {
    fn get_is_error_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledHabr {
    fn get_is_error_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledMedrxiv {
    fn get_is_error_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledReddit {
    fn get_is_error_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsErrorPrintsEnabledTwitter {
    fn get_is_error_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabled {
    fn get_is_time_measurement_prints_enabled(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledProviders {
    fn get_is_time_measurement_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledArxiv {
    fn get_is_time_measurement_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledBiorxiv {
    fn get_is_time_measurement_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledGithub {
    fn get_is_time_measurement_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledHabr {
    fn get_is_time_measurement_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledMedrxiv {
    fn get_is_time_measurement_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledReddit {
    fn get_is_time_measurement_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsTimeMeasurementPrintsEnabledTwitter {
    fn get_is_time_measurement_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabled {
    fn get_is_info_prints_enabled(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledProviders {
    fn get_is_info_prints_enabled_providers(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledArxiv {
    fn get_is_info_prints_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledBiorxiv {
    fn get_is_info_prints_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledGithub {
    fn get_is_info_prints_enabled_github(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledHabr {
    fn get_is_info_prints_enabled_habr(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledMedrxiv {
    fn get_is_info_prints_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledReddit {
    fn get_is_info_prints_enabled_reddit(&self) -> &bool;
}
pub trait GetIsInfoPrintsEnabledTwitter {
    fn get_is_info_prints_enabled_twitter(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledProviders {
    fn get_is_links_limit_enabled_providers(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledArxiv {
    fn get_is_links_limit_enabled_arxiv(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledBiorxiv {
    fn get_is_links_limit_enabled_biorxiv(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledGithub {
    fn get_is_links_limit_enabled_github(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledHabr {
    fn get_is_links_limit_enabled_habr(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledMedrxiv {
    fn get_is_links_limit_enabled_medrxiv(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledReddit {
    fn get_is_links_limit_enabled_reddit(&self) -> &bool;
}
pub trait GetIsLinksLimitEnabledTwitter {
    fn get_is_links_limit_enabled_twitter(&self) -> &bool;
}
pub trait GetLinksLimitProviders {
    fn get_links_limit_providers(&self) -> &usize;
}
pub trait GetLinksLimitArxiv {
    fn get_links_limit_arxiv(&self) -> &usize;
}
pub trait GetLinksLimitBiorxiv {
    fn get_links_limit_biorxiv(&self) -> &usize;
}
pub trait GetLinksLimitGithub {
    fn get_links_limit_github(&self) -> &usize;
}
pub trait GetLinksLimitHabr {
    fn get_links_limit_habr(&self) -> &usize;
}
pub trait GetLinksLimitMedrxiv {
    fn get_links_limit_medrxiv(&self) -> &usize;
}
pub trait GetLinksLimitReddit {
    fn get_links_limit_reddit(&self) -> &usize;
}
pub trait GetLinksLimitTwitter {
    fn get_links_limit_twitter(&self) -> &usize;
}
pub trait GetIsPreparationEnabled {
    fn get_is_preparation_enabled(&self) -> &bool;
}
pub trait GetLogType {
    fn get_log_type(&self) -> &LogType;
}
pub trait GetTracingType {
    fn get_tracing_type(&self) -> &TracingType;
}
pub trait GetIsParentTracingEnabled {
    fn get_is_parent_tracing_enabled(&self) -> &bool;
}
pub trait GetSourcePlaceType {
    fn get_source_place_type(&self) -> &SourcePlaceType;
}
pub trait GetIsTracingTimeEnabled {
    fn get_is_tracing_time_enabled(&self) -> &bool;
}
pub trait GetErrorRed {
    fn get_error_red(&self) -> &u8;
}
pub trait GetErrorGreen {
    fn get_error_green(&self) -> &u8;
}
pub trait GetErrorBlue {
    fn get_error_blue(&self) -> &u8;
}
pub trait GetWarningHighRed {
    fn get_warning_high_red(&self) -> &u8;
}
pub trait GetWarningHighGreen {
    fn get_warning_high_green(&self) -> &u8;
}
pub trait GetWarningHighBlue {
    fn get_warning_high_blue(&self) -> &u8;
}
pub trait GetWarningLowRed {
    fn get_warning_low_red(&self) -> &u8;
}
pub trait GetWarningLowGreen {
    fn get_warning_low_green(&self) -> &u8;
}
pub trait GetWarningLowBlue {
    fn get_warning_low_blue(&self) -> &u8;
}
pub trait GetSuccessRed {
    fn get_success_red(&self) -> &u8;
}
pub trait GetSuccessGreen {
    fn get_success_green(&self) -> &u8;
}
pub trait GetSuccessBlue {
    fn get_success_blue(&self) -> &u8;
}
pub trait GetPartialSuccessRed {
    fn get_partial_success_red(&self) -> &u8;
}
pub trait GetPartialSuccessGreen {
    fn get_partial_success_green(&self) -> &u8;
}
pub trait GetPartialSuccessBlue {
    fn get_partial_success_blue(&self) -> &u8;
}
pub trait GetCleaningRed {
    fn get_cleaning_red(&self) -> &u8;
}
pub trait GetCleaningGreen {
    fn get_cleaning_green(&self) -> &u8;
}
pub trait GetCleaningBlue {
    fn get_cleaning_blue(&self) -> &u8;
}
pub trait GetTimeMeasurementRed {
    fn get_time_measurement_red(&self) -> &u8;
}
pub trait GetTimeMeasurementGreen {
    fn get_time_measurement_green(&self) -> &u8;
}
pub trait GetTimeMeasurementBlue {
    fn get_time_measurement_blue(&self) -> &u8;
}
pub trait GetInfoRed {
    fn get_info_red(&self) -> &u8;
}
pub trait GetInfoGreen {
    fn get_info_green(&self) -> &u8;
}
pub trait GetInfoBlue {
    fn get_info_blue(&self) -> &u8;
}
//FileLineColumn
pub trait GetFile {
    fn get_file(&self) -> &String;
}
pub trait GetLine {
    fn get_line(&self) -> &u32;
}

pub trait GetColumn {
    fn get_column(&self) -> &u32;
}
