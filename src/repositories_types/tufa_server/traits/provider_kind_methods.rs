pub trait ProviderKindMethods {
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_mongo_log_collection_name(&self, config: & impl crate::traits::fields::GetMongoProvidersLogsDbCollectionHandleSecondPart) -> String;
    fn get_path_to_logs_directory(&self, config: &impl crate::traits::fields::GetWarningLogsDirectoryName) -> String;
    fn get_path_to_provider_log_file(
        &self,
        config: &(
            impl crate::traits::fields::GetWarningLogsDirectoryName
            + crate::traits::fields::GetUnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir
        )
    ) -> String;
    fn get_init_local_data_file_path(
        &self,
        config: &(
            impl crate::traits::fields::GetPathToProviderLinkPartsFolder
            + crate::traits::fields::GetLogFileExtension
        )
    ) -> String;
    fn remove_logs_directory(&self, config: &impl crate::traits::fields::GetWarningLogsDirectoryName) -> Result<(), crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>;
    fn stringify(&self) -> &'static str;
    fn generate_provider_links(
        &self, 
        names_vector: Vec<String>,
        config: &impl crate::traits::fields::GetGithubToken
    ) -> Vec<String>;
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> std::collections::HashMap<Self, Vec<String>>
    where
        Self: Sized;
    fn get_enabled_providers_vec() -> Vec<Self>
    where
        Self: Sized;
    fn get_enabled_string_name_vec() -> Vec<String>;
    fn get_mongo_initialization_provider_kind_vec() -> Vec<Self>
    where
        Self: Sized;
    fn get_mongo_initialization_string_name_vec() -> Vec<String>;
    fn into_string_name_and_kind_hashmap() -> std::collections::HashMap<String, Self>
    where
        Self: Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, Self)>
    where
        Self: Sized;
    fn remove_existing_providers_logs_directories(config: &impl crate::traits::fields::GetWarningLogsDirectoryName) -> Result<(), std::collections::HashMap<Self, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::RemoveDirError>>
    where
        Self: Sized;
    fn remove_providers_logs_directories(config: &impl crate::traits::fields::GetWarningLogsDirectoryName) -> Result<(), std::collections::HashMap<Self, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>>
    where
        Self: Sized;
    fn get_db_tag(&self) -> String
    where
        Self: Sized;
    fn get_postgres_table_name(&self) -> String
    where
        Self: Sized;
    fn get_dbs_initialization_enabled_vec() -> Vec<Self>
    where
        Self: Sized;
}
