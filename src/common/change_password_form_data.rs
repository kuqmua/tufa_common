#[derive(serde::Deserialize)]
pub struct ChangePasswordFormData {
    pub current_password: secrecy::Secret<String>,
    pub new_password: secrecy::Secret<String>,
    pub new_password_check: secrecy::Secret<String>,
}
