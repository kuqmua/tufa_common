use strum_macros::Display;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(Debug, thiserror::Error, serde::Serialize)] //, error_occurence::ImplErrorOccurence
pub enum InitSubcriberErrorEnum<'a> {
    SetLogger {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    SetGlobalDefault {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> std::fmt::Display for InitSubcriberErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kwko")
    }
}

// todo - implement conversion to serialize\Deserialize version

#[derive(Debug)]
pub struct Kekw {}
//, serde::Serialize, error_occurence::ImplErrorOccurence

#[derive(Debug, thiserror::Error)] //, error_occurence::ImplErrorOccurence
pub enum Seven<'a> {
    Something {
        // #[display_is_not_implemented]
        error: Kekw,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> Seven<'a> {
    pub fn into_serialize_deserialize_version(self) -> SevenWithDeserialize<'a> {
        match self {
            Seven::Something {
                error,
                code_occurence,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                SevenWithDeserialize::Something {
                    error: error.display_foreign_type(),
                    code_occurence: code_occurence.into_serialize_deserialize_version(),
                }
            }
        }
    }
}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> String {
        String::from("kekw")
    }
}

impl<'a> std::fmt::Display for Seven<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for SevenWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}
impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for Seven<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            Seven::Something {
                error: _unused_first_argument,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                self.source_to_string_without_config()
            }
        }
    }
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for Seven<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            Seven::Something {
                error,
                code_occurence: _unused_second_argument,
            } => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                error.display_foreign_type()
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a> for Seven<'a> {
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            Seven::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum SevenWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for SevenWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            SevenWithDeserialize::Something {
                error,
                code_occurence: _unused_second_argument,
            } => error.to_string(),
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for SevenWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            SevenWithDeserialize::Something {
                error: _unused_first_argument,
                code_occurence,
            } => code_occurence,
        }
    }
}

//
//
#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum OneError<'a> {
    Something {
        inner_error: TwoError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> OneError<'a> {
    pub fn into_serialize_deserialize_version(self) -> OneErrorWithDeserialize<'a> {
        match self {
            OneError::Something {
                inner_error,
                code_occurence,
            } => OneErrorWithDeserialize::Something {
                inner_error: inner_error.into_serialize_deserialize_version(),
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum TwoError<'a> {
    Something {
        inner_errors: std::collections::HashMap<String, TwoErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> TwoError<'a> {
    pub fn into_serialize_deserialize_version(self) -> TwoErrorWithDeserialize<'a> {
        match self {
            TwoError::Something {
                inner_errors,
                code_occurence,
            } => TwoErrorWithDeserialize::Something {
                inner_errors: inner_errors
                    .into_iter()
                    .map(|(k, v)| (k, v.into_serialize_deserialize_version()))
                    .collect(),
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum TwoErrorEnum<'a> {
    Three(ThreeError<'a>),
}

impl<'a> TwoErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> TwoErrorEnumWithDeserialize<'a> {
        match self {
            TwoErrorEnum::Three(i) => {
                TwoErrorEnumWithDeserialize::Three(i.into_serialize_deserialize_version())
            }
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum ThreeError<'a> {
    Something {
        inner_errors: std::vec::Vec<ThreeErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> ThreeError<'a> {
    pub fn into_serialize_deserialize_version(self) -> ThreeErrorWithDeserialize<'a> {
        match self {
            ThreeError::Something {
                inner_errors,
                code_occurence,
            } => ThreeErrorWithDeserialize::Something {
                inner_errors: inner_errors
                    .into_iter()
                    .map(|e| e.into_serialize_deserialize_version())
                    .collect(),
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //
pub enum ThreeErrorEnum<'a> {
    Seven(Seven<'a>),
}

impl<'a> ThreeErrorEnum<'a> {
    pub fn into_serialize_deserialize_version(self) -> ThreeErrorEnumWithDeserialize<'a> {
        match self {
            ThreeErrorEnum::Seven(i) => {
                ThreeErrorEnumWithDeserialize::Seven(i.into_serialize_deserialize_version())
            }
        }
    }
}

pub fn one<'a>() -> Result<(), Box<OneError<'a>>> {
    if let Err(e) = two() {
        let f = OneError::Something {
            inner_error: *e,
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn two<'a>() -> Result<(), Box<TwoError<'a>>> {
    if let Err(e) = three() {
        let f = TwoError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("four_hashmap_key"),
                TwoErrorEnum::Three(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn three<'a>() -> Result<(), Box<ThreeError<'a>>> {
    if let Err(e) = seven() {
        let f = ThreeError::Something {
            inner_errors: vec![ThreeErrorEnum::Seven(*e)],
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn seven<'a>() -> Result<(), Box<Seven<'a>>> {
    return Err(Box::new(Seven::Something {
        error: Kekw {},
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}
