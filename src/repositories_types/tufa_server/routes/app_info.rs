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

impl<'a> crate::server::routes::git_info::GitInfoRouteParameters<'a> for AppInfo<'a> {}

impl<'a> crate::server::routes::helpers::get_postgres_pool::GetPostgresPool for AppInfo<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}

impl<'a> crate::common::git::project_git_info::GetProjectGitInfo<'a> for AppInfo<'a> {
    fn get_project_git_info(&self) -> &'a crate::common::git::project_git_info::ProjectGitInfo<'a> {
        self.project_git_info
    }
}

impl<'a> crate::common::git::git_info::GetGitInfo<'a> for AppInfo<'a> {
    fn get_git_info(&self) -> &'a crate::common::git::git_info::GitInfo<'a> {
        self.repository_git_info
    }
}
