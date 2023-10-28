#[derive(Debug)]
pub struct UuidWrapper(sqlx::types::Uuid);

impl UuidWrapper {
    pub fn to_inner(&self) -> &sqlx::types::Uuid {
        &self.0
    }
    pub fn into_inner(self) -> sqlx::types::Uuid {
        self.0
    }
}

impl std::fmt::Display for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<sqlx::types::Uuid> for UuidWrapper {
    fn from(value: sqlx::types::Uuid) -> UuidWrapper {
        Self(value)
    }
}

impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for UuidWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.0.to_string()
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum UuidWrapperTryFromPossibleUuidWrapperErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::TryFrom<PossibleUuidWrapper> for UuidWrapper {
    type Error = UuidWrapperTryFromPossibleUuidWrapperErrorNamed;
    fn try_from(value: PossibleUuidWrapper) -> Result<Self, Self::Error> {
        match sqlx::types::Uuid::parse_str(value.to_inner()) {
            Ok(value) => Ok(UuidWrapper(value)),
            Err(e) => Err(UuidWrapperTryFromPossibleUuidWrapperErrorNamed::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PossibleUuidWrapper(std::string::String);

impl PossibleUuidWrapper {
    pub fn to_inner(&self) -> &std::string::String {
        &self.0
    }
    pub fn into_inner(self) -> std::string::String {
        self.0
    }
}

impl std::fmt::Display for PossibleUuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
