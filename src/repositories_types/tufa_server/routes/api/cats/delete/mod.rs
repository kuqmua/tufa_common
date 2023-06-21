pub mod request;
pub mod route;

#[derive(serde::Deserialize)]
pub struct DeleteQueryParameters {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl std::string::ToString for DeleteQueryParameters {
    fn to_string(&self) -> String {
        match (&self.name, &self.color) {
            (None, None) => String::from(""),
            (None, Some(color)) => format!("color={color}"),
            (Some(name), None) => format!("name={name}"),
            (Some(name), Some(color)) => format!("name={name}&color={color}"),
        }
    }
}
