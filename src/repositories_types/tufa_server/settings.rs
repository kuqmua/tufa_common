use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: PostgresDatabaseSettings,
    pub redis_uri: secrecy::Secret<String>,
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
