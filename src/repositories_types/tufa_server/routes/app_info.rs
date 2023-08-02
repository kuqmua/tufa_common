pub struct AppInfo<'a> {
    pub postgres_pool: sqlx::PgPool,
    pub config: &'a crate::repositories_types::tufa_server::config::config_struct::Config,
    pub project_git_info: &'a crate::common::git::project_git_info::ProjectGitInfo<'a>,
    pub repository_git_info: &'a crate::common::git::git_info::GitInfo<'a>,
}

impl<'a> crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
    for AppInfo<'a>
{
}

impl<'a> crate::repositories_types::tufa_server::config::config_struct::GetConfig for AppInfo<'a> {
    fn get_config(&self) -> &crate::repositories_types::tufa_server::config::config_struct::Config {
        self.config
    }
}
impl<'a> crate::common::config::config_fields::GetSourcePlaceType for AppInfo<'a> {
    fn get_source_place_type(&self) -> &crate::common::source_place_type::SourcePlaceType {
        crate::repositories_types::tufa_server::config::config_struct::GetConfig::get_config(self)
            .get_source_place_type()
    }
}
impl<'a> crate::common::config::config_fields::GetTimezone for AppInfo<'a> {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        crate::repositories_types::tufa_server::config::config_struct::GetConfig::get_config(self)
            .get_timezone()
    }
}

impl<'a> crate::server::routes::git_info::GitInfoRouteParameters for AppInfo<'a> {}
impl<'a> crate::server::routes::not_found::NotFoundRouteParameters for AppInfo<'a> {}

impl<'a> crate::server::routes::helpers::get_postgres_pool::GetPostgresPool for AppInfo<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}

impl<'a> crate::common::git::project_git_info::GetProjectGitCommitLink for AppInfo<'a> {
    fn get_project_git_commit_link(&self) -> std::string::String {
        self.project_git_info.get_project_git_commit_link()
    }
}

impl<'a> crate::common::git::git_fields::GetGitCommitId for AppInfo<'a> {
    fn get_git_commit_id(&self) -> std::string::String {
        self.repository_git_info.get_git_commit_id()
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for AppInfo<'a> {
    fn get_git_commit_link(&self) -> std::string::String {
        self.repository_git_info.get_git_commit_link()
    }
}
