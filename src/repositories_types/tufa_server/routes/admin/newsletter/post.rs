#[derive(serde::Deserialize)]
pub struct FormData {
    pub title: std::string::String,
    pub text_content: std::string::String,
    pub html_content: std::string::String,
    pub idempotency_key: std::string::String,
}
