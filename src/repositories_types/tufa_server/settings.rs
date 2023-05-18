use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: PostgresDatabaseSettings,
    pub email_client: EmailClientSettings,
    pub redis_uri: secrecy::Secret<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: secrecy::Secret<String>,
    pub timeout_milliseconds: u64,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<crate::repositories_types::tufa_server::domain::SubscriberEmail, String> {
        crate::repositories_types::tufa_server::domain::SubscriberEmail::try_from(self.sender_email.clone())
    }
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
    pub fn client(self) -> crate::repositories_types::tufa_server::email_client::EmailClient {
        let sender_email = self.sender().expect("Invalid sender email address.");
        let timeout = self.timeout();
        crate::repositories_types::tufa_server::email_client::EmailClient::new(
            self.base_url,
            sender_email,
            self.authorization_token,
            timeout,
        )
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PostgresDatabaseSettings {
    pub username: String,
    pub password: secrecy::Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl PostgresDatabaseSettings {
    pub fn with_db(&self) -> sqlx::postgres::PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        {
            use sqlx::ConnectOptions;
            options.log_statements(tracing::log::LevelFilter::Trace)
        };
        options
    }
    pub fn without_db(&self) -> sqlx::postgres::PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            sqlx::postgres::PgSslMode::Require
        } else {
            sqlx::postgres::PgSslMode::Prefer
        };
        sqlx::postgres::PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password({
                use secrecy::ExposeSecret;
                self.password.expose_secret()
            })
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    settings.try_into()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
