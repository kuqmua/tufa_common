//
// #[derive(Debug)]
// pub struct UuidWrapper(sqlx::types::Uuid);
//

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PossibleUuidWrapper(#[serde(deserialize_with = "deserialize_uuid_wrapper")] std::string::String); //sqlx::types::Uuid

// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for PossibleUuidWrapper
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<PossibleUuidWrapper, Box<dyn std::error::Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let str_value = <&str as sqlx::Decode<DB>>::decode(value)?;
        let value = Self::try_from(str_value)?;
        Ok(value)
    }
}

impl PossibleUuidWrapper {
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
    match PossibleUuidWrapper::try_from(value.as_str()) {
        Ok(_) => Ok(value),
        Err(e) => match e {
            PossibleUuidWrapperTryFromStrErrorNamed::NotUuid { not_uuid, code_occurence } => Err(serde::de::Error::custom(&format!(
                "invalid type: Postgresql PossibleUuidWrapper, expected Postgresql Uuid. Error: `{not_uuid}` code_occurence: `{code_occurence}`")
            )),
        },
    }
}

impl std::fmt::Display for PossibleUuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PossibleUuidWrapperTryFromStrErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryFrom<&str> for PossibleUuidWrapper {
    type Error = PossibleUuidWrapperTryFromStrErrorNamed;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match sqlx::types::Uuid::parse_str(value) {
            Ok(_) => Ok(PossibleUuidWrapper(value.to_string())),
            Err(e) => Err(PossibleUuidWrapperTryFromStrErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

impl std::convert::TryFrom<std::string::String> for PossibleUuidWrapper {
    type Error = PossibleUuidWrapperTryFromStrErrorNamed;
    fn try_from(value: std::string::String) -> Result<Self, Self::Error> {
        match sqlx::types::Uuid::parse_str(&value) {
            Ok(_) => Ok(PossibleUuidWrapper(value.to_string())),
            Err(e) => Err(PossibleUuidWrapperTryFromStrErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum PossibleUuidWrapperTryIntoSqlxTypesUuidErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryInto<sqlx::types::Uuid> for PossibleUuidWrapper {
    type Error = PossibleUuidWrapperTryIntoSqlxTypesUuidErrorNamed;
    fn try_into(self) -> Result<sqlx::types::Uuid, Self::Error> {
        match sqlx::types::Uuid::parse_str(self.to_inner()) {
            Ok(value) => Ok(value),
            Err(e) => Err(PossibleUuidWrapperTryIntoSqlxTypesUuidErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

impl std::convert::From<sqlx::types::Uuid> for PossibleUuidWrapper {
    fn from(value: sqlx::types::Uuid) -> PossibleUuidWrapper {
        Self(value.to_string())
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for PossibleUuidWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.0
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for Vec<PossibleUuidWrapper> {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let mut value = std::string::String::from("");
        for element in self {
            value.push_str(&format!("{element},"));
        }
        value.pop();
        value
    }
}
