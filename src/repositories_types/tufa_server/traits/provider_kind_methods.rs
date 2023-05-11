pub trait ProviderKindMethods {
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_mongo_log_collection_name(&self, config: & impl crate::traits::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart) -> String;
    fn get_path_to_logs_directory(&self, config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName) -> String;
    fn get_path_to_provider_log_file(
        &self,
        config: &(
            impl crate::traits::config_fields::GetWarningLogsDirectoryName
            + crate::traits::config_fields::GetUnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir
        )
    ) -> String;
    fn get_init_local_data_file_path(
        &self,
        config: &(
            impl crate::traits::config_fields::GetPathToProviderLinkPartsFolder
            + crate::traits::config_fields::GetLogFileExtension
        )
    ) -> String;
    fn remove_logs_directory(&self, config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName) -> Result<(), crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>;
    fn stringify(&self) -> &'static str;
    fn generate_provider_links(
        &self, 
        names_vector: Vec<String>,
        config: &impl crate::traits::config_fields::GetGithubToken
    ) -> Vec<String>;
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers(
        config: &(
            impl crate::traits::config_fields::GetIsEnabledArxiv
            + crate::traits::config_fields::GetIsEnabledBiorxiv
            + crate::traits::config_fields::GetIsEnabledGithub
            + crate::traits::config_fields::GetIsEnabledHabr
            + crate::traits::config_fields::GetIsEnabledMedrxiv
            + crate::traits::config_fields::GetIsEnabledReddit
            + crate::traits::config_fields::GetIsEnabledTwitter
        ) 
    ) -> std::collections::HashMap<Self, Vec<String>>
    where
        Self: Sized;
    fn get_enabled_providers_vec(
        config: &(
            impl crate::traits::config_fields::GetIsEnabledArxiv
            + crate::traits::config_fields::GetIsEnabledBiorxiv
            + crate::traits::config_fields::GetIsEnabledGithub
            + crate::traits::config_fields::GetIsEnabledHabr
            + crate::traits::config_fields::GetIsEnabledMedrxiv
            + crate::traits::config_fields::GetIsEnabledReddit
            + crate::traits::config_fields::GetIsEnabledTwitter
        )
    ) -> Vec<Self>
    where
        Self: Sized;
    fn get_enabled_string_name_vec(
        config: &(
            impl crate::traits::config_fields::GetIsEnabledArxiv
            + crate::traits::config_fields::GetIsEnabledBiorxiv
            + crate::traits::config_fields::GetIsEnabledGithub
            + crate::traits::config_fields::GetIsEnabledHabr
            + crate::traits::config_fields::GetIsEnabledMedrxiv
            + crate::traits::config_fields::GetIsEnabledReddit
            + crate::traits::config_fields::GetIsEnabledTwitter
        )   
    ) -> Vec<String>;
    fn get_mongo_initialization_provider_kind_vec(
        config: &(
            impl crate::traits::config_fields::GetIsMongoInitializationEnabledArxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledBiorxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledGithub
            + crate::traits::config_fields::GetIsMongoInitializationEnabledHabr
            + crate::traits::config_fields::GetIsMongoInitializationEnabledMedrxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledReddit
            + crate::traits::config_fields::GetIsMongoInitializationEnabledTwitter
        )
    ) -> Vec<Self>
    where
        Self: Sized;
    fn get_mongo_initialization_string_name_vec(
        config: &(
            impl crate::traits::config_fields::GetIsMongoInitializationEnabledArxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledBiorxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledGithub
            + crate::traits::config_fields::GetIsMongoInitializationEnabledHabr
            + crate::traits::config_fields::GetIsMongoInitializationEnabledMedrxiv
            + crate::traits::config_fields::GetIsMongoInitializationEnabledReddit
            + crate::traits::config_fields::GetIsMongoInitializationEnabledTwitter
        )
    ) -> Vec<String>;
    fn into_string_name_and_kind_hashmap() -> std::collections::HashMap<String, Self>
    where
        Self: Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, Self)>
    where
        Self: Sized;
    fn remove_existing_providers_logs_directories(
        config: &(
            impl crate::traits::config_fields::GetWarningLogsDirectoryName
    
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledArxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledBiorxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledGithub
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledHabr
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledMedrxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledReddit
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledTwitter
        )
    ) -> Result<(), std::collections::HashMap<Self, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::RemoveDirError>>
    where
        Self: Sized;
    fn remove_providers_logs_directories(
        config: &(
            impl crate::traits::config_fields::GetWarningLogsDirectoryName

            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledArxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledBiorxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledGithub
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledHabr
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledMedrxiv
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledReddit
            + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledTwitter
        )
    ) -> Result<(), std::collections::HashMap<Self, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>>
    where
        Self: Sized;
    fn get_db_tag(&self) -> String
    where
        Self: Sized;
    fn get_postgres_table_name(&self) -> String
    where
        Self: Sized;
    fn get_dbs_initialization_enabled_vec(
        config: &(
            impl crate::traits::config_fields::GetIsDbsInitializationEnabledArxiv
            + crate::traits::config_fields::GetIsDbsInitializationEnabledBiorxiv
            + crate::traits::config_fields::GetIsDbsInitializationEnabledGithub
            + crate::traits::config_fields::GetIsDbsInitializationEnabledHabr
            + crate::traits::config_fields::GetIsDbsInitializationEnabledMedrxiv
            + crate::traits::config_fields::GetIsDbsInitializationEnabledReddit
            + crate::traits::config_fields::GetIsDbsInitializationEnabledTwitter
        )
    ) -> Vec<Self>
    where
        Self: Sized;
}
