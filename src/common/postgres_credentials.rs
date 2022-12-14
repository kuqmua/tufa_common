pub struct PostgresCredentials {
    pub username: String,
    pub password: secrecy::Secret<String>,
}
