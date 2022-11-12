use secrecy::Secret;

pub struct PostgresCredentials {
    pub username: String,
    pub password: Secret<String>,
}
