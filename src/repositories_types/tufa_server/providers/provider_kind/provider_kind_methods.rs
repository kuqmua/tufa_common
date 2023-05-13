impl crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods for crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind {
    fn get_item_handle(&self) -> Option<&'static str> {
        match self {
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Arxiv => Some("</item>"),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Biorxiv => Some("</item>"),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Github => Some("</entry>"),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Habr => Some("</item>"),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Medrxiv => Some("</item>"),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Reddit => None,
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Twitter => Some("</item>"),
        }
    }
    fn get_mongo_log_collection_name(
        &self,
        config: & impl crate::traits::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart 
    ) -> String {
        format!(
            "{self}{}",
            config.get_mongo_providers_logs_db_collection_handle_second_part()//todo rename it into db log collection
        )
    }
    fn get_path_to_logs_directory(
        &self,
        config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName
    ) -> String {
        format!("logs/{}/{self:?}", config.get_warning_logs_directory_name())
    }
    fn get_path_to_provider_log_file(
        &self,
        config: &(
            impl crate::traits::config_fields::GetWarningLogsDirectoryName
            + crate::traits::config_fields::GetUnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir
        )
    ) -> String {
        format!(
            "logs/{}/{self:?}/{}",
            config.get_warning_logs_directory_name(),
            config.get_unhandled_success_handled_success_are_there_items_initialized_posts_dir()
        )
    }
    fn get_init_local_data_file_path(
        &self,
        config: &(
            impl crate::traits::config_fields::GetPathToProviderLinkPartsFolder
            + crate::traits::config_fields::GetLogFileExtension
        )
    ) -> String {
        format!(
            "{}{self}_link_parts{}",
            config.get_path_to_provider_link_parts_folder(),
            config.get_log_file_extension()
        )
    }
    fn remove_logs_directory(
        &self,
        config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName
    ) -> Result<(), crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError> {
        let path = self.get_path_to_logs_directory(config);
        if !std::path::Path::new(&path).is_dir() {
            return Err(crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError::PathIsNotDir { path });
        }
        std::fs::remove_dir_all(&path)?; //todo: its blocking, rewrite to async //update: also its has vulnerability https://blog.rust-lang.org/2022/01/20/cve-2022-21658.html
        Ok(())
    }
    fn stringify(&self) -> &'static str {
        match self {
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Biorxiv => stringify!(ProviderKind::Biorxiv),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Github => stringify!(ProviderKind::Github),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Habr => stringify!(ProviderKind::Habr),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Medrxiv => stringify!(ProviderKind::Medrxiv),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Reddit => stringify!(ProviderKind::Reddit),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Twitter => stringify!(ProviderKind::Twitter),
        }
    }
    fn generate_provider_links(
        &self, 
        names_vector: Vec<String>,
        config: &impl crate::traits::config_fields::GetGithubToken
    ) -> Vec<String> {
        match self {
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Arxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links(names_vector),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Biorxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links(names_vector),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Github => crate::repositories_types::tufa_server::providers::providers_info::links::generate_github_links::generate_github_links(names_vector, config),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Habr => crate::repositories_types::tufa_server::providers::providers_info::links::generate_habr_links::generate_habr_links(names_vector),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Medrxiv => crate::repositories_types::tufa_server::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links(names_vector),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Reddit => crate::repositories_types::tufa_server::providers::providers_info::links::generate_reddit_links::generate_reddit_links(names_vector),
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::Twitter => crate::repositories_types::tufa_server::providers::providers_info::links::generate_twitter_links::generate_twitter_links(names_vector),
        }
    }
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> {
        crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec()
            .iter()
            .map(|pk| (*pk, Vec::<String>::new()))
            .collect()
    }
    fn get_enabled_providers_vec() -> Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }.filter(|pk| {
            // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
            // *pk.is_enabled()
            false
        }).collect()
    }
    fn get_enabled_string_name_vec() -> Vec<String> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .filter_map(|pk| {
                if {
                    // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
                    // *pk.is_enabled()
                    false
                } {
                    return Some(format!("{pk}"));
                }
                None
            })
            .collect()
    }
    fn get_mongo_initialization_provider_kind_vec() -> Vec<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .filter(|pk| {
                // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
                // *pk.is_mongo_initialization_enabled()
                false
            })
            .collect()
    }
    fn get_mongo_initialization_string_name_vec() -> Vec<String> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .filter_map(|pk| {
                // if {
                //     use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
                //     *pk.is_mongo_initialization_enabled(config)
                // } {
                //     return Some(format!("{pk}"));
                // }
                None
            })
            .collect()
    }
    fn into_string_name_and_kind_hashmap() -> std::collections::HashMap<String, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .map(|pk| (format!("{pk}"), pk))
            .collect()
    }
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind)> {
        {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .map(|pk| (format!("{pk}"), pk))
            .collect()
    }
    fn remove_existing_providers_logs_directories(
        config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName
    ) -> Result<(), std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::RemoveDirError>> {
        if let Err(error_hashmap) = crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::remove_providers_logs_directories(config) {
            let return_hashmap = error_hashmap
                .into_iter()
                .filter_map(|(pk, error)| {
                    if let crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError::CannotRemoveDir { error: e } = error {
                        return Some((pk, e));
                    }
                    None
                })
                .collect::<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::RemoveDirError>>();
            if !return_hashmap.is_empty() {
                return Err(return_hashmap);
            }
            return Ok(());
        }
        Ok(())
    }
    fn remove_providers_logs_directories(config: &impl crate::traits::config_fields::GetWarningLogsDirectoryName) -> Result<(), std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>> {
        let result_hashmap = {
            use strum::IntoEnumIterator;
            crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::iter()
        }
            .filter_map(|pk| {
                if {
                    // use crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
                    // *pk.is_cleaning_warning_logs_directory_enabled(config)
                    false
                } {
                    if let Err(e) = pk.remove_logs_directory(config) {
                        return Some((pk, e));
                    }
                }
                None
            })
            .collect::<std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::CleanLogsDirError>>();
        if !result_hashmap.is_empty() {
            return Err(result_hashmap);
        }
        Ok(())
    }
    fn get_db_tag(&self) -> String {
        format!("{self}")
    }
    fn get_postgres_table_name(&self) -> String {
        format!("{}_link_parts", self.to_lower_snake_case())
    }
}
