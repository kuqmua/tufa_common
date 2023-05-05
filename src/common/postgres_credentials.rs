pub struct PostgresCredentials {
    pub username: std::string::String,
    pub password: secrecy::Secret<String>,
}
