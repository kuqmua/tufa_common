#[derive(serde::Deserialize)]
pub struct FormData {
    pub title: String,
    pub text_content: String,
    pub html_content: String,
    pub idempotency_key: String,
}
