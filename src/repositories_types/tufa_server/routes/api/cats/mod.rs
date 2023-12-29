//todo openapi
//todo test if create\update\delete empty array
pub trait GetConfigGetPostgresPool:
    crate::repositories_types::tufa_server::config::config_struct::GetConfig
    + crate::server::routes::helpers::get_postgres_pool::GetPostgresPool
    + crate::common::config::config_fields::GetSourcePlaceType
    + crate::common::config::config_fields::GetTimezone
{
}

pub type DynArcGetConfigGetPostgresPoolSendSync = std::sync::Arc<
    dyn crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
        + Send
        + Sync,
>;

#[derive(
    Debug,
    // generate_postgresql_crud::GeneratePostgresqlCrud,
)]
// #[generate_postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::read_many_with_body_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
// #[generate_postgresql_crud::delete_many_with_body_additional_http_status_codes_error_variants{}]

// #[generate_postgresql_crud::additional_http_status_codes_error_variants{
//     #[path(crate::server::extractors::project_commit_extractor::)]
//     enum ProjectCommitExtractorCheckErrorNamed {
//         #[tvfrr_400_bad_request]
//         ProjectCommitExtractorNotEqual {
//             #[eo_display_with_serialize_deserialize]
//             project_commit_not_equal: std::string::String,
//             #[eo_display_with_serialize_deserialize]
//             project_commit_to_use: std::string::String,
//             code_occurence: crate::common::code_occurence::CodeOccurence,
//         },
//         #[tvfrr_400_bad_request]
//         ProjectCommitExtractorToStrConversion {
//             #[eo_display]
//             project_commit_to_str_conversion: http::header::ToStrError,
//             code_occurence: crate::common::code_occurence::CodeOccurence,
//         },
//         #[tvfrr_400_bad_request]
//         NoProjectCommitExtractorHeader {
//             #[eo_display_with_serialize_deserialize]
//             no_project_commit_header: std::string::String,
//             code_occurence: crate::common::code_occurence::CodeOccurence,
//         },
//     }
//     // ;
//     // enum SomethingErrorNamed {
//     //     #[tvfrr_400_bad_request]
//     //     SomethingVariant {
//     //         #[eo_display_with_serialize_deserialize]
//     //         something_field: std::string::String,
//     //         code_occurence: crate::common::code_occurence::CodeOccurence,
//     //     },
//     // }
// }]
pub struct Dog {
    // #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    // #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    // #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}

// fn s() {
//     let f = crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader {
//             no_project_commit_header: std::string::String::from(""),
//             code_occurence: crate::code_occurence_tufa_common!(),
//         };
// }

pub const TABLE_NAME: &str = "dogs";
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: std::option::Option<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: std::option::Option<std::string::String>,
}
impl std::convert::From<Dog> for DogOptions {
    fn from(value: Dog) -> Self {
        Self {
            id: Some(value.id.into()),
            name: Some(value.name.into()),
            color: Some(value.color.into()),
        }
    }
}
#[derive(Debug)]
pub struct DogId {
    pub id: sqlx::types::Uuid,
}
#[derive(Debug)]
pub struct DogName {
    pub name: std::string::String,
}
#[derive(Debug)]
pub struct DogColor {
    pub color: std::string::String,
}
#[derive(Debug)]
pub struct DogIdName {
    pub id: sqlx::types::Uuid,
    pub name: std::string::String,
}
#[derive(Debug)]
pub struct DogIdColor {
    pub id: sqlx::types::Uuid,
    pub color: std::string::String,
}
#[derive(Debug)]
pub struct DogNameColor {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug)]
pub struct DogIdNameColor {
    pub id: sqlx::types::Uuid,
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogIdTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogId {
    type Error = DogIdTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id = match value.id {
            Some(value) => {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    Ok(value) => value.into_inner(),
                    Err(e) => {
                        return Err(Self::Error::UuidWrapperTryFromPossibleUuidWrapper {
                            uuid_wrapper_try_from_possible_uuid_wrapper: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                }
            }
            None => {
                return Err(Self::Error::IdIsNone {
                    id_is_none: std::string::String::from("id is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogNameTryFromDogOptionsErrorNamed {
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogName {
    type Error = DogNameTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let name = match value.name {
            Some(value) => value,
            None => {
                return Err(Self::Error::NameIsNone {
                    name_is_none: std::string::String::from("name is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { name })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogColorTryFromDogOptionsErrorNamed {
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogColor {
    type Error = DogColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let color = match value.color {
            Some(value) => value,
            None => {
                return Err(Self::Error::ColorIsNone {
                    color_is_none: std::string::String::from("color is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogIdNameTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdName {
    type Error = DogIdNameTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id = match value.id {
            Some(value) => {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    Ok(value) => value.into_inner(),
                    Err(e) => {
                        return Err(Self::Error::UuidWrapperTryFromPossibleUuidWrapper {
                            uuid_wrapper_try_from_possible_uuid_wrapper: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                }
            }
            None => {
                return Err(Self::Error::IdIsNone {
                    id_is_none: std::string::String::from("id is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let name = match value.name {
            Some(value) => value,
            None => {
                return Err(Self::Error::NameIsNone {
                    name_is_none: std::string::String::from("name is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id, name })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogIdColorTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdColor {
    type Error = DogIdColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id = match value.id {
            Some(value) => {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    Ok(value) => value.into_inner(),
                    Err(e) => {
                        return Err(Self::Error::UuidWrapperTryFromPossibleUuidWrapper {
                            uuid_wrapper_try_from_possible_uuid_wrapper: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                }
            }
            None => {
                return Err(Self::Error::IdIsNone {
                    id_is_none: std::string::String::from("id is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let color = match value.color {
            Some(value) => value,
            None => {
                return Err(Self::Error::ColorIsNone {
                    color_is_none: std::string::String::from("color is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogNameColorTryFromDogOptionsErrorNamed {
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogNameColor {
    type Error = DogNameColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let name = match value.name {
            Some(value) => value,
            None => {
                return Err(Self::Error::NameIsNone {
                    name_is_none: std::string::String::from("name is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let color = match value.color {
            Some(value) => value,
            None => {
                return Err(Self::Error::ColorIsNone {
                    color_is_none: std::string::String::from("color is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { name, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogIdNameColorTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdNameColor {
    type Error = DogIdNameColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id = match value.id {
            Some(value) => {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    Ok(value) => value.into_inner(),
                    Err(e) => {
                        return Err(Self::Error::UuidWrapperTryFromPossibleUuidWrapper {
                            uuid_wrapper_try_from_possible_uuid_wrapper: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                }
            }
            None => {
                return Err(Self::Error::IdIsNone {
                    id_is_none: std::string::String::from("id is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let name = match value.name {
            Some(value) => value,
            None => {
                return Err(Self::Error::NameIsNone {
                    name_is_none: std::string::String::from("name is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let color = match value.color {
            Some(value) => value,
            None => {
                return Err(Self::Error::ColorIsNone {
                    color_is_none: std::string::String::from("color is None"),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id, name, color })
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension ::
EnumExtension,
    strum_macros :: EnumIter,
    PartialEq,
    Eq,
    from_str :: FromStr,
)]
pub enum DogColumn {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
}
impl std::fmt::Display for DogColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::to_lower_snake_case(self))
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    Clone,
    strum_macros
:: Display,
)]
pub enum DogColumnSelect {
    Id,
    Name,
    Color,
    IdName,
    IdColor,
    NameColor,
    IdNameColor,
}
impl crate::server::postgres::generate_query::GenerateQuery for DogColumnSelect {
    fn generate_query(&self) -> std::string::String {
        match self {
            Self::Id => std::string::String::from("id"),
            Self::Name => std::string::String::from("name"),
            Self::Color => std::string::String::from("color"),
            Self::IdName => std::string::String::from("id,name"),
            Self::IdColor => std::string::String::from("id,color"),
            Self::NameColor => std::string::String::from("name,color"),
            Self::IdNameColor => std::string::String::from("id,name,color"),
        }
    }
}
impl std::default::Default for DogColumnSelect {
    fn default() -> Self {
        Self::IdNameColor
    }
}
impl std::convert::From<std::option::Option<Self>> for DogColumnSelect {
    fn from(option_value: std::option::Option<Self>) -> Self {
        match option_value {
            Some(value) => value,
            None => Self::default(),
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogColumnSelectFromStrErrorNamed {
    NotCorrect {
        #[eo_display_with_serialize_deserialize]
        not_correct_value: std::string::String,
        #[eo_display_with_serialize_deserialize]
        supported_values: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::str::FromStr for DogColumnSelect {
    type Err = DogColumnSelectFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value
        {
            "Id" => Ok(Self :: Id), "Name" => Ok(Self :: Name), "Color" =>
            Ok(Self :: Color), "IdName" => Ok(Self :: IdName), "IdColor" =>
            Ok(Self :: IdColor), "NameColor" => Ok(Self :: NameColor),
            "IdNameColor" => Ok(Self :: IdNameColor), _ =>
            Err(Self :: Err :: NotCorrect
            {
                not_correct_value : std :: string :: String :: from(value),
                supported_values : std :: string :: String ::
                from("\"Id\",\"Name\",\"Color\",\"IdName\",\"IdColor\",\"NameColor\",\"IdNameColor\""),
                code_occurence : crate :: code_occurence_tufa_common! (),
            }),
        }
    }
}
impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogColumnSelect {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.to_string()
    }
}
impl DogColumnSelect {
    fn options_try_from_sqlx_row<'a, R: sqlx::Row>(&self, row: &'a R) -> sqlx::Result<DogOptions>
    where
        &'a std::primitive::str: sqlx::ColumnIndex<R>,
        std::option::Option<sqlx::types::Uuid>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<sqlx::types::Uuid>: sqlx::types::Type<R::Database>,
        std::option::Option<std::string::String>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::string::String>: sqlx::types::Type<R::Database>,
        std::option::Option<std::string::String>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::string::String>: sqlx::types::Type<R::Database>,
    {
        let mut id: std::option::Option<
            crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
        > = None;
        let mut name: std::option::Option<std::string::String> = None;
        let mut color: std::option::Option<std::string::String> = None;
        match self {
            Self::Id => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Self::Name => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = row.try_get("name")?;
            }
            Self::Color => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                color = row.try_get("color")?;
            }
            Self::IdName => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = row.try_get("name")?;
            }
            Self::IdColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                color = row.try_get("color")?;
            }
            Self::NameColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
            Self::IdNameColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => option_primary_key.map(|value| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value)
                    }),
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = row.try_get("name")?;
                color = row.try_get("color")?;
            }
        }
        Ok(DogOptions { id, name, color })
    }
}
fn primary_key_uuid_wrapper_try_from_sqlx_row<'a, R: sqlx::Row>(
    row: &'a R,
) -> sqlx::Result<crate::server::postgres::uuid_wrapper::UuidWrapper>
where
    &'a std::primitive::str: sqlx::ColumnIndex<R>,
    sqlx::types::Uuid: sqlx::decode::Decode<'a, R::Database>,
    sqlx::types::Uuid: sqlx::types::Type<R::Database>,
{
    let primary_key: sqlx::types::Uuid = row.try_get("id")?;
    Ok(crate::server::postgres::uuid_wrapper::UuidWrapper::from(
        primary_key,
    ))
}
fn deserialize_dog_order_by<'de, D>(
    deserializer: D,
) -> Result<crate::server::postgres::order_by::OrderBy<DogColumn>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        String::deserialize(deserializer)?
    };
    let split_inner_url_parameters_symbol = ',';
    let default_message = format!("Invalid DogOrderBy:");
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_column) => match {
                                use std::str::FromStr;
                                DogColumn::from_str(possible_column)
                            } {
                                Ok(column) => column,
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {column_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_column) => match {
                            use std::str::FromStr;
                            DogColumn::from_str(possible_column)
                        } {
                            Ok(column) => column,
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return
                            Err(serde :: de :: Error ::
                            custom(& format!
                            ("{default_message} {column_equal_str} failed to offset_slice.get(0..)")))
                            ;
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {column_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {column_equal_str} index overflow"
                )));
            }
        },
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} {column_equal_str} not found"
            )));
        }
    };
    let order = match string_deserialized.find(order_equal_str) {
        Some(index) => match index.checked_add(order_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_order) => match {
                                use std::str::FromStr;
                                crate::server::postgres::order::Order::from_str(possible_order)
                            } {
                                Ok(order) => Some(order),
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {order_equal_str} {e}"
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_order) => match {
                            use std::str::FromStr;
                            crate::server::postgres::order::Order::from_str(possible_order)
                        } {
                            Ok(order) => Some(order),
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} {e}"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(
                                &format!
                            ("{default_message} {order_equal_str} failed to offset_slice.get(0..)"),
                            ));
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {order_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {order_equal_str} index overflow"
                )));
            }
        },
        None => None,
    };
    Ok(crate::server::postgres::order_by::OrderBy { column, order })
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOrderByWrapper(
    #[serde(deserialize_with = "deserialize_dog_order_by")]
    pub  crate::server::postgres::order_by::OrderBy<DogColumn>,
);
impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogOrderByWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.unwrap_or_default();
        format!("column={column},order={order}")
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DogOrderByWrapperFromStrErrorNamed {
    ColumnFromStr {
        #[eo_display_with_serialize_deserialize]
        column_from_str: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNoOffsetValue {
        #[eo_display_with_serialize_deserialize]
        column_no_offset_value: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnOffsetSliceGet {
        #[eo_display_with_serialize_deserialize]
        column_offset_slice_get: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedGet {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_get: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        column_index_checked_add: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedFind {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_find: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OrderFromStr {
        #[eo_display_with_serialize_deserialize]
        order_from_str: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OrderOffsetSliceGetNone {
        #[eo_display_with_serialize_deserialize]
        order_offset_slice_get_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OrderStringDeserializedGetNone {
        #[eo_display_with_serialize_deserialize]
        order_string_deserialized_get_none: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OrderIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        order_index_checked_add: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::str::FromStr for DogOrderByWrapper {
    type Err = DogOrderByWrapperFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let string_deserialized = value.to_string();
        let split_inner_url_parameters_symbol = ',';
        let default_message = format!("Invalid DogOrderBy:");
        let column_equal_str = "column=";
        let order_equal_str = "order=";
        let column = match string_deserialized.find(column_equal_str) {
            Some(index) => match index.checked_add(column_equal_str.len()) {
                Some(offset) => match string_deserialized.get(offset..) {
                    Some(offset_slice) => {
                        match offset_slice.find(split_inner_url_parameters_symbol) {
                            Some(offset_slice_next_comma_index) => {
                                match offset_slice.get(0..offset_slice_next_comma_index) {
                                    Some(possible_column) => {
                                        match DogColumn::from_str(possible_column) {
                                            Ok(column) => column,
                                            Err(e) => {
                                                return Err(Self::Err::ColumnFromStr {
                                                    column_from_str: e,
                                                    code_occurence: crate::code_occurence_tufa_common!(),
                                                });
                                            }
                                        }
                                    }
                                    None => {
                                        return Err(Self::Err::ColumnNoOffsetValue {
                                            column_no_offset_value: std::string::String::from(
                                                "no offset value",
                                            ),
                                            code_occurence: crate::code_occurence_tufa_common!(),
                                        });
                                    }
                                }
                            }
                            None => match offset_slice.get(0..) {
                                Some(possible_column) => match DogColumn::from_str(possible_column)
                                {
                                    Ok(column) => column,
                                    Err(e) => {
                                        return Err(Self::Err::ColumnFromStr {
                                            column_from_str: e,
                                            code_occurence: crate::code_occurence_tufa_common!(),
                                        });
                                    }
                                },
                                None => {
                                    return Err(Self::Err::ColumnOffsetSliceGet {
                                        column_offset_slice_get: std::string::String::from(
                                            "offset_slice_get",
                                        ),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    });
                                }
                            },
                        }
                    }
                    None => {
                        return Err(Self::Err::ColumnStringDeserializedGet {
                            column_string_deserialized_get: std::string::String::from(
                                "string_deserialized_get",
                            ),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                },
                None => {
                    return Err(Self::Err::ColumnIndexCheckedAdd {
                        column_index_checked_add: std::string::String::from("index_checked_add"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    });
                }
            },
            None => {
                return Err(Self::Err::ColumnStringDeserializedFind {
                    column_string_deserialized_find: std::string::String::from(
                        "string_deserialized_find",
                    ),
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let order = match string_deserialized.find(order_equal_str) {
            Some(index) => match index.checked_add(order_equal_str.len()) {
                Some(offset) => match string_deserialized.get(offset..) {
                    Some(offset_slice) => {
                        match offset_slice.find(split_inner_url_parameters_symbol) {
                            Some(offset_slice_next_comma_index) => {
                                match offset_slice.get(0..offset_slice_next_comma_index) {
                                    Some(possible_order) => {
                                        match crate::server::postgres::order::Order::from_str(
                                            possible_order,
                                        ) {
                                            Ok(order) => Some(order),
                                            Err(e) => {
                                                return Err(Self::Err::OrderFromStr {
                                                    order_from_str: e,
                                                    code_occurence: crate::code_occurence_tufa_common!(),
                                                });
                                            }
                                        }
                                    }
                                    None => {
                                        return Err(Self::Err::OrderOffsetSliceGetNone {
                                            order_offset_slice_get_none: std::string::String::from(
                                                "order_offset_slice_get_none",
                                            ),
                                            code_occurence: crate::code_occurence_tufa_common!(),
                                        });
                                    }
                                }
                            }
                            None => match offset_slice.get(0..) {
                                Some(possible_order) => {
                                    match crate::server::postgres::order::Order::from_str(
                                        possible_order,
                                    ) {
                                        Ok(order) => Some(order),
                                        Err(e) => {
                                            return Err(Self::Err::OrderFromStr {
                                                order_from_str: e,
                                                code_occurence: crate::code_occurence_tufa_common!(),
                                            });
                                        }
                                    }
                                }
                                None => {
                                    return Err(Self::Err::OrderOffsetSliceGetNone {
                                        order_offset_slice_get_none: std::string::String::from(
                                            "order_offset_slice_get_none",
                                        ),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    });
                                }
                            },
                        }
                    }
                    None => {
                        return Err(Self::Err::OrderStringDeserializedGetNone {
                            order_string_deserialized_get_none: std::string::String::from(
                                "string_deserialized_get_none",
                            ),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        });
                    }
                },
                None => {
                    return Err(Self::Err::OrderIndexCheckedAdd {
                        order_index_checked_add: std::string::String::from(
                            "order_index_checked_add",
                        ),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    });
                }
            },
            None => None,
        };
        Ok(Self(crate::server::postgres::order_by::OrderBy {
            column,
            order,
        }))
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [
    http::Method::GET,
    http::Method::POST,
    http::Method::PATCH,
    http::Method::DELETE,
];
pub struct DogColumnReadPermission {
    id: bool,
    name: bool,
    color: bool,
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        async fn find_out_if_it_works() {
            let api_location = std::string::String::from("http://127.0.0.1:8080/api");
            let limit = 1000;
            let offset = 0;
            println!("--------------try_create_one-----------------");
            let id = match crate :: repositories_types :: tufa_server ::
            routes :: api :: cats ::
            try_create_one(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: CreateOneParameters
            {
                payload : crate :: repositories_types :: tufa_server :: routes
                :: api :: cats :: CreateOnePayload
                {
                    name : String :: from("try_create_one_name"), color : String
                    :: from("try_create_one_color"),
                }
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; value }, Err(e) =>
                { panic! ("{e}") ; }
            } ;
            println!("--------------try_read_one-----------------");
            match crate :: repositories_types :: tufa_server :: routes :: api
            :: cats ::
            try_read_one(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: ReadOneParameters
            {
                path : crate :: repositories_types :: tufa_server :: routes ::
                api :: cats :: ReadOnePath { id : id.clone() }, query : crate
                :: repositories_types :: tufa_server :: routes :: api :: cats
                :: ReadOneQuery
                {
                    select :
                    Some(crate :: repositories_types :: tufa_server :: routes ::
                    api :: cats :: DogColumnSelect :: IdNameColor)
                }
            },).await
            {
                Ok(value) => println! ("{value:#?}"), Err(e) =>
                { panic! ("{e}") ; }
            }
            println!("--------------try_update_one------------------");
            let id = match crate :: repositories_types :: tufa_server ::
            routes :: api :: cats ::
            try_update_one(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: UpdateOneParameters
            {
                path : crate :: repositories_types :: tufa_server :: routes ::
                api :: cats :: UpdateOnePath { id : id.clone() }, payload :
                crate :: repositories_types :: tufa_server :: routes :: api ::
                cats :: UpdateOnePayload
                {
                    name : Some(std :: string :: String :: from("name")), color
                    : Some(std :: string :: String :: from("color")),
                }
            }).await
            {
                Ok(value) => { println! ("{value:#?}") ; value }, Err(e) =>
                panic! ("{e}"),
            } ;
            println!("--------------try_read_one-----------------");
            match crate :: repositories_types :: tufa_server :: routes :: api
            :: cats ::
            try_read_one(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: ReadOneParameters
            {
                path : crate :: repositories_types :: tufa_server :: routes ::
                api :: cats :: ReadOnePath { id : id.clone() }, query : crate
                :: repositories_types :: tufa_server :: routes :: api :: cats
                :: ReadOneQuery
                {
                    select :
                    Some(crate :: repositories_types :: tufa_server :: routes ::
                    api :: cats :: DogColumnSelect :: IdNameColor)
                }
            },).await
            {
                Ok(value) => println! ("{value:#?}"), Err(e) =>
                { panic! ("{e}") ; }
            }
            println!("--------------try_delete_one------------------");
            match crate::repositories_types::tufa_server::routes::api::cats::try_delete_one(
                &api_location,
                crate::repositories_types::tufa_server::routes::api::cats::DeleteOneParameters {
                    path:
                        crate::repositories_types::tufa_server::routes::api::cats::DeleteOnePath {
                            id: id.clone(),
                        },
                },
            )
            .await
            {
                Ok(value) => println!("{value:#?}"),
                Err(e) => panic!("{e}"),
            }
            println!("--------------try_read_one-----------------");
            match
            crate :: repositories_types :: tufa_server :: routes :: api ::
            cats ::
            try_read_one(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: ReadOneParameters
            {
                path : crate :: repositories_types :: tufa_server :: routes ::
                api :: cats :: ReadOnePath { id }, query : crate ::
                repositories_types :: tufa_server :: routes :: api :: cats ::
                ReadOneQuery
                {
                    select :
                    Some(crate :: repositories_types :: tufa_server :: routes ::
                    api :: cats :: DogColumnSelect :: IdNameColor)
                }
            },).await
            {
                Ok(value) => println! ("{value:#?}"), Err(e) =>
                { println! ("{e}") ; }
            }
            println!("--------------try_create_many-----------------");
            let ids = match crate :: repositories_types :: tufa_server ::
            routes :: api :: cats ::
            try_create_many(& api_location, crate :: repositories_types ::
            tufa_server :: routes :: api :: cats :: CreateManyParameters
            {
                payload : vec!
                [crate :: repositories_types :: tufa_server :: routes :: api
                :: cats :: CreateManyPayloadElement
                {
                    name : String :: from("try_create_many_name1"), color :
                    String :: from("try_create_many_color1"),
                }, crate :: repositories_types :: tufa_server :: routes :: api
                :: cats :: CreateManyPayloadElement
                {
                    name : String :: from("try_create_many_name2"), color :
                    String :: from("try_create_many_color2"),
                },]
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; value }, Err(e) =>
                { panic! ("{e}") ; }
            } ;
            println!("--------------try_read_many_with_body-----------------");
            match
            crate :: repositories_types :: tufa_server :: routes :: api ::
            cats ::
            try_read_many_with_body(& api_location, crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            ReadManyWithBodyParameters
            {
                payload : crate :: repositories_types :: tufa_server :: routes
                :: api :: cats :: ReadManyWithBodyPayload
                {
                    select : crate :: repositories_types :: tufa_server ::
                    routes :: api :: cats :: DogColumnSelect :: IdNameColor, id
                    : Some(ids.clone()), name : None, color : None, order_by :
                    crate :: server :: postgres :: order_by :: OrderBy
                    {
                        column : crate :: repositories_types :: tufa_server ::
                        routes :: api :: cats :: DogColumn :: Name, order :
                        Some(crate :: server :: postgres :: order :: Order :: Desc),
                    }, limit : crate :: server :: postgres :: postgres_bigint ::
                    PostgresBigint(limit), offset : crate :: server :: postgres
                    :: postgres_bigint :: PostgresBigint(offset),
                }
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; } Err(e) =>
                { panic! ("{e}") ; }
            }
            println!("--------------try_update_many------------------");
            match crate::repositories_types::tufa_server::routes::api::cats::try_update_many(
                &api_location,
                crate::repositories_types::tufa_server::routes::api::cats::UpdateManyParameters {
                    payload: ids
                        .clone()
                        .into_iter()
                        .map(|element| {
                            crate :: repositories_types :: tufa_server :: routes :: api
                    :: cats :: UpdateManyPayloadElement
                    {
                        id : element, name : std :: string :: String ::
                        from("name"), color : std :: string :: String ::
                        from("color"),
                    }
                        })
                        .collect(),
                },
            )
            .await
            {
                Ok(value) => println!("{value:#?}"),
                Err(e) => {
                    panic!("{e}");
                }
            }
            println!("--------------try_read_many_with_body-----------------");
            match
            crate :: repositories_types :: tufa_server :: routes :: api ::
            cats ::
            try_read_many_with_body(& api_location, crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            ReadManyWithBodyParameters
            {
                payload : crate :: repositories_types :: tufa_server :: routes
                :: api :: cats :: ReadManyWithBodyPayload
                {
                    select : crate :: repositories_types :: tufa_server ::
                    routes :: api :: cats :: DogColumnSelect :: IdNameColor, id
                    : Some(ids.clone()), name : None, color : None, order_by :
                    crate :: server :: postgres :: order_by :: OrderBy
                    {
                        column : crate :: repositories_types :: tufa_server ::
                        routes :: api :: cats :: DogColumn :: Name, order :
                        Some(crate :: server :: postgres :: order :: Order :: Desc),
                    }, limit : crate :: server :: postgres :: postgres_bigint ::
                    PostgresBigint(limit), offset : crate :: server :: postgres
                    :: postgres_bigint :: PostgresBigint(offset),
                }
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; } Err(e) =>
                { panic! ("{e}") ; }
            }
            println!("--------------try_delete_many_with_body-----------------");
            match crate :: repositories_types :: tufa_server :: routes :: api
            :: cats ::
            try_delete_many_with_body(& api_location, crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            DeleteManyWithBodyParameters
            {
                payload : crate :: repositories_types :: tufa_server :: routes
                :: api :: cats :: DeleteManyWithBodyPayload
                { id : Some(ids.clone()), name : None, color : None, }
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; } Err(e) =>
                { println! ("{e}") ; }
            }
            println!("--------------try_read_many_with_body-----------------");
            match
            crate :: repositories_types :: tufa_server :: routes :: api ::
            cats ::
            try_read_many_with_body(& api_location, crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            ReadManyWithBodyParameters
            {
                payload : crate :: repositories_types :: tufa_server :: routes
                :: api :: cats :: ReadManyWithBodyPayload
                {
                    select : crate :: repositories_types :: tufa_server ::
                    routes :: api :: cats :: DogColumnSelect :: IdNameColor, id
                    : Some(ids.clone()), name : None, color : None, order_by :
                    crate :: server :: postgres :: order_by :: OrderBy
                    {
                        column : crate :: repositories_types :: tufa_server ::
                        routes :: api :: cats :: DogColumn :: Name, order :
                        Some(crate :: server :: postgres :: order :: Order :: Desc),
                    }, limit : crate :: server :: postgres :: postgres_bigint ::
                    PostgresBigint(limit), offset : crate :: server :: postgres
                    :: postgres_bigint :: PostgresBigint(offset),
                }
            },).await
            {
                Ok(value) => { println! ("{value:#?}") ; } Err(e) =>
                { println! ("{e}") ; }
            }
        }
        match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cpus::get())
            .enable_all()
            .build()
        {
            Err(e) => {
                panic!
                ("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {e:#?}")
            }
            Ok(runtime) => {
                runtime.block_on(find_out_if_it_works());
            }
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyParameters {
    pub payload: std::vec::Vec<CreateManyPayloadElement>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyPayloadElement {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryCreateManyRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed {
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(
        crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
    ),
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
    fn from(value: TryCreateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateManyResponseVariants> for http::StatusCode {
    fn from(value: &TryCreateManyResponseVariants) -> Self {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(_) => http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Decode { decode : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => http
            :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => http :: StatusCode :: CREATED, TryCreateManyResponseVariants
            :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: CREATED, TryCreateManyResponseVariants
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateManyResponseVariantsTvfrr201Created {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
async fn try_from_response_try_create_many(
    response: reqwest::Response,
) -> Result<
    TryCreateManyResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::CREATED {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr201Created > (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateManyResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryCreateManyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryCreateManyWithSerializeDeserialize;
    fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
            TryCreateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryCreateManyResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryCreateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateManyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryCreateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_create_many<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    TryCreateManyRequestError,
> {
    match future.await
    {
        Ok(response) => match
        try_from_response_try_create_many(response).await
        {
            Ok(variants) => match std :: vec :: Vec :: < crate :: server ::
            postgres :: uuid_wrapper :: PossibleUuidWrapper > ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryCreateManyRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryCreateManyRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryCreateManyRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryCreateManyRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryCreateManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            } TryCreateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }
        }
    }
}
pub async fn try_create_many<'a>(
    server_location: &str,
    parameters: CreateManyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryCreateManyErrorNamed,
> {
    let payload = match serde_json::to_string(&parameters.payload) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/batch", server_location,);
    match tvfrr_extraction_logic_try_create_many(
        reqwest::Client::new()
            .post(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send(),
    )
    .await
    {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryCreateManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => Err(TryCreateManyErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn create_many(
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<std::vec::Vec<CreateManyPayloadElement>>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                std::vec::Vec<CreateManyPayloadElement>,
                TryCreateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            "insert into dogs (name, color) select name, color from unnest($1, $2) as a(name, color) returning id"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.len();
            let (name_vec, color_vec) = parameters.payload.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.name);
                    acc.1.push(element.color);
                    acc
                },
            );
            query = query.bind(name_vec);
            query = query.bind(color_vec);
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(e) => {
                    let error = TryCreateMany::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        } {
            match {
                use sqlx::Row;
                row.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => {
                    vec_values.push(
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                    );
                }
                Err(e) => {
                    let error = TryCreateMany ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        }
        TryCreateManyResponseVariants::Desirable(vec_values)
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryCreateMany
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateOnePayload {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryCreateOneRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper_in_client:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneResponseVariants {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOne> for TryCreateOneResponseVariants {
    fn from(value: TryCreateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateOneResponseVariants> for http::StatusCode {
    fn from(value: &TryCreateOneResponseVariants) -> Self {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(_) => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } => http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Decode { decode : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => http
            :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => http :: StatusCode :: CREATED, TryCreateOneResponseVariants
            :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: CREATED, TryCreateOneResponseVariants
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateOneResponseVariantsTvfrr201Created {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr201Created>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr408RequestTimeout>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr400BadRequest>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryCreateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr404NotFound>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryCreateOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr500InternalServerError>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
async fn try_from_response_try_create_one(
    response: reqwest::Response,
) -> Result<
    TryCreateOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::CREATED {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateOneResponseVariantsTvfrr201Created > (& response_text)
            {
                Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateOneResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryCreateOneResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryCreateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryCreateOneResponseVariants>
    for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
{
    type Error = TryCreateOneWithSerializeDeserialize;
    fn try_from(value: TryCreateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(i) => Ok(i),
            TryCreateOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryCreateOneResponseVariants
            :: JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryCreateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryCreateOneResponseVariants
            ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryCreateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryCreateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryCreateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateOneWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_create_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryCreateOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_create_one(response).await
        {
            Ok(variants) => match crate :: server :: postgres :: uuid_wrapper
            :: PossibleUuidWrapper :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryCreateOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryCreateOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryCreateOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryCreateOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryCreateOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryCreateOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            } TryCreateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: CREATED ; res
            }
        }
    }
}
pub async fn try_create_one<'a>(
    server_location: &str,
    parameters: CreateOneParameters,
) -> Result<crate::server::postgres::uuid_wrapper::UuidWrapper, TryCreateOneErrorNamed> {
    let payload = match serde_json::to_string(&parameters.payload) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs", server_location);
    match
    tvfrr_extraction_logic_try_create_one(reqwest :: Client ::
    new().post(&
    url).header(crate :: common :: git :: project_git_info :: PROJECT_COMMIT,
    crate :: global_variables :: compile_time :: project_git_info ::
    PROJECT_GIT_INFO.project_commit,).header(reqwest :: header ::
    CONTENT_TYPE, "application/json").body(payload).send()).await
    {
        Ok(value) => match crate :: server :: postgres :: uuid_wrapper ::
        UuidWrapper :: try_from(value)
        {
            Ok(value) => Ok(value), Err(e) =>
            Err(TryCreateOneErrorNamed ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_client : e,
                code_occurence : crate :: code_occurence_tufa_common! (),
            })
        }, Err(e) =>
        Err(TryCreateOneErrorNamed :: RequestError
        {
            request_error : e, code_occurence : crate ::
            code_occurence_tufa_common! (),
        }),
    }
}
pub async fn create_one(
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<CreateOnePayload>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateOnePayload,
                TryCreateOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = { "insert into dogs(name, color) values ($1, $2) returning id" };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.payload.name,
                query,
            );
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.payload.color,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => TryCreateOneResponseVariants::Desirable(
                    crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                ),
                Err(e) => {
                    let error = TryCreateOne ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryCreateOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryCreateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryCreateOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryCreateOne
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub path: ReadOnePath,
    pub query: ReadOneQuery,
}
#[derive(Debug)]
pub struct ReadOnePath {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadOnePathWithSerializeDeserialize {
    id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadOnePathWithSerializeDeserialize> for ReadOnePath {
    type Error = ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamed;
    fn try_from(value: ReadOnePathWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let id = match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value.id) {
            Ok(value) => value,
            Err(e) => {
                return Err(Self::Error::NotUuid {
                    not_uuid: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id })
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadOneQuery {
    pub select: std::option::Option<DogColumnSelect>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
struct ReadOneQueryWithSerializeDeserialize {
    select: std::option::Option<std::string::String>,
}
impl ReadOneQuery {
    fn into_url_encoding_version(self) -> ReadOneQueryWithSerializeDeserialize {
        let select = self.select.map(|value| {
            crate::common::serde_urlencoded::SerdeUrlencodedParameter::serde_urlencoded_parameter(
                value,
            )
        });
        ReadOneQueryWithSerializeDeserialize { select }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    QueryEncode {
        #[eo_display]
        url_encoding: serde_urlencoded::ser::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RequestError {
        #[eo_error_occurence]
        request_error: TryReadOneRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryReadOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
        #[eo_error_occurence]
        read_one_path_try_from_read_one_path_with_serialize_deserialize:
            ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneResponseVariants {
    Desirable(DogOptions),
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializeQueryString {
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
        read_one_path_try_from_read_one_path_with_serialize_deserialize:
            ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOne> for TryReadOneResponseVariants {
    fn from(value: TryReadOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryReadOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadOneWithSerializeDeserialize :: Tls { tls, code_occurence }
            => Self :: Tls { tls, code_occurence },
            TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryReadOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryReadOneWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryReadOneWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryReadOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence } => Self ::
            MissingPathParams { missing_path_params, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence } => Self ::
            FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            } => Self :: ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            }, TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryReadOneResponseVariants> for http::StatusCode {
    fn from(value: &TryReadOneResponseVariants) -> Self {
        match value {
            TryReadOneResponseVariants::Desirable(_) => http::StatusCode::OK,
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::FailedToDeserializePathParams {
                failed_to_deserialize_path_params: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::MissingPathParams {
                missing_path_params: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::FailedToDeserializeQueryString {
                failed_to_deserialize_query_string: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
                read_one_path_try_from_read_one_path_with_serialize_deserialize: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => http::StatusCode::OK,
            TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => http::StatusCode::OK,
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadOneResponseVariantsTvfrr200Ok {
    Desirable(DogOptions),
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr200Ok> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr500InternalServerError>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr500InternalServerError::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryReadOneResponseVariantsTvfrr500InternalServerError::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr408RequestTimeout>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr404NotFound> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializeQueryString {
        failed_to_deserialize_query_string: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
        read_one_path_try_from_read_one_path_with_serialize_deserialize:
            ReadOnePathTryFromReadOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        project_commit_not_equal: std::string::String,
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        project_commit_to_str_conversion: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr400BadRequest>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryReadOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: MissingPathParams
            { missing_path_params, code_occurence } => Self ::
            MissingPathParams { missing_path_params, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence } => Self ::
            FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            } => Self :: ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            }, TryReadOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
async fn try_from_response_try_read_one(
    response: reqwest::Response,
) -> Result<
    TryReadOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadOneResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadOneResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadOneResponseVariantsTvfrr400BadRequest > (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryReadOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryReadOneResponseVariants> for DogOptions {
    type Error = TryReadOneWithSerializeDeserialize;
    fn try_from(value: TryReadOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryReadOneResponseVariants :: Desirable(i) => Ok(i),
            TryReadOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryReadOneResponseVariants ::
            Database { database, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryReadOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryReadOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryReadOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryReadOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryReadOneResponseVariants ::
            TypeNotFound { type_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryReadOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryReadOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }), TryReadOneResponseVariants
            :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryReadOneResponseVariants :: Decode { decode, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryReadOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryReadOneResponseVariants ::
            PoolClosed { pool_closed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryReadOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryReadOneResponseVariants ::
            Migrate { migrate, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryReadOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryReadOneResponseVariants
            :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence }),
            TryReadOneResponseVariants :: MissingPathParams
            { missing_path_params, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence }),
            TryReadOneResponseVariants :: FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            FailedToDeserializeQueryString
            { failed_to_deserialize_query_string, code_occurence }),
            TryReadOneResponseVariants ::
            ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ReadOnePathTryFromReadOnePathWithSerializeDeserialize
            {
                read_one_path_try_from_read_one_path_with_serialize_deserialize,
                code_occurence
            }), TryReadOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryReadOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryReadOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadOneWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_read_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<DogOptions, TryReadOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_read_one(response).await
        {
            Ok(variants) => match DogOptions :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryReadOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryReadOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryReadOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryReadOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryReadOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryReadOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    FailedToDeserializePathParamsTvfrr400BadRequest,
    MissingPathParamsTvfrr400BadRequest,
    FailedToDeserializeQueryStringTvfrr400BadRequest,
    ReadOnePathTryFromReadOnePathWithSerializeDeserializeTvfrr400BadRequest,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match &self {
            TryReadOneResponseVariants::Desirable(_) => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::FailedToDeserializePathParams {
                failed_to_deserialize_path_params: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::MissingPathParams {
                missing_path_params: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::FailedToDeserializeQueryString {
                failed_to_deserialize_query_string: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
                read_one_path_try_from_read_one_path_with_serialize_deserialize: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = http::StatusCode::OK;
                res
            }
        }
    }
}
pub async fn try_read_one<'a>(
    server_location: &str,
    parameters: ReadOneParameters,
) -> Result<DogOptions, TryReadOneErrorNamed> {
    let encoded_query =
        match serde_urlencoded::to_string(parameters.query.into_url_encoding_version()) {
            Ok(value) => value,
            Err(e) => {
                return Err(TryReadOneErrorNamed::QueryEncode {
                    url_encoding: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
    let url = format!(
        "{}/dogs/{}?{}",
        server_location, parameters.path.id, encoded_query
    );
    match tvfrr_extraction_logic_try_read_one(
        reqwest::Client::new()
            .get(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .send(),
    )
    .await
    {
        Ok(value) => Ok(value),
        Err(e) => Err(TryReadOneErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn read_one(
    path_extraction_result: Result<
        axum::extract::Path<ReadOnePathWithSerializeDeserialize>,
        axum::extract::rejection::PathRejection,
    >,
    query_extraction_result: Result<
        axum::extract::Query<ReadOneQuery>,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
) -> impl axum::response::IntoResponse {
    let parameters = ReadOneParameters {
        path: match crate::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            ReadOnePathWithSerializeDeserialize,
            TryReadOneResponseVariants,
        >::try_extract_value(path_extraction_result, &app_info_state)
        {
            Ok(value) => match ReadOnePath::try_from(value) {
                Ok(value) => value,
                Err(e) => {
                    let error = TryReadOne::ReadOnePathTryFromReadOnePathWithSerializeDeserialize {
                        read_one_path_try_from_read_one_path_with_serialize_deserialize: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadOneResponseVariants::from(error);
                }
            },
            Err(err) => {
                return err;
            }
        },
        query:
            match crate::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
                ReadOneQuery,
                TryReadOneResponseVariants,
            >::try_extract_value(query_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let select = parameters.query.select.unwrap_or_default();
        let query_string = {
            format!(
                "select {} from dogs where id = $1",
                crate::server::postgres::generate_query::GenerateQuery::generate_query(&select),
            )
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.path.id,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(row) => match select.options_try_from_sqlx_row(&row) {
                Ok(value) => TryReadOneResponseVariants::Desirable(value),
                Err(e) => {
                    let error = TryReadOne::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryReadOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryReadOne
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct ReadManyWithBodyParameters {
    pub payload: ReadManyWithBodyPayload,
}
#[derive(Debug)]
pub struct ReadManyWithBodyPayload {
    pub select: DogColumnSelect,
    pub id: std::option::Option<std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>>,
    pub name:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: crate::server::postgres::order_by::OrderBy<DogColumn>,
    pub limit: crate::server::postgres::postgres_bigint::PostgresBigint,
    pub offset: crate::server::postgres::postgres_bigint::PostgresBigint,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadManyWithBodyPayloadWithSerializeDeserialize {
    pub select: DogColumnSelect,
    pub id: std::option::Option<std::vec::Vec<std::string::String>>,
    pub name:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub order_by: crate::server::postgres::order_by::OrderBy<DogColumn>,
    pub limit: crate::server::postgres::postgres_bigint::PostgresBigint,
    pub offset: crate::server::postgres::postgres_bigint::PostgresBigint,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadManyWithBodyPayloadWithSerializeDeserialize>
    for ReadManyWithBodyPayload
{
    type Error =
        ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: ReadManyWithBodyPayloadWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let select = value.select;
        let id = match value.id
        {
            Some(value) => match
            value.into_iter().map(| element | crate :: server :: postgres ::
            uuid_wrapper :: UuidWrapper ::
            try_from(crate :: server :: postgres :: uuid_wrapper ::
            PossibleUuidWrapper :: from(element))).collect :: < Result < std
            :: vec :: Vec < crate :: server :: postgres :: uuid_wrapper ::
            UuidWrapper >, crate :: server :: postgres :: uuid_wrapper ::
            UuidWrapperTryFromPossibleUuidWrapperErrorNamed >> ()
            {
                Ok(value) => Some(value), Err(e) =>
                {
                    return
                    Err(Self :: Error :: NotUuid
                    {
                        not_uuid : e, code_occurence : crate ::
                        code_occurence_tufa_common! (),
                    }) ;
                }
            }, None => None
        } ;
        let name = value.name;
        let color = value.color;
        let order_by = value.order_by;
        let limit = value.limit;
        let offset = value.offset;
        Ok(Self {
            select,
            id,
            name,
            color,
            order_by,
            limit,
            offset,
        })
    }
}
impl std::convert::From<ReadManyWithBodyPayload>
    for ReadManyWithBodyPayloadWithSerializeDeserialize
{
    fn from(value: ReadManyWithBodyPayload) -> Self {
        let select = value.select;
        let id = match value.id {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| element.to_string())
                    .collect::<std::vec::Vec<std::string::String>>(),
            ),
            None => None,
        };
        let name = value.name;
        let color = value.color;
        let order_by = value.order_by;
        let limit = value.limit;
        let offset = value.offset;
        Self {
            select,
            id,
            name,
            color,
            order_by,
            limit,
            offset,
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadManyWithBodyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryReadManyWithBodyRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryReadManyWithBody {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotUniqueIdVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_id_vec: std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotUniqueNameVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_name_vec: std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotUniqueColorVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_color_vec: std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize:
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyWithBodyResponseVariants {
    Desirable(std :: vec :: Vec :: < DogOptions >), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec : std :: vec :: Vec < crate :: server :: postgres ::
        regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NotUniqueNameVec
    {
        not_unique_name_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniqueColorVec
    {
        not_unique_color_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NotUuid
    {
        not_uuid : std :: string :: String, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    },
    ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
    {
        read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
        :
        ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryReadManyWithBody> for TryReadManyWithBodyResponseVariants {
    fn from(value: TryReadManyWithBody) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadManyWithBodyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence } => Self :: PoolTimedOut
            { pool_timed_out, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } => Self :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueNameVec
            { not_unique_name_vec, code_occurence } => Self ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueColorVec
            { not_unique_color_vec, code_occurence } => Self ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize :: NotUuid
            { not_uuid, code_occurence } => Self :: NotUuid
            { not_uuid, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }, TryReadManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadManyWithBodyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryReadManyWithBodyResponseVariants> for http::StatusCode {
    fn from(value: &TryReadManyWithBodyResponseVariants) -> Self {
        match value
        {
            TryReadManyWithBodyResponseVariants :: Desirable(_) => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants :: Io
            { io : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryReadManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryReadManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryReadManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => http
            :: StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            NotUniqueIdVec { not_unique_id_vec : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            NotUniqueNameVec { not_unique_name_vec : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants
            :: NotUniqueColorVec
            { not_unique_color_vec : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryReadManyWithBodyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryReadManyWithBodyResponseVariants :: NotUuid
            { not_uuid : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryReadManyWithBodyResponseVariants ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants
            :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryReadManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadManyWithBodyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<DogOptions>),
}
impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr200Ok>
    for TryReadManyWithBodyResponseVariants
{
    fn from(value: TryReadManyWithBodyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryReadManyWithBodyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadManyWithBodyResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec : std :: vec :: Vec < crate :: server :: postgres ::
        regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NotUniqueNameVec
    {
        not_unique_name_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniqueColorVec
    {
        not_unique_color_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NotUuid
    {
        not_uuid : std :: string :: String, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    },
    ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
    {
        read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
        :
        ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr400BadRequest>
    for TryReadManyWithBodyResponseVariants
{
    fn from(value: TryReadManyWithBodyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
            JsonSyntaxError { json_syntax_error, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueIdVec { not_unique_id_vec, code_occurence } => Self ::
            NotUniqueIdVec { not_unique_id_vec, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence } => Self
            :: NotUniqueNameVec { not_unique_name_vec, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence } => Self
            :: NotUniqueColorVec { not_unique_color_vec, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest :: NotUuid
            { not_uuid, code_occurence } => Self :: NotUuid
            { not_uuid, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }, TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadManyWithBodyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr404NotFound>
    for TryReadManyWithBodyResponseVariants
{
    fn from(value: TryReadManyWithBodyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryReadManyWithBodyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout>
    for TryReadManyWithBodyResponseVariants
{
    fn from(value: TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError>
    for TryReadManyWithBodyResponseVariants
{
    fn from(value: TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Tls { tls, code_occurence } => Self :: Tls
            { tls, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Decode { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence }
        }
    }
}
async fn try_from_response_try_read_many_with_body(
    response: reqwest::Response,
) -> Result<
    TryReadManyWithBodyResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadManyWithBodyResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) =>
                Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadManyWithBodyResponseVariantsTvfrr400BadRequest >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadManyWithBodyResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryReadManyWithBodyResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryReadManyWithBodyResponseVariants :: from(value)), Err(e)
                =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryReadManyWithBodyResponseVariants> for std::vec::Vec<DogOptions> {
    type Error = TryReadManyWithBodyWithSerializeDeserialize;
    fn try_from(value: TryReadManyWithBodyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryReadManyWithBodyResponseVariants :: Desirable(i) => Ok(i),
            TryReadManyWithBodyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }),
            TryReadManyWithBodyResponseVariants :: Database
            { database, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryReadManyWithBodyResponseVariants
            :: Io { io, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryReadManyWithBodyResponseVariants ::
            Tls { tls, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryReadManyWithBodyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryReadManyWithBodyResponseVariants
            :: RowNotFound { row_not_found, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }),
            TryReadManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }),
            TryReadManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryReadManyWithBodyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryReadManyWithBodyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryReadManyWithBodyResponseVariants :: Decode
            { decode, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryReadManyWithBodyResponseVariants
            :: PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }),
            TryReadManyWithBodyResponseVariants :: PoolClosed
            { pool_closed, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }),
            TryReadManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }),
            TryReadManyWithBodyResponseVariants :: Migrate
            { migrate, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryReadManyWithBodyResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryReadManyWithBodyResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryReadManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryReadManyWithBodyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryReadManyWithBodyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryReadManyWithBodyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence }),
            TryReadManyWithBodyResponseVariants :: NotUniqueNameVec
            { not_unique_name_vec, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence }),
            TryReadManyWithBodyResponseVariants :: NotUniqueColorVec
            { not_unique_color_vec, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence }),
            TryReadManyWithBodyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }),
            TryReadManyWithBodyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }),
            TryReadManyWithBodyResponseVariants :: NotUuid
            { not_uuid, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize :: NotUuid
            { not_uuid, code_occurence }), TryReadManyWithBodyResponseVariants
            ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }), TryReadManyWithBodyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryReadManyWithBodyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryReadManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryReadManyWithBodyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryReadManyWithBodyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadManyWithBodyWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_read_many_with_body<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<std::vec::Vec<DogOptions>, TryReadManyWithBodyRequestError> {
    match future.await
    {
        Ok(response) => match
        try_from_response_try_read_many_with_body(response).await
        {
            Ok(variants) => match std :: vec :: Vec :: < DogOptions > ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryReadManyWithBodyRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryReadManyWithBodyRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryReadManyWithBodyRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryReadManyWithBodyRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryReadManyWithBodyRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryReadManyWithBodyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    NotUniqueIdVecTvfrr400BadRequest,
    NotUniqueNameVecTvfrr400BadRequest,
    NotUniqueColorVecTvfrr400BadRequest,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    NotUuidTvfrr400BadRequest,
    ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadManyWithBodyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryReadManyWithBodyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryReadManyWithBodyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: NotUniqueNameVec
            { not_unique_name_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: NotUniqueColorVec
            { not_unique_color_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants :: NotUuid
            { not_uuid : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants ::
            ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
            {
                read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryReadManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_read_many_with_body<'a>(
    server_location: &str,
    parameters: ReadManyWithBodyParameters,
) -> Result<std::vec::Vec<DogOptions>, TryReadManyWithBodyErrorNamed> {
    let payload = match serde_json::to_string(
        &ReadManyWithBodyPayloadWithSerializeDeserialize::from(parameters.payload),
    ) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryReadManyWithBodyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/search", server_location);
    match tvfrr_extraction_logic_try_read_many_with_body(
        reqwest::Client::new()
            .post(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send(),
    )
    .await
    {
        Ok(value) => Ok(value),
        Err(e) => Err(TryReadManyWithBodyErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn read_many_with_body(
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<ReadManyWithBodyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = ReadManyWithBodyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                ReadManyWithBodyPayloadWithSerializeDeserialize,
                TryReadManyWithBodyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => match ReadManyWithBodyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryReadManyWithBody ::
                    ReadManyWithBodyPayloadTryFromReadManyWithBodyPayloadWithSerializeDeserialize
                    {
                        read_many_with_body_payload_try_from_read_many_with_body_payload_with_serialize_deserialize
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadManyWithBodyResponseVariants::from(error);
                    }
                },
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let Some(id) = &parameters.payload.id {
            let not_unique_primary_keys = {
                let mut vec = std::vec::Vec::with_capacity(id.len());
                let mut not_unique_primary_keys = std::vec::Vec::with_capacity(id.len());
                for element in id {
                    let handle = element;
                    match vec.contains(&handle) {
                        true => {
                            not_unique_primary_keys.push(element.clone());
                        }
                        false => {
                            vec.push(element);
                        }
                    }
                }
                not_unique_primary_keys
            };
            if let false = not_unique_primary_keys.is_empty() {
                let error = TryReadManyWithBody::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryReadManyWithBodyResponseVariants::from(error);
            }
        }
        let name_handle = match parameters.payload.name {
            Some(value) => {
                let is_unique = {
                    let mut vec = std::vec::Vec::with_capacity(value.len());
                    let mut is_unique = true;
                    for element in &value {
                        match vec.contains(&element) {
                            true => {
                                is_unique = false;
                                break;
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    is_unique
                };
                match is_unique {
                    true => Some(value),
                    false => {
                        let not_unique_name_vec = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut not_unique_name_vec = std::vec::Vec::with_capacity(value.len());
                            for element in value {
                                match vec.contains(&element) {
                                    true => {
                                        not_unique_name_vec.push(element);
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            not_unique_name_vec
                        };
                        let error = TryReadManyWithBody::NotUniqueNameVec {
                            not_unique_name_vec,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadManyWithBodyResponseVariants::from(error);
                    }
                }
            }
            None => None,
        };
        let color_handle = match parameters.payload.color {
            Some(value) => {
                let is_unique = {
                    let mut vec = std::vec::Vec::with_capacity(value.len());
                    let mut is_unique = true;
                    for element in &value {
                        match vec.contains(&element) {
                            true => {
                                is_unique = false;
                                break;
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    is_unique
                };
                match is_unique {
                    true => Some(value),
                    false => {
                        let not_unique_color_vec = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut not_unique_color_vec =
                                std::vec::Vec::with_capacity(value.len());
                            for element in value {
                                match vec.contains(&element) {
                                    true => {
                                        not_unique_color_vec.push(element);
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            not_unique_color_vec
                        };
                        let error = TryReadManyWithBody::NotUniqueColorVec {
                            not_unique_color_vec,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadManyWithBodyResponseVariants::from(error);
                    }
                }
            }
            None => None,
        };
        let query_string = {
            format!(
                "select {} from dogs {}",
                crate::server::postgres::generate_query::GenerateQuery::generate_query(
                    &parameters.payload.select
                ),
                {
                    let mut increment: u64 = 0;
                    let mut additional_parameters = std::string::String::default();
                    if let Some(value) = &parameters.payload.id {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        match increment.checked_add(1) {
                            Some(value) => {
                                increment = value;
                            }
                            None => {
                                let e = crate :: server :: postgres :: bind_query ::
                            TryGenerateBindIncrementsErrorNamed :: CheckedAdd
                            {
                                checked_add : std :: string :: String ::
                                from("checked_add is None"), code_occurence : crate ::
                                code_occurence_tufa_common! (),
                            } ;
                                return TryReadManyWithBodyResponseVariants::BindQuery {
                                    bind_query: e.into_serialize_deserialize_version(),
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                            }
                        }
                        additional_parameters
                            .push_str(&format!("{} id in (select unnest(${}))", prefix, increment));
                    }
                    if let Some(value) = &name_handle {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let bind_increments = {
                            let mut bind_increments = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match crate :: server :: postgres :: bind_query :: BindQuery
                            :: try_generate_bind_increments(element, & mut increment)
                            {
                                Ok(value) =>
                                {
                                    let handle = format! ("name ~ {value} ") ; match index == 0
                                    {
                                        true => { bind_increments.push_str(& handle) ; }, false =>
                                        {
                                            bind_increments.push_str(& format!
                                            ("{} {handle}", element.conjuctive_operator)) ;
                                        },
                                    }
                                }, Err(e) =>
                                {
                                    return TryReadManyWithBodyResponseVariants :: BindQuery
                                    {
                                        bind_query : e.into_serialize_deserialize_version(),
                                        code_occurence : crate :: code_occurence_tufa_common! ()
                                    } ;
                                },
                            }
                            }
                            if let false = bind_increments.is_empty() {
                                bind_increments.pop();
                            }
                            bind_increments
                        };
                        additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                    }
                    if let Some(value) = &color_handle {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let bind_increments = {
                            let mut bind_increments = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match crate :: server :: postgres :: bind_query :: BindQuery
                            :: try_generate_bind_increments(element, & mut increment)
                            {
                                Ok(value) =>
                                {
                                    let handle = format! ("color ~ {value} ") ; match index == 0
                                    {
                                        true => { bind_increments.push_str(& handle) ; }, false =>
                                        {
                                            bind_increments.push_str(& format!
                                            ("{} {handle}", element.conjuctive_operator)) ;
                                        },
                                    }
                                }, Err(e) =>
                                {
                                    return TryReadManyWithBodyResponseVariants :: BindQuery
                                    {
                                        bind_query : e.into_serialize_deserialize_version(),
                                        code_occurence : crate :: code_occurence_tufa_common! ()
                                    } ;
                                },
                            }
                            }
                            if let false = bind_increments.is_empty() {
                                bind_increments.pop();
                            }
                            bind_increments
                        };
                        additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = &parameters.payload.order_by;
                        let order_stringified = match &value.order {
                            Some(order) => order.to_string(),
                            None => crate::server::postgres::order::Order::default().to_string(),
                        };
                        additional_parameters.push_str(&format!(
                            "{}order by {} {}",
                            prefix, value.column, order_stringified
                        ));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match crate ::
                    server :: postgres :: bind_query :: BindQuery ::
                    try_generate_bind_increments(& parameters.payload.limit, &
                    mut increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadManyWithBodyResponseVariants :: BindQuery
                            {
                                bind_query : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        },
                    } ;
                        additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match crate ::
                    server :: postgres :: bind_query :: BindQuery ::
                    try_generate_bind_increments(& parameters.payload.offset, &
                    mut increment)
                    {
                        Ok(value) => value, Err(e) =>
                        {
                            return TryReadManyWithBodyResponseVariants :: BindQuery
                            {
                                bind_query : e.into_serialize_deserialize_version(),
                                code_occurence : crate :: code_occurence_tufa_common! ()
                            } ;
                        },
                    } ;
                        additional_parameters.push_str(&format!("{}offset {}", prefix, value));
                    }
                    additional_parameters
                }
            )
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters.payload.id {
                query = query.bind(
                    value
                        .into_iter()
                        .map(|element| element.into_inner().clone())
                        .collect::<std::vec::Vec<sqlx::types::Uuid>>(),
                );
            }
            if let Some(values) = name_handle {
                for value in values {
                    query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                        value, query,
                    );
                }
            }
            if let Some(values) = color_handle {
                for value in values {
                    query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                        value, query,
                    );
                }
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.payload.limit,
                query,
            );
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.payload.offset,
                query,
            );
            query
        };
        let vec_values = {
            let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                Ok(value) => value,
                Err(e) => {
                    let error = TryReadManyWithBody::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadManyWithBodyResponseVariants::from(error);
                }
            };
            let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(value) => value,
                Err(e) => {
                    let error = TryReadManyWithBody::from(e);
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryReadManyWithBodyResponseVariants::from(error);
                }
            };
            let mut rows = binded_query.fetch(pg_connection.as_mut());
            let mut vec_values = std::vec::Vec::new();
            while let Some(row) = {
                match {
                    use futures::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryReadManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadManyWithBodyResponseVariants::from(error);
                    }
                }
            } {
                match parameters.payload.select.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let error = TryReadManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryReadManyWithBodyResponseVariants::from(error);
                    }
                }
            }
            vec_values
        };
        TryReadManyWithBodyResponseVariants::Desirable(vec_values)
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryReadManyWithBody
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub path: UpdateOnePath,
    pub payload: UpdateOnePayload,
}
#[derive(Debug)]
pub struct UpdateOnePath {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateOnePathWithSerializeDeserialize {
    id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<UpdateOnePathWithSerializeDeserialize> for UpdateOnePath {
    type Error = UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed;
    fn try_from(value: UpdateOnePathWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let id = match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value.id) {
            Ok(value) => value,
            Err(e) => {
                return Err(Self::Error::NotUuid {
                    not_uuid: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        Ok(Self { id })
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateOnePayload {
    pub name: std::option::Option<std::string::String>,
    pub color: std::option::Option<std::string::String>,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryUpdateOneRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper_in_client:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize {
        #[eo_error_occurence]
        update_one_path_try_from_update_one_path_with_serialize_deserialize:
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneResponseVariants {
    Desirable(crate :: server :: postgres :: uuid_wrapper ::
    PossibleUuidWrapper), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
    {
        update_one_path_try_from_update_one_path_with_serialize_deserialize :
        UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryUpdateOne> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence } => Self ::
            MissingPathParams { missing_path_params, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryUpdateOneResponseVariants> for http::StatusCode {
    fn from(value: &TryUpdateOneResponseVariants) -> Self {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(_) => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: Io { io : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            Decode { decode : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            MissingPathParams { missing_path_params : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => http
            :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr200Ok {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr200Ok> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
    {
        update_one_path_try_from_update_one_path_with_serialize_deserialize :
        UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr400BadRequest>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            MissingPathParams { missing_path_params, code_occurence } => Self
            :: MissingPathParams { missing_path_params, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }, TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr500InternalServerError>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr404NotFound>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
async fn try_from_response_try_update_one(
    response: reqwest::Response,
) -> Result<
    TryUpdateOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr400BadRequest > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateOneResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryUpdateOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryUpdateOneResponseVariants>
    for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
{
    type Error = TryUpdateOneWithSerializeDeserialize;
    fn try_from(value: TryUpdateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(i) => Ok(i),
            TryUpdateOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryUpdateOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryUpdateOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryUpdateOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryUpdateOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryUpdateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryUpdateOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryUpdateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryUpdateOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryUpdateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryUpdateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryUpdateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryUpdateOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryUpdateOneResponseVariants
            :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence }),
            TryUpdateOneResponseVariants :: MissingPathParams
            { missing_path_params, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence }),
            TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryUpdateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryUpdateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryUpdateOneResponseVariants
            :: BindQuery { bind_query, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryUpdateOneResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize,
                code_occurence
            }), TryUpdateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryUpdateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryUpdateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateOneWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_update_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryUpdateOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_update_one(response).await
        {
            Ok(variants) => match crate :: server :: postgres :: uuid_wrapper
            :: PossibleUuidWrapper :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryUpdateOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryUpdateOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryUpdateOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryUpdateOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryUpdateOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryUpdateOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    FailedToDeserializePathParamsTvfrr400BadRequest,
    MissingPathParamsTvfrr400BadRequest,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    UpdateOnePathTryFromUpdateOnePathWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryUpdateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryUpdateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: MissingPathParams
            { missing_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize
            {
                update_one_path_try_from_update_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_update_one<'a>(
    server_location: &str,
    parameters: UpdateOneParameters,
) -> Result<crate::server::postgres::uuid_wrapper::UuidWrapper, TryUpdateOneErrorNamed> {
    let payload = match serde_json::to_string(&parameters.payload) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/{}", server_location, parameters.path.id.to_inner());
    match
    tvfrr_extraction_logic_try_update_one(reqwest :: Client ::
    new().patch(&
    url).header(crate :: common :: git :: project_git_info :: PROJECT_COMMIT,
    crate :: global_variables :: compile_time :: project_git_info ::
    PROJECT_GIT_INFO.project_commit,).header(reqwest :: header ::
    CONTENT_TYPE, "application/json").body(payload).send()).await
    {
        Ok(value) => match crate :: server :: postgres :: uuid_wrapper ::
        UuidWrapper :: try_from(value)
        {
            Ok(value) => Ok(value), Err(e) =>
            Err(TryUpdateOneErrorNamed ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_client : e,
                code_occurence : crate :: code_occurence_tufa_common! (),
            })
        }, Err(e) =>
        Err(TryUpdateOneErrorNamed :: RequestError
        {
            request_error : e, code_occurence : crate ::
            code_occurence_tufa_common! (),
        }),
    }
}
pub async fn update_one<'a>(
    path_extraction_result: Result<
        axum::extract::Path<UpdateOnePathWithSerializeDeserialize>,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
    payload_extraction_result: Result<
        axum::Json<UpdateOnePayload>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateOneParameters {
        path: match crate::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            UpdateOnePathWithSerializeDeserialize,
            TryUpdateOneResponseVariants,
        >::try_extract_value(path_extraction_result, &app_info_state)
        {
            Ok(value) => match UpdateOnePath::try_from(value) {
                Ok(value) => value,
                Err(e) => {
                    let error =
                        TryUpdateOne::UpdateOnePathTryFromUpdateOnePathWithSerializeDeserialize {
                            update_one_path_try_from_update_one_path_with_serialize_deserialize: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryUpdateOneResponseVariants::from(error);
                }
            },
            Err(err) => {
                return err;
            }
        },
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                UpdateOnePayload,
                TryUpdateOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None) = (&parameters.payload.name, &parameters.payload.color) {
            return TryUpdateOneResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
        }
        let query_string = {
            let mut increment: u64 = 0;
            let mut query = std::string::String::from("update dogs set ");
            if let Some(value) = &parameters.payload.name {
                match crate::server::postgres::bind_query::BindQuery::try_increment(
                    value,
                    &mut increment,
                ) {
                    Ok(_) => {
                        query.push_str(&format!("name = ${increment}, "));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            bind_query: e.into_serialize_deserialize_version(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    }
                }
            }
            if let Some(value) = &parameters.payload.color {
                match crate::server::postgres::bind_query::BindQuery::try_increment(
                    value,
                    &mut increment,
                ) {
                    Ok(_) => {
                        query.push_str(&format!("color = ${increment}"));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            bind_query: e.into_serialize_deserialize_version(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    }
                }
            }
            match crate::server::postgres::bind_query::BindQuery::try_increment(
                &parameters.path.id,
                &mut increment,
            ) {
                Ok(_) => {
                    query.push_str(&format!(" where id = ${increment}"));
                }
                Err(e) => {
                    return TryUpdateOneResponseVariants::BindQuery {
                        bind_query: e.into_serialize_deserialize_version(),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                }
            }
            query.push_str(&format!(" returning id"));
            query
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters.payload.name {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            if let Some(value) = parameters.payload.color {
                query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                    value, query,
                );
            }
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.path.id,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => TryUpdateOneResponseVariants::Desirable(
                    crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                ),
                Err(e) => {
                    let error = TryUpdateOne ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryUpdateOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryUpdateOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryUpdateOne
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: std::vec::Vec<UpdateManyPayloadElement>,
}
#[derive(Debug)]
pub struct UpdateManyPayloadElement {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize {
    pub id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
    pub name: std::string::String,
    pub color: std::string::String,
}
impl std::convert::From<UpdateManyPayloadElement>
    for UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: UpdateManyPayloadElement) -> Self {
        let id = crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value.id);
        let name = value.name;
        let color = value.color;
        Self { id, name, color }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_display]
        not_uuid: sqlx::types::uuid::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<UpdateManyPayloadElementWithSerializeDeserialize>
    for UpdateManyPayloadElement
{
    type Error =
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: UpdateManyPayloadElementWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let id = match sqlx::types::Uuid::parse_str(value.id.to_inner()) {
            Ok(value) => crate::server::postgres::uuid_wrapper::UuidWrapper::from(value),
            Err(e) => {
                return Err(Self::Error::NotUuid {
                    not_uuid: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let name = value.name;
        let color = value.color;
        Ok(Self { id, name, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryUpdateManyRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateMany {
    Configuration
    {
        #[eo_display_with_serialize_deserialize] configuration : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Database
    {
        #[eo_display_with_serialize_deserialize] database : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Io
    {
        #[eo_display] io : std :: io :: Error, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Tls
    {
        #[eo_display_with_serialize_deserialize] tls : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Protocol
    {
        #[eo_display_with_serialize_deserialize] protocol : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, RowNotFound
    {
        #[eo_display_with_serialize_deserialize] row_not_found : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, TypeNotFound
    {
        #[eo_display_with_serialize_deserialize] type_not_found : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        #[eo_display_with_serialize_deserialize] column_index_out_of_bounds :
        usize, #[eo_display_with_serialize_deserialize] len : usize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        #[eo_display_with_serialize_deserialize] column_not_found : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, ColumnDecode
    {
        #[eo_display_with_serialize_deserialize] column_decode_index : std ::
        string :: String, #[eo_display_with_serialize_deserialize]
        source_handle : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Decode
    {
        #[eo_display_with_serialize_deserialize] decode : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, PoolTimedOut
    {
        #[eo_display_with_serialize_deserialize] pool_timed_out : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, PoolClosed
    {
        #[eo_display_with_serialize_deserialize] pool_closed : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, WorkerCrashed
    {
        #[eo_display_with_serialize_deserialize] worker_crashed : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, Migrate
    {
        #[eo_display] migrate : sqlx :: migrate :: MigrateError,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        #[eo_display_with_serialize_deserialize] unexpected_case : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, JsonDataError
    {
        #[eo_display] json_data_error : axum :: extract :: rejection ::
        JsonDataError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, JsonSyntaxError
    {
        #[eo_display] json_syntax_error : axum :: extract :: rejection ::
        JsonSyntaxError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, MissingJsonContentType
    {
        #[eo_display_with_serialize_deserialize] missing_json_content_type :
        std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, BytesRejection
    {
        #[eo_display_with_serialize_deserialize] bytes_rejection : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        #[eo_vec_display] not_unique_primary_keys : std :: vec :: Vec < crate
        :: server :: postgres :: uuid_wrapper :: UuidWrapper >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        #[eo_error_occurence] bind_query : crate :: server :: postgres ::
        bind_query :: TryGenerateBindIncrementsErrorNamed, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, CheckedAdd
    {
        #[eo_display_with_serialize_deserialize] checked_add : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NoPayloadFields
    {
        #[eo_display_with_serialize_deserialize] no_payload_fields : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, CommitFailed
    {
        #[eo_display] commit_failed : sqlx :: Error, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        #[eo_vec_display] non_existing_primary_keys : std :: vec :: Vec <
        crate :: server :: postgres :: uuid_wrapper :: UuidWrapper >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        #[eo_display] primary_key_from_row : sqlx :: Error, #[eo_display]
        rollback_error : sqlx :: Error, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        #[eo_vec_display] non_existing_primary_keys : std :: vec :: Vec <
        crate :: server :: postgres :: uuid_wrapper :: UuidWrapper >,
        #[eo_display] rollback_error : sqlx :: Error, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        #[eo_display] query_error : sqlx :: Error, #[eo_display]
        rollback_error : sqlx :: Error, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    },
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
    {
        #[eo_error_occurence]
        update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
        :
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        #[eo_display_with_serialize_deserialize] project_commit_not_equal :
        std :: string :: String, #[eo_display_with_serialize_deserialize]
        project_commit_to_use : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        #[eo_display] project_commit_to_str_conversion : http :: header ::
        ToStrError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        #[eo_display_with_serialize_deserialize] no_project_commit_header :
        std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, CheckedAdd
    {
        checked_add : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, rollback_error : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    },
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
    {
        update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
        :
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryUpdateMany> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryUpdateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryUpdateManyWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } => Self ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: CheckedAdd
            { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence } => Self :: CommitFailed
            { commit_failed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            }, TryUpdateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryUpdateManyResponseVariants> for http::StatusCode {
    fn from(value: &TryUpdateManyResponseVariants) -> Self {
        match value
        {
            TryUpdateManyResponseVariants :: Desirable(_) => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: Io { io : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            Decode { decode : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: CheckedAdd
            { checked_add : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } => http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr200Ok> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, rollback_error : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    },
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
    {
        update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
        :
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr400BadRequest>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            }, TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateManyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, CheckedAdd
    {
        checked_add : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr500InternalServerError>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr404NotFound>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryUpdateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
async fn try_from_response_try_update_many(
    response: reqwest::Response,
) -> Result<
    TryUpdateManyResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateManyResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateManyResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateManyResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryUpdateManyResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryUpdateManyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryUpdateManyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryUpdateManyWithSerializeDeserialize;
    fn try_from(value: TryUpdateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryUpdateManyResponseVariants :: Desirable(i) => Ok(i),
            TryUpdateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryUpdateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryUpdateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryUpdateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryUpdateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryUpdateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryUpdateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryUpdateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryUpdateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryUpdateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryUpdateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryUpdateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryUpdateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryUpdateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryUpdateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryUpdateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryUpdateManyResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryUpdateManyResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryUpdateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryUpdateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryUpdateManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence }),
            TryUpdateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryUpdateManyResponseVariants ::
            CheckedAdd { checked_add, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: CheckedAdd
            { checked_add, code_occurence }), TryUpdateManyResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence }), TryUpdateManyResponseVariants
            :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence }),
            TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize,
                code_occurence
            }), TryUpdateManyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryUpdateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryUpdateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryUpdateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateManyWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_update_many<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    TryUpdateManyRequestError,
> {
    match future.await
    {
        Ok(response) => match
        try_from_response_try_update_many(response).await
        {
            Ok(variants) => match std :: vec :: Vec :: < crate :: server ::
            postgres :: uuid_wrapper :: PossibleUuidWrapper > ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryUpdateManyRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryUpdateManyRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryUpdateManyRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryUpdateManyRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryUpdateManyRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryUpdateManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    CheckedAddTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    CommitFailedTvfrr500InternalServerError,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeTvfrr400BadRequest,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryUpdateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryUpdateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CheckedAdd
            { checked_add : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
            {
                update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_update_many<'a>(
    server_location: &str,
    parameters: UpdateManyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryUpdateManyErrorNamed,
> {
    let payload = match serde_json::to_string(
        (&parameters
            .payload
            .into_iter()
            .map(|element| UpdateManyPayloadElementWithSerializeDeserialize::from(element))
            .collect::<std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>>()),
    ) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/batch", server_location,);
    match tvfrr_extraction_logic_try_update_many(
        reqwest::Client::new()
            .patch(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send(),
    )
    .await
    {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryUpdateManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => Err(TryUpdateManyErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn update_many<'a>(
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
    payload_extraction_result: Result<
        axum::Json<std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateManyParameters
    {
        payload : match crate :: server :: routes :: helpers ::
        json_extractor_error :: JsonValueResultExtractor :: < std :: vec ::
        Vec :: < UpdateManyPayloadElementWithSerializeDeserialize >,
        TryUpdateManyResponseVariants, > ::
        try_extract_value(payload_extraction_result, & app_info_state)
        {
            Ok(value) => match
            value.into_iter().map(| element | UpdateManyPayloadElement ::
            try_from(element)).collect :: < Result < std :: vec :: Vec <
            UpdateManyPayloadElement >,
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
            >> ()
            {
                Ok(value) => value, Err(e) =>
                {
                    let error = TryUpdateMany ::
                    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserialize
                    {
                        update_many_payload_element_try_from_update_many_payload_element_with_serialize_deserialize
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ; crate :: common :: error_logs_logic :: error_log ::
                    ErrorLog :: error_log(& error, app_info_state.as_ref(),) ;
                    return TryUpdateManyResponseVariants :: from(error) ;
                }
            }, Err(err) => { return err ; }
        },
    } ;
    println!("{:#?}", parameters);
    {
        {
            let not_unique_primary_keys = {
                let mut vec = std::vec::Vec::with_capacity(parameters.payload.len());
                let mut not_unique_primary_keys =
                    std::vec::Vec::with_capacity(parameters.payload.len());
                for element in &parameters.payload {
                    let handle = &element.id;
                    match vec.contains(&handle) {
                        true => {
                            not_unique_primary_keys.push(element.id.clone());
                        }
                        false => {
                            vec.push(&element.id);
                        }
                    }
                }
                not_unique_primary_keys
            };
            if let false = not_unique_primary_keys.is_empty() {
                let error = TryUpdateMany::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateManyResponseVariants::from(error);
            }
        }
        let expected_updated_primary_keys = {
            parameters
                .payload
                .iter()
                .map(|element| element.id.clone())
                .collect::<std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>>()
        };
        let binded_query = {
            let query_string = {
                "update dogs as t set name = data.name, color = data.color from (select * from unnest($1, $2, $3)) as data(id, name, color) where t.id = data.id returning data.id"
            };
            println!("{}", query_string);
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.len();
            let (id_vec, name_vec, color_vec) = parameters.payload.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.id);
                    acc.1.push(element.name);
                    acc.2.push(element.color);
                    acc
                },
            );
            query = query.bind(
                id_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<sqlx::types::Uuid>>(),
            );
            query = query.bind(name_vec);
            query = query.bind(color_vec);
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateManyResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateManyResponseVariants::from(error);
            }
        };
        let mut postgres_transaction = match {
            use sqlx::Acquire;
            pg_connection.begin()
        }
        .await
        {
            Ok(value) => value,
            Err(e) => {
                let error = TryUpdateMany::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryUpdateManyResponseVariants::from(error);
            }
        };
        let results_vec = {
            let mut results_vec = std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            let mut option_error: Option<sqlx::Error> = None;
            {
                let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                while let (Some(Some(row)), None) = (
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => Some(value),
                        Err(e) => {
                            option_error = Some(e);
                            None
                        }
                    },
                    &option_error,
                ) {
                    results_vec.push(row);
                }
            }
            if let Some(e) = option_error {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateMany::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateManyResponseVariants::from(error);
                    }
                    Err(rollback_error) => {
                        let error = TryUpdateMany::QueryAndRollbackFailed {
                            query_error: e,
                            rollback_error,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateManyResponseVariants::from(error);
                    }
                }
            }
            results_vec
        };
        let primary_key_vec = {
            let mut primary_key_vec =
                std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            for element in results_vec {
                match primary_key_uuid_wrapper_try_from_sqlx_row(&element) {
                    Ok(primary_key) => {
                        primary_key_vec.push(primary_key);
                    }
                    Err(e) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateMany::from(e);
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(error);
                        }
                        Err(rollback_error) => {
                            let error = TryUpdateMany::PrimaryKeyFromRowAndFailedRollback {
                                primary_key_from_row: e,
                                rollback_error,
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(error);
                        }
                    },
                }
            }
            primary_key_vec
        };
        {
            let non_existing_primary_keys = {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(
                    std::vec::Vec::with_capacity(len),
                    |mut acc, element| {
                        if let false = primary_key_vec.contains(&element) {
                            acc.push(element);
                        }
                        acc
                    },
                )
            };
            if let false = non_existing_primary_keys.is_empty() {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateMany::NonExistingPrimaryKeys {
                            non_existing_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateManyResponseVariants::from(error);
                    }
                    Err(e) => {
                        let error = TryUpdateMany::NonExistingPrimaryKeysAndFailedRollback {
                            non_existing_primary_keys,
                            rollback_error: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryUpdateManyResponseVariants::from(error);
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => TryUpdateManyResponseVariants::Desirable(
                primary_key_vec
                    .into_iter()
                    .map(|element| {
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(element)
                    })
                    .collect(),
            ),
            Err(e) => {
                let error = TryUpdateMany::CommitFailed {
                    commit_failed: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                TryUpdateManyResponseVariants::from(error)
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryUpdateMany
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct DeleteOneParameters {
    pub path: DeleteOnePath,
}
#[derive(Debug)]
pub struct DeleteOnePath {
    pub id: crate::server::postgres::uuid_wrapper::UuidWrapper,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteOnePathWithSerializeDeserialize {
    pub id: crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteOnePathWithSerializeDeserialize> for DeleteOnePath {
    type Error = DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamed;
    fn try_from(value: DeleteOnePathWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value.id) {
            Ok(value) => Ok(Self { id: value }),
            Err(e) => Err(Self::Error::NotUuid {
                not_uuid: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryDeleteOneErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryDeleteOneRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper_in_client:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize {
        #[eo_error_occurence]
        delete_one_path_try_from_delete_one_path_with_serialize_deserialize:
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneResponseVariants {
    Desirable(crate :: server :: postgres :: uuid_wrapper ::
    PossibleUuidWrapper), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
    {
        delete_one_path_try_from_delete_one_path_with_serialize_deserialize :
        DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryDeleteOne> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence } => Self ::
            MissingPathParams { missing_path_params, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteOneResponseVariants> for http::StatusCode {
    fn from(value: &TryDeleteOneResponseVariants) -> Self {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(_) => http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: Io { io : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            Decode { decode : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteOneResponseVariants ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            MissingPathParams { missing_path_params : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteOneResponseVariantsTvfrr200Ok {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr200Ok> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, FailedToDeserializePathParams
    {
        failed_to_deserialize_path_params : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, MissingPathParams
    {
        missing_path_params : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
    {
        delete_one_path_try_from_delete_one_path_with_serialize_deserialize :
        DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr400BadRequest>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } => Self ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            MissingPathParams { missing_path_params, code_occurence } => Self
            :: MissingPathParams { missing_path_params, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            }, TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr404NotFound>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr500InternalServerError>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
async fn try_from_response_try_delete_one(
    response: reqwest::Response,
) -> Result<
    TryDeleteOneResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteOneResponseVariantsTvfrr200Ok > (& response_text)
            {
                Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteOneResponseVariantsTvfrr500InternalServerError >
            (& response_text)
            {
                Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteOneResponseVariantsTvfrr404NotFound > (& response_text)
            {
                Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteOneResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) => Ok(TryDeleteOneResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryDeleteOneResponseVariants>
    for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
{
    type Error = TryDeleteOneWithSerializeDeserialize;
    fn try_from(value: TryDeleteOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryDeleteOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryDeleteOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryDeleteOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryDeleteOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryDeleteOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryDeleteOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryDeleteOneResponseVariants ::
            UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryDeleteOneResponseVariants
            :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            FailedToDeserializePathParams
            { failed_to_deserialize_path_params, code_occurence }),
            TryDeleteOneResponseVariants :: MissingPathParams
            { missing_path_params, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: MissingPathParams
            { missing_path_params, code_occurence }),
            TryDeleteOneResponseVariants ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize,
                code_occurence
            }), TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryDeleteOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryDeleteOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryDeleteOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryDeleteOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteOneWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_delete_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryDeleteOneRequestError> {
    match future.await
    {
        Ok(response) => match try_from_response_try_delete_one(response).await
        {
            Ok(variants) => match crate :: server :: postgres :: uuid_wrapper
            :: PossibleUuidWrapper :: try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryDeleteOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryDeleteOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryDeleteOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryDeleteOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryDeleteOneRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryDeleteOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    FailedToDeserializePathParamsTvfrr400BadRequest,
    MissingPathParamsTvfrr400BadRequest,
    DeleteOnePathTryFromDeleteOnePathWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: FailedToDeserializePathParams
            { failed_to_deserialize_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: MissingPathParams
            { missing_path_params : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize
            {
                delete_one_path_try_from_delete_one_path_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_delete_one<'a>(
    server_location: &str,
    parameters: DeleteOneParameters,
) -> Result<crate::server::postgres::uuid_wrapper::UuidWrapper, TryDeleteOneErrorNamed> {
    let url = format!("{}/dogs/{}", server_location, parameters.path.id);
    match
    tvfrr_extraction_logic_try_delete_one(reqwest :: Client ::
    new().delete(&
    url).header(crate :: common :: git :: project_git_info :: PROJECT_COMMIT,
    crate :: global_variables :: compile_time :: project_git_info ::
    PROJECT_GIT_INFO.project_commit,).send()).await
    {
        Ok(value) => match crate :: server :: postgres :: uuid_wrapper ::
        UuidWrapper :: try_from(value)
        {
            Ok(value) => Ok(value), Err(e) =>
            Err(TryDeleteOneErrorNamed ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_client : e,
                code_occurence : crate :: code_occurence_tufa_common! (),
            })
        }, Err(e) =>
        Err(TryDeleteOneErrorNamed :: RequestError
        {
            request_error : e, code_occurence : crate ::
            code_occurence_tufa_common! (),
        }),
    }
}
pub async fn delete_one<'a>(
    path_extraction_result: Result<
        axum::extract::Path<DeleteOnePathWithSerializeDeserialize>,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state : axum :: extract :: State < crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
DynArcGetConfigGetPostgresPoolSendSync >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteOneParameters {
        path: match crate::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            DeleteOnePathWithSerializeDeserialize,
            TryDeleteOneResponseVariants,
        >::try_extract_value(path_extraction_result, &app_info_state)
        {
            Ok(value) => match DeleteOnePath::try_from(value) {
                Ok(value) => value,
                Err(e) => {
                    let error =
                        TryDeleteOne::DeleteOnePathTryFromDeleteOnePathWithSerializeDeserialize {
                            delete_one_path_try_from_delete_one_path_with_serialize_deserialize: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryDeleteOneResponseVariants::from(error);
                }
            },
            Err(err) => {
                return err;
            }
        },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            let mut query = format!("delete from dogs where");
            query.push_str(&format!(" id = $1"));
            query.push_str(&format!(" returning id"));
            query
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                parameters.path.id,
                query,
            );
            query
        };
        let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryDeleteOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryDeleteOneResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryDeleteOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryDeleteOneResponseVariants::from(error);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => TryDeleteOneResponseVariants::Desirable(
                    crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                ),
                Err(e) => {
                    let error = TryDeleteOne ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryDeleteOneResponseVariants::from(error);
                }
            },
            Err(e) => {
                let error = TryDeleteOne::from(e);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info_state.as_ref(),
                );
                return TryDeleteOneResponseVariants::from(error);
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryDeleteOne
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug)]
pub struct DeleteManyWithBodyParameters {
    pub payload: DeleteManyWithBodyPayload,
}
#[derive(Debug)]
pub struct DeleteManyWithBodyPayload {
    pub id: std::option::Option<std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>>,
    pub name:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteManyWithBodyPayloadWithSerializeDeserialize {
    pub id: std::option::Option<
        std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    >,
    pub name:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
    pub color:
        std::option::Option<std::vec::Vec<crate::server::postgres::regex_filter::RegexFilter>>,
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamed
{
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteManyWithBodyPayloadWithSerializeDeserialize>
    for DeleteManyWithBodyPayload
{
    type Error =
        DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: DeleteManyWithBodyPayloadWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let id = match value.id
        {
            Some(value) => match
            value.into_iter().map(| element | crate :: server :: postgres ::
            uuid_wrapper :: UuidWrapper :: try_from(element)).collect :: <
            Result < std :: vec :: Vec < crate :: server :: postgres ::
            uuid_wrapper :: UuidWrapper >, crate :: server :: postgres ::
            uuid_wrapper :: UuidWrapperTryFromPossibleUuidWrapperErrorNamed >>
            ()
            {
                Ok(value) => Some(value), Err(e) =>
                {
                    return
                    Err(Self :: Error :: NotUuid
                    {
                        not_uuid : e, code_occurence : crate ::
                        code_occurence_tufa_common! (),
                    }) ;
                },
            }, None => None,
        } ;
        let name = value.name;
        let color = value.color;
        Ok(Self { id, name, color })
    }
}
impl std::convert::From<DeleteManyWithBodyPayload>
    for DeleteManyWithBodyPayloadWithSerializeDeserialize
{
    fn from(value: DeleteManyWithBodyPayload) -> Self {
        let id =
            match value.id {
                Some(value) => {
                    Some(
                        value
                            .into_iter()
                            .map(|element| {
                                crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(
                                    element,
                                )
                            })
                            .collect::<std::vec::Vec<
                                crate::server::postgres::uuid_wrapper::PossibleUuidWrapper,
                            >>(),
                    )
                }
                None => None,
            };
        let name = value.name;
        let color = value.color;
        Self { id, name, color }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryDeleteManyWithBodyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryDeleteManyWithBodyRequestError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteManyWithBody {
    Configuration
    {
        #[eo_display_with_serialize_deserialize] configuration : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Database
    {
        #[eo_display_with_serialize_deserialize] database : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Io
    {
        #[eo_display] io : std :: io :: Error, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Tls
    {
        #[eo_display_with_serialize_deserialize] tls : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, Protocol
    {
        #[eo_display_with_serialize_deserialize] protocol : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, RowNotFound
    {
        #[eo_display_with_serialize_deserialize] row_not_found : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, TypeNotFound
    {
        #[eo_display_with_serialize_deserialize] type_not_found : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        #[eo_display_with_serialize_deserialize] column_index_out_of_bounds :
        usize, #[eo_display_with_serialize_deserialize] len : usize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        #[eo_display_with_serialize_deserialize] column_not_found : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, ColumnDecode
    {
        #[eo_display_with_serialize_deserialize] column_decode_index : std ::
        string :: String, #[eo_display_with_serialize_deserialize]
        source_handle : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Decode
    {
        #[eo_display_with_serialize_deserialize] decode : std :: string ::
        String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, PoolTimedOut
    {
        #[eo_display_with_serialize_deserialize] pool_timed_out : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, PoolClosed
    {
        #[eo_display_with_serialize_deserialize] pool_closed : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, WorkerCrashed
    {
        #[eo_display_with_serialize_deserialize] worker_crashed : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, Migrate
    {
        #[eo_display] migrate : sqlx :: migrate :: MigrateError,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        #[eo_display_with_serialize_deserialize] unexpected_case : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, JsonDataError
    {
        #[eo_display] json_data_error : axum :: extract :: rejection ::
        JsonDataError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, JsonSyntaxError
    {
        #[eo_display] json_syntax_error : axum :: extract :: rejection ::
        JsonSyntaxError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, MissingJsonContentType
    {
        #[eo_display_with_serialize_deserialize] missing_json_content_type :
        std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, BytesRejection
    {
        #[eo_display_with_serialize_deserialize] bytes_rejection : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, NotUniqueIdVec
    {
        #[eo_vec_display_with_serialize_deserialize] not_unique_id_vec : std
        :: vec :: Vec < crate :: server :: postgres :: regex_filter ::
        RegexFilter >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NotUniqueNameVec
    {
        #[eo_vec_display_with_serialize_deserialize] not_unique_name_vec : std
        :: vec :: Vec < crate :: server :: postgres :: regex_filter ::
        RegexFilter >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NotUniqueColorVec
    {
        #[eo_vec_display_with_serialize_deserialize] not_unique_color_vec :
        std :: vec :: Vec < crate :: server :: postgres :: regex_filter ::
        RegexFilter >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NotUniquePrimaryKeys
    {
        #[eo_vec_display] not_unique_primary_keys : std :: vec :: Vec < crate
        :: server :: postgres :: uuid_wrapper :: UuidWrapper >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        #[eo_error_occurence] bind_query : crate :: server :: postgres ::
        bind_query :: TryGenerateBindIncrementsErrorNamed, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        #[eo_display_with_serialize_deserialize] no_payload_fields : std ::
        string :: String, code_occurence : crate :: common :: code_occurence
        :: CodeOccurence
    }, NoPayloadParameters
    {
        #[eo_display_with_serialize_deserialize] no_payload_parameters : std
        :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        #[eo_vec_display] non_existing_primary_keys : std :: vec :: Vec <
        crate :: server :: postgres :: uuid_wrapper :: UuidWrapper >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        #[eo_vec_display] non_existing_primary_keys : std :: vec :: Vec <
        crate :: server :: postgres :: uuid_wrapper :: UuidWrapper >,
        #[eo_display] rollback_error : sqlx :: Error, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        #[eo_display] primary_key_from_row : sqlx :: Error, #[eo_display]
        rollback_error : sqlx :: Error, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, CommitFailed
    {
        #[eo_display] commit_failed : sqlx :: Error, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        #[eo_display] query_error : sqlx :: Error, #[eo_display]
        rollback_error : sqlx :: Error, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    },
    DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
    {
        #[eo_error_occurence]
        delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
        :
        DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : sqlx :: Error, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        #[eo_display_with_serialize_deserialize] project_commit_not_equal :
        std :: string :: String, #[eo_display_with_serialize_deserialize]
        project_commit_to_use : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        #[eo_display] project_commit_to_str_conversion : http :: header ::
        ToStrError, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        #[eo_display_with_serialize_deserialize] no_project_commit_header :
        std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyWithBodyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, RowNotFound
    {
        row_not_found : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec : std :: vec :: Vec < crate :: server :: postgres ::
        regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NotUniqueNameVec
    {
        not_unique_name_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniqueColorVec
    {
        not_unique_color_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadParameters
    {
        no_payload_parameters : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, rollback_error : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    },
    DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
    {
        delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
        :
        DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryDeleteManyWithBody> for TryDeleteManyWithBodyResponseVariants {
    fn from(value: TryDeleteManyWithBody) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteManyWithBodyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence } => Self :: PoolTimedOut
            { pool_timed_out, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } => Self :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: NotUniqueNameVec
            { not_unique_name_vec, code_occurence } => Self ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: NotUniqueColorVec
            { not_unique_color_vec, code_occurence } => Self ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            NoPayloadParameters { no_payload_parameters, code_occurence } =>
            Self :: NoPayloadParameters
            { no_payload_parameters, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence } => Self :: CommitFailed
            { commit_failed, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteManyWithBodyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryDeleteManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryDeleteManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryDeleteManyWithBodyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteManyWithBodyResponseVariants> for http::StatusCode {
    fn from(value: &TryDeleteManyWithBodyResponseVariants) -> Self {
        match value
        {
            TryDeleteManyWithBodyResponseVariants :: Desirable(_) => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            Database { database : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants :: Io
            { io : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => http :: StatusCode
            :: OK, TryDeleteManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => http
            :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NotUniqueIdVec { not_unique_id_vec : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NotUniqueNameVec { not_unique_name_vec : _, code_occurence : _ }
            => http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants
            :: NotUniqueColorVec
            { not_unique_color_vec : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NoPayloadFields { no_payload_fields : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } => http ::
            StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } => http :: StatusCode ::
            OK, TryDeleteManyWithBodyResponseVariants ::
            QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } => http :: StatusCode :: OK,
            TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            http :: StatusCode :: OK, TryDeleteManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } => http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteManyWithBodyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr200Ok>
    for TryDeleteManyWithBodyResponseVariants
{
    fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteManyWithBodyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteManyWithBodyResponseVariants
{
    fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Database
    {
        database : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Tls
    {
        tls : std :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Protocol
    {
        protocol : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize < >, len : usize < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std :: string :: String < >, source_handle : std
        :: string :: String < >, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, Decode
    {
        decode : std :: string :: String < >, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, PoolClosed
    {
        pool_closed : std :: string :: String < >, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, BindQuery
    {
        bind_query : crate :: server :: postgres :: bind_query ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError>
    for TryDeleteManyWithBodyResponseVariants
{
    fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Tls { tls, code_occurence } => Self :: Tls
            { tls, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Decode { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: PoolClosed { pool_closed, code_occurence } => Self ::
            PoolClosed { pool_closed, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr500InternalServerError
            ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std :: string :: String < >, code_occurence : crate
        :: common :: code_occurence :: CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence : crate ::
        common :: code_occurence :: CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std :: string :: String < >,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec : std :: vec :: Vec < crate :: server :: postgres ::
        regex_filter :: RegexFilter < >>, code_occurence : crate :: common ::
        code_occurence :: CodeOccurence
    }, NotUniqueNameVec
    {
        not_unique_name_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniqueColorVec
    {
        not_unique_color_vec : std :: vec :: Vec < crate :: server :: postgres
        :: regex_filter :: RegexFilter < >>, code_occurence : crate :: common
        :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std :: vec :: Vec < std :: string :: String
        >, code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NoPayloadParameters
    {
        no_payload_parameters : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, code_occurence : crate :: common :: code_occurence ::
        CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String >, rollback_error : std :: string :: String, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    },
    DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
    {
        delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
        :
        DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorNotEqual
    {
        project_commit_not_equal : std :: string :: String < >,
        project_commit_to_use : std :: string :: String < >, code_occurence :
        crate :: common :: code_occurence :: CodeOccurence
    }, ProjectCommitExtractorToStrConversion
    {
        project_commit_to_str_conversion : std :: string :: String,
        code_occurence : crate :: common :: code_occurence :: CodeOccurence
    }, NoProjectCommitExtractorHeader
    {
        no_project_commit_header : std :: string :: String < >, code_occurence
        : crate :: common :: code_occurence :: CodeOccurence
    }
}
impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest>
    for TryDeleteManyWithBodyResponseVariants
{
    fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
            JsonSyntaxError { json_syntax_error, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueIdVec { not_unique_id_vec, code_occurence } => Self ::
            NotUniqueIdVec { not_unique_id_vec, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence } => Self
            :: NotUniqueNameVec { not_unique_name_vec, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence } => Self
            :: NotUniqueColorVec { not_unique_color_vec, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NoPayloadFields { no_payload_fields, code_occurence } => Self ::
            NoPayloadFields { no_payload_fields, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NoPayloadParameters { no_payload_parameters, code_occurence } =>
            Self :: NoPayloadParameters
            { no_payload_parameters, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
enum TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound>
    for TryDeleteManyWithBodyResponseVariants
{
    fn from(value: TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
async fn try_from_response_try_delete_many_with_body(
    response: reqwest::Response,
) -> Result<
    TryDeleteManyWithBodyResponseVariants,
    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
> {
    let status_code = response.status();
    let headers = response.headers().clone();
    if status_code == http::StatusCode::OK {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteManyWithBodyResponseVariantsTvfrr200Ok >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteManyWithBodyResponseVariantsTvfrr408RequestTimeout >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteManyWithBodyResponseVariantsTvfrr404NotFound >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match response.text().await
        {
            Ok(response_text) => match serde_json :: from_str :: <
            TryDeleteManyWithBodyResponseVariantsTvfrr400BadRequest >
            (& response_text)
            {
                Ok(value) =>
                Ok(TryDeleteManyWithBodyResponseVariants :: from(value)),
                Err(e) =>
                Err(crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde : e, status_code, headers, response_text }),
            }, Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: FailedToGetResponseText
            { reqwest : e, status_code, headers, }),
        }
    } else {
        match response.text().await
        {
            Ok(response_text) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ResponseText(response_text)
            },), Err(e) =>
            Err(crate :: common :: api_request_unexpected_error ::
            ApiRequestUnexpectedError :: StatusCode
            {
                status_code, headers, response_text_result : crate :: common
                :: api_request_unexpected_error :: ResponseTextResult ::
                ReqwestError(e),
            },),
        }
    }
}
impl TryFrom<TryDeleteManyWithBodyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryDeleteManyWithBodyWithSerializeDeserialize;
    fn try_from(value: TryDeleteManyWithBodyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteManyWithBodyResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteManyWithBodyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: Database
            { database, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Database
            { database, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: Io { io, code_occurence }
            =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteManyWithBodyResponseVariants ::
            Tls { tls, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteManyWithBodyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: RowNotFound
            { row_not_found, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: Decode
            { decode, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteManyWithBodyResponseVariants
            :: PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: PoolClosed
            { pool_closed, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: Migrate
            { migrate, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: JsonDataError
            { json_data_error, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            JsonSyntaxError { json_syntax_error, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            BytesRejection { bytes_rejection, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NotUniqueIdVec { not_unique_id_vec, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NotUniqueNameVec
            { not_unique_name_vec, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NotUniqueNameVec { not_unique_name_vec, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NotUniqueColorVec
            { not_unique_color_vec, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NotUniqueColorVec { not_unique_color_vec, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NoPayloadFields
            { no_payload_fields, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NoPayloadFields { no_payload_fields, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NoPayloadParameters
            { no_payload_parameters, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NoPayloadParameters { no_payload_parameters, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence }),
            TryDeleteManyWithBodyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence }),
            TryDeleteManyWithBodyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: CommitFailed
            { commit_failed, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence }),
            TryDeleteManyWithBodyResponseVariants :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }),
            TryDeleteManyWithBodyResponseVariants ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize,
                code_occurence
            }), TryDeleteManyWithBodyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }), TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }),
            TryDeleteManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } =>
            Err(TryDeleteManyWithBodyWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence :: ErrorOccurence)]
pub enum TryDeleteManyWithBodyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteManyWithBodyWithSerializeDeserialize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_delete_many_with_body<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    TryDeleteManyWithBodyRequestError,
> {
    match future.await
    {
        Ok(response) => match
        try_from_response_try_delete_many_with_body(response).await
        {
            Ok(variants) => match std :: vec :: Vec :: < crate :: server ::
            postgres :: uuid_wrapper :: PossibleUuidWrapper > ::
            try_from(variants)
            {
                Ok(value) => Ok(value), Err(e) =>
                Err(TryDeleteManyWithBodyRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : crate ::
                    code_occurence_tufa_common! (),
                }),
            }, Err(e) => match e
            {
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: StatusCode
                { status_code, headers, response_text_result, } =>
                Err(TryDeleteManyWithBodyRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryDeleteManyWithBodyRequestError ::
                FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }), crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryDeleteManyWithBodyRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    crate :: code_occurence_tufa_common! ()
                }),
            },
        }, Err(e) =>
        Err(TryDeleteManyWithBodyRequestError :: Reqwest
        {
            reqwest : e, code_occurence : crate :: code_occurence_tufa_common!
            (),
        }),
    }
}
pub enum TryDeleteManyWithBodyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    NotUniqueIdVecTvfrr400BadRequest,
    NotUniqueNameVecTvfrr400BadRequest,
    NotUniqueColorVecTvfrr400BadRequest,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    NoPayloadParametersTvfrr400BadRequest,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    CommitFailedTvfrr500InternalServerError,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteManyWithBodyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteManyWithBodyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            } TryDeleteManyWithBodyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NotUniqueNameVec
            { not_unique_name_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NotUniqueColorVec
            { not_unique_color_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
            {
                delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal : _, project_commit_to_use : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }, TryDeleteManyWithBodyResponseVariants ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = http :: StatusCode :: OK ; res
            }
        }
    }
}
pub async fn try_delete_many_with_body<'a>(
    server_location: &str,
    parameters: DeleteManyWithBodyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryDeleteManyWithBodyErrorNamed,
> {
    let payload = match serde_json::to_string(
        &DeleteManyWithBodyPayloadWithSerializeDeserialize::from(parameters.payload),
    ) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryDeleteManyWithBodyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: crate::code_occurence_tufa_common!(),
            });
        }
    };
    let url = format!("{}/dogs/search", server_location,);
    match tvfrr_extraction_logic_try_delete_many_with_body(
        reqwest::Client::new()
            .delete(&url)
            .header(
                crate::common::git::project_git_info::PROJECT_COMMIT,
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit,
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send(),
    )
    .await
    {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryDeleteManyWithBodyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : crate ::
                    code_occurence_tufa_common! ()
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => Err(TryDeleteManyWithBodyErrorNamed::RequestError {
            request_error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }),
    }
}
pub async fn delete_many_with_body<'a>(
    app_info_state : axum :: extract :: State < crate :: repositories_types ::
tufa_server :: routes :: api :: cats :: DynArcGetConfigGetPostgresPoolSendSync
>,
    payload_extraction_result: Result<
        axum::Json<DeleteManyWithBodyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteManyWithBodyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                DeleteManyWithBodyPayloadWithSerializeDeserialize,
                TryDeleteManyWithBodyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_info_state)
            {
                Ok(value) => match DeleteManyWithBodyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody ::
                    DeleteManyWithBodyPayloadTryFromDeleteManyWithBodyPayloadWithSerializeDeserialize
                    {
                        delete_many_with_body_payload_try_from_delete_many_with_body_payload_with_serialize_deserialize
                        : e, code_occurence : crate :: code_occurence_tufa_common!
                        (),
                    } ;
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                },
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None, None) = (
            &parameters.payload.id,
            &parameters.payload.name,
            &parameters.payload.color,
        ) {
            return TryDeleteManyWithBodyResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
        }
        match (
            &parameters.payload.id,
            &parameters.payload.name,
            &parameters.payload.color,
        ) {
            (Some(id), None, None) => {
                let not_unique_primary_keys = {
                    let mut vec = std::vec::Vec::with_capacity(id.len());
                    let mut not_unique_primary_keys = std::vec::Vec::with_capacity(id.len());
                    for element in id {
                        let handle = element;
                        match vec.contains(&handle) {
                            true => {
                                not_unique_primary_keys.push(element.clone());
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    not_unique_primary_keys
                };
                if let false = not_unique_primary_keys.is_empty() {
                    let error = TryDeleteManyWithBody::NotUniquePrimaryKeys {
                        not_unique_primary_keys,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                    crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &error,
                        app_info_state.as_ref(),
                    );
                    return TryDeleteManyWithBodyResponseVariants::from(error);
                }
                let expected_updated_primary_keys = {
                    id.iter().map(| element | element.clone()).collect :: < std
                    :: vec :: Vec < crate :: server :: postgres :: uuid_wrapper
                    :: UuidWrapper > > ()
                };
                let binded_query = {
                    let query_string =
                        { "delete from dogs where id in (select unnest($1)) returning id" };
                    println!("{}", query_string);
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    query = query.bind(
                        id.into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<std::vec::Vec<sqlx::types::Uuid>>(),
                    );
                    query
                };
                let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                };
                let mut postgres_transaction = match {
                    use sqlx::Acquire;
                    pg_connection.begin()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                };
                let results_vec = {
                    let mut results_vec =
                        std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                    let mut option_error: Option<sqlx::Error> = None;
                    {
                        let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                        while let (Some(Some(row)), None) = (
                            match {
                                use futures::TryStreamExt;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(value) => Some(value),
                                Err(e) => {
                                    option_error = Some(e);
                                    None
                                }
                            },
                            &option_error,
                        ) {
                            results_vec.push(row);
                        }
                    }
                    if let Some(e) = option_error {
                        match postgres_transaction.rollback().await {
                            Ok(_) => {
                                let error = TryDeleteManyWithBody::from(e);
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                            Err(rollback_error) => {
                                let error = TryDeleteManyWithBody::QueryAndRollbackFailed {
                                    query_error: e,
                                    rollback_error,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                    results_vec
                };
                let primary_key_vec =
                    {
                        let mut primary_key_vec =
                            std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                        for element in results_vec {
                            match primary_key_uuid_wrapper_try_from_sqlx_row(&element) {
                                Ok(primary_key) => {
                                    primary_key_vec.push(primary_key);
                                }
                                Err(e) => match postgres_transaction.rollback().await {
                                    Ok(_) => {
                                        let error = TryDeleteManyWithBody::from(e);
                                        crate ::
                                    common :: error_logs_logic :: error_log :: ErrorLog ::
                                    error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteManyWithBodyResponseVariants::from(error);
                                    }
                                    Err(rollback_error) => {
                                        let error = TryDeleteManyWithBody ::
                                    PrimaryKeyFromRowAndFailedRollback
                                    {
                                        primary_key_from_row : e, rollback_error, code_occurence :
                                        crate :: code_occurence_tufa_common! (),
                                    } ;
                                        crate :: common :: error_logs_logic :: error_log ::
                                    ErrorLog :: error_log(& error, app_info_state.as_ref(),) ;
                                        return TryDeleteManyWithBodyResponseVariants::from(error);
                                    }
                                },
                            }
                        }
                        primary_key_vec
                    };
                {
                    let non_existing_primary_keys = {
                        let len = expected_updated_primary_keys.len();
                        expected_updated_primary_keys.into_iter().fold(
                            std::vec::Vec::with_capacity(len),
                            |mut acc, element| {
                                if let false = primary_key_vec.contains(&element) {
                                    acc.push(element);
                                }
                                acc
                            },
                        )
                    };
                    if let false = non_existing_primary_keys.is_empty() {
                        match postgres_transaction.rollback().await {
                            Ok(_) => {
                                let error = TryDeleteManyWithBody::NonExistingPrimaryKeys {
                                    non_existing_primary_keys,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                            Err(e) => {
                                let error = TryDeleteManyWithBody ::
                                NonExistingPrimaryKeysAndFailedRollback
                                {
                                    non_existing_primary_keys, rollback_error : e,
                                    code_occurence : crate :: code_occurence_tufa_common! (),
                                } ;
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                }
                match postgres_transaction.commit().await {
                    Ok(_) => TryDeleteManyWithBodyResponseVariants::Desirable(
                        primary_key_vec
                            .into_iter()
                            .map(|element| {
                                crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(
                                    element,
                                )
                            })
                            .collect(),
                    ),
                    Err(e) => {
                        let error = TryDeleteManyWithBody::CommitFailed {
                            commit_failed: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        TryDeleteManyWithBodyResponseVariants::from(error)
                    }
                }
            }
            _ => {
                if let Some(id) = &parameters.payload.id {
                    let not_unique_primary_keys = {
                        let mut vec = std::vec::Vec::with_capacity(id.len());
                        let mut not_unique_primary_keys = std::vec::Vec::with_capacity(id.len());
                        for element in id {
                            let handle = element;
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(element.clone());
                                }
                                false => {
                                    vec.push(element);
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let error = TryDeleteManyWithBody::NotUniquePrimaryKeys {
                            not_unique_primary_keys,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        };
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                }
                let name_handle = match parameters.payload.name {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_name_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut not_unique_name_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_name_vec.push(element);
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_name_vec
                                };
                                let error = TryDeleteManyWithBody::NotUniqueNameVec {
                                    not_unique_name_vec,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                    None => None,
                };
                let color_handle = match parameters.payload.color {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_color_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut not_unique_color_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_color_vec.push(element);
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_color_vec
                                };
                                let error = TryDeleteManyWithBody::NotUniqueColorVec {
                                    not_unique_color_vec,
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                };
                                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                    &error,
                                    app_info_state.as_ref(),
                                );
                                return TryDeleteManyWithBodyResponseVariants::from(error);
                            }
                        }
                    }
                    None => None,
                };
                let query_string = {
                    format!("delete from dogs where {} returning id", {
                        let mut increment: u64 = 0;
                        let mut additional_parameters = std::string::String::default();
                        if let Some(value) = &name_handle {
                            match crate::server::postgres::bind_query::BindQuery::try_increment(
                                value,
                                &mut increment,
                            ) {
                                Ok(_) => {
                                    let handle = format!("name = ${increment}");
                                    match additional_parameters.is_empty() {
                                        true => {
                                            additional_parameters.push_str(&handle);
                                        }
                                        false => {
                                            additional_parameters
                                                .push_str(&format!(" AND {handle}"));
                                        }
                                    }
                                }
                                Err(e) => {
                                    return TryDeleteManyWithBodyResponseVariants::BindQuery {
                                        bind_query: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(value) = &color_handle {
                            match crate::server::postgres::bind_query::BindQuery::try_increment(
                                value,
                                &mut increment,
                            ) {
                                Ok(_) => {
                                    let handle = format!("color = ${increment}");
                                    match additional_parameters.is_empty() {
                                        true => {
                                            additional_parameters.push_str(&handle);
                                        }
                                        false => {
                                            additional_parameters
                                                .push_str(&format!(" AND {handle}"));
                                        }
                                    }
                                }
                                Err(e) => {
                                    return TryDeleteManyWithBodyResponseVariants::BindQuery {
                                        bind_query: e.into_serialize_deserialize_version(),
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    };
                                }
                            }
                        }
                        if let Some(id) = &parameters.payload.id {
                            if let false = additional_parameters.is_empty() {
                                additional_parameters.push_str(" and");
                            }
                            additional_parameters.push_str(& format!
                            (" id in ({})",
                            {
                                let mut additional_parameters = std :: string :: String ::
                                default() ; for element in id
                                {
                                    match crate :: server :: postgres :: bind_query :: BindQuery
                                    :: try_increment(element, & mut increment,)
                                    {
                                        Ok(_) =>
                                        {
                                            additional_parameters.push_str(& format! ("${increment},"))
                                            ;
                                        } Err(e) =>
                                        {
                                            return TryDeleteManyWithBodyResponseVariants :: BindQuery
                                            {
                                                bind_query : e.into_serialize_deserialize_version(),
                                                code_occurence : crate :: code_occurence_tufa_common! ()
                                            } ;
                                        }
                                    }
                                } additional_parameters.pop() ; additional_parameters
                            })) ;
                        }
                        additional_parameters
                    })
                };
                println!("{}", query_string);
                let binded_query = {
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    if let Some(value) = name_handle {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(value) = color_handle {
                        query = crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                            value, query,
                        );
                    }
                    if let Some(id) = parameters.payload.id {
                        for element in id {
                            query =
                                crate::server::postgres::bind_query::BindQuery::bind_value_to_query(
                                    element, query,
                                );
                        }
                    }
                    query
                };
                let mut pool_connection = match app_info_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryDeleteManyWithBody::from(e);
                        crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &error,
                            app_info_state.as_ref(),
                        );
                        return TryDeleteManyWithBodyResponseVariants::from(error);
                    }
                };
                let mut rows = binded_query.fetch(pg_connection.as_mut());
                let mut vec_values = std::vec::Vec::new();
                while let Some(row) = {
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => value,
                        Err(e) => {
                            let error = TryDeleteManyWithBody::from(e);
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryDeleteManyWithBodyResponseVariants::from(error);
                        }
                    }
                } {
                    match {
                        use sqlx::Row;
                        row.try_get::<sqlx::types::Uuid, &str>("id")
                    } {
                        Ok(value) => {
                            vec_values.push(
                                crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(
                                    value,
                                ),
                            );
                        }
                        Err(e) => {
                            let error = TryDeleteManyWithBody ::
                            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                            {
                                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                                : e, code_occurence : crate :: code_occurence_tufa_common!
                                (),
                            } ;
                            crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                                &error,
                                app_info_state.as_ref(),
                            );
                            return TryDeleteManyWithBodyResponseVariants::from(error);
                        }
                    }
                }
                TryDeleteManyWithBodyResponseVariants::Desirable(vec_values)
            }
        }
    }
}
impl
    std::convert::From<
        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed,
    > for TryDeleteManyWithBody
{
    fn from(
        value : crate :: server :: extractors :: project_commit_extractor ::
    ProjectCommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, crate :: server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence }, crate ::
            server :: extractors :: project_commit_extractor ::
            ProjectCommitExtractorCheckErrorNamed ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
