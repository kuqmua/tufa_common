#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryBuildActixWebDevServer<'a> {
    // TcpListenerBind {
    //     #[eo_error_occurence]
    //     tcp_listener_bind: crate::traits::try_create_tcp_listener::TryCreateTcpListenerErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    // TcpListenerLocalAddress {
    //     #[eo_display]
    //     tcp_listener_local_address: std::io::Error,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    // NewRedisSessionStore {
    //     #[eo_display_with_serialize_deserialize]
    //     new_redis_session_store: std::string::String,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    HttpServerListen {
        #[eo_display]
        http_server_listen: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub struct AppInfo<'a> {
    pub postgres_pool: sqlx::PgPool,
    pub config: &'a crate::repositories_types::tufa_server::config::config_struct::Config,
    pub project_git_info: &'a crate::common::git::project_git_info::ProjectGitInfo<'a>,
    pub repository_git_info: &'a crate::common::git::git_info::GitInfo<'a>,
}

pub trait GetAppInfo<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
    fn get_config(&self) -> &crate::repositories_types::tufa_server::config::config_struct::Config;
}
impl<'a> GetAppInfo<'a> for AppInfo<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
    fn get_config(&self) -> &crate::repositories_types::tufa_server::config::config_struct::Config {
        self.config
    }
}

impl<'a> crate::server::routes::git_info::GitInfoRouteParameters<'a> for AppInfo<'a> {
    fn get_project_git_info(&self) -> &'a crate::common::git::project_git_info::ProjectGitInfo<'a> {
        self.project_git_info
    }
    fn get_repository_git_info(&self) -> &'a crate::common::git::git_info::GitInfo<'a> {
        self.repository_git_info
    }
}

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

// impl<'a> crate::common::git::git_fields::GetGitCommitId for ProjectGitInfo<'a> {
//     fn get_git_commit_id(&self) -> std::string::String {
//         self.project_commit.to_string()
//     }
// }

// impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for ProjectGitInfo<'a> {
