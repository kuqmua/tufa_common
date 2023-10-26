#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UuidWrapper(#[serde(deserialize_with = "deserialize_uuid_wrapper")] std::string::String); //sqlx::types::Uuid

// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for UuidWrapper
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<UuidWrapper, Box<dyn std::error::Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let str_value = <&str as sqlx::Decode<DB>>::decode(value)?;
        let value = Self::try_from(str_value)?;
        Ok(value)
    }
}

impl UuidWrapper {
    pub fn to_inner(&self) -> &std::string::String {
        &self.0
    }
    pub fn into_inner(self) -> std::string::String {
        self.0
    }
}

fn deserialize_uuid_wrapper<'de, D>(deserializer: D) -> Result<std::string::String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;
    let value = std::string::String::deserialize(deserializer)?;
    match UuidWrapper::try_from(value.as_str()) {
        Ok(_) => Ok(value),
        Err(e) => match e {
            UuidWrapperTryFromStrErrorNamed::NotUuid { not_uuid, code_occurence } => Err(serde::de::Error::custom(&format!(
                "invalid type: Postgresql UuidWrapper, expected Postgresql Uuid. Error: `{not_uuid}` code_occurence: `{code_occurence}`")
            )),
        },
    }
}

impl std::fmt::Display for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum UuidWrapperTryFromStrErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryFrom<&str> for UuidWrapper {
    type Error = UuidWrapperTryFromStrErrorNamed;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match sqlx::types::Uuid::parse_str(value) {
            Ok(_) => Ok(UuidWrapper(value.to_string())),
            Err(e) => Err(UuidWrapperTryFromStrErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

impl std::convert::TryFrom<std::string::String> for UuidWrapper {
    type Error = UuidWrapperTryFromStrErrorNamed;
    fn try_from(value: std::string::String) -> Result<Self, Self::Error> {
        match sqlx::types::Uuid::parse_str(&value) {
            Ok(_) => Ok(UuidWrapper(value.to_string())),
            Err(e) => Err(UuidWrapperTryFromStrErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum UuidWrapperTryIntoSqlxTypesUuidErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryInto<sqlx::types::Uuid> for UuidWrapper {
    type Error = UuidWrapperTryIntoSqlxTypesUuidErrorNamed;
    fn try_into(self) -> Result<sqlx::types::Uuid, Self::Error> {
        match sqlx::types::Uuid::parse_str(self.to_inner()) {
            Ok(value) => Ok(value),
            Err(e) => Err(UuidWrapperTryIntoSqlxTypesUuidErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

impl std::convert::From<sqlx::types::Uuid> for UuidWrapper {
    fn from(value: sqlx::types::Uuid) -> UuidWrapper {
        Self(value.to_string())
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for UuidWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.0
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for Vec<UuidWrapper> {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let mut value = std::string::String::from("");
        for element in self {
            value.push_str(&format!("{element},"));
        }
        value.pop();
        value
    }
}
