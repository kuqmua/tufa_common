#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum ThreeWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_error: ThreeWrapperErrorEnum<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for ThreeWrapperError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for ThreeWrapperError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime;
        match self {
            ThreeWrapperError::Something {
                inner_error,
                code_occurence: _code_occurence,
            } => inner_error.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for ThreeWrapperError<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        match self {
            ThreeWrapperError::Something { inner_error, code_occurence: _code_occurence } => inner_error.to_string_without_config_lifetime(),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for ThreeWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            ThreeWrapperError::Something {
                inner_error: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum ThreeWrapperErrorEnum<'a> {
    FourWrapper(FourWrapperError<'a>),
}

impl<'a> std::fmt::Display for ThreeWrapperErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for ThreeWrapperErrorEnum<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<'a>
    for ThreeWrapperErrorEnum<'a>
{
    fn to_string_without_config_lifetime(&self) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}

pub fn three<'a>() -> Result<(), Box<ThreeWrapperError<'a>>> {
    if let Err(e) = four() {
        let f = ThreeWrapperError::Something {
            inner_error: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FourWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FourWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for FourWrapperError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for FourWrapperError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
        match self {
            FourWrapperError::Something {
                inner_errors,
                code_occurence: _code_occurence,
            } => inner_errors.few_to_string_with_config(config),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for FourWrapperError<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetime;
        match self {
            FourWrapperError::Something { inner_errors, code_occurence: _code_occurence } => inner_errors.few_to_string_without_config_lifetime(),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for FourWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            FourWrapperError::Something {
                inner_errors: _inner_errors,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FourWrapperErrorEnum<'a> {
    FiveWrapper(FiveWrapperError<'a>),
    SixWrapper(SixWrapperError<'a>),
}

impl<'a> std::fmt::Display for FourWrapperErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for FourWrapperErrorEnum<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_with_config_lifetime(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<'a>
    for FourWrapperErrorEnum<'a>
{
    fn to_string_without_config_lifetime(&self) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.to_string_without_config_lifetime(),
            FourWrapperErrorEnum::SixWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}

pub fn four<'a>() -> Result<(), Box<FourWrapperError<'a>>> {
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = FourWrapperError::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s),
                    ),
                ]),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FiveWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: std::collections::HashMap<String, FiveWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for FiveWrapperError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for FiveWrapperError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
        match self {
            FiveWrapperError::Something {
                inner_errors,
                code_occurence: _code_occurence,
            } => inner_errors.few_to_string_with_config(config),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for FiveWrapperError<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetime;
        match self {
            FiveWrapperError::Something { inner_errors, code_occurence: _code_occurence } => inner_errors.few_to_string_without_config_lifetime(),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for FiveWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            FiveWrapperError::Something {
                inner_errors: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FiveWrapperErrorEnum<'a> {
    FiveOneOrigin(FiveOneOriginError<'a>),
}

impl<'a> std::fmt::Display for FiveWrapperErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for FiveWrapperErrorEnum<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfigLifetime;
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => {
                i.origin_to_string_with_config_lifetime(config)
            }
        }
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<'a>
    for FiveWrapperErrorEnum<'a>
{
    fn to_string_without_config_lifetime(&self) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.to_string_without_config_lifetime(),
        }
    }
}

pub fn five<'a>() -> Result<(), Box<FiveWrapperError<'a>>> {
    if let Err(e) = five_one() {
        let f = FiveWrapperError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                FiveWrapperErrorEnum::FiveOneOrigin(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FiveOneOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for FiveOneOriginError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for FiveOneOriginError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, _config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime;
        self.source_to_string_without_config_lifetime()
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for FiveOneOriginError<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            FiveOneOriginError::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for FiveOneOriginError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            FiveOneOriginError::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn five_one<'a>() -> Result<(), Box<FiveOneOriginError<'a>>> {
    return Err(Box::new(FiveOneOriginError::Something {
        error: String::from("five_one error"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum SixWrapperError<'a> {
    Something {
        //todo how to implement from for it?
        inner_errors: Vec<SixWrapperErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for SixWrapperError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for SixWrapperError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
        match self {
            SixWrapperError::Something {
                inner_errors,
                code_occurence: _code_occurence,
            } => inner_errors.few_to_string_with_config(config),
        }
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<
        'a,
    > for SixWrapperError<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetime;
        match self {
            SixWrapperError::Something {
                inner_errors,
                code_occurence: _code_occurence,
            } => inner_errors.few_to_string_without_config_lifetime(),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for SixWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            SixWrapperError::Something {
                inner_errors: _inner_errors,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum SixWrapperErrorEnum<'a> {
    #[serde(borrow)]
    SevenWrapper(SevenOriginError<'a>),
    #[serde(borrow)]
    EightWrapper(EightOriginError<'a>),
}

impl<'a> std::fmt::Display for SixWrapperErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for SixWrapperErrorEnum<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.to_string_with_config_lifetime(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<'a>
    for SixWrapperErrorEnum<'a>
{
    fn to_string_without_config_lifetime(&self) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.to_string_without_config_lifetime(),
            SixWrapperErrorEnum::EightWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}

pub fn six<'a>() -> Result<(), Box<SixWrapperError<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperError::Something {
                inner_errors: vec![
                    SixWrapperErrorEnum::SevenWrapper(*seven_error),
                    SixWrapperErrorEnum::EightWrapper(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum SevenOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for SevenOriginError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for SevenOriginError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, _config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime;
        self.source_to_string_without_config_lifetime()
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for SevenOriginError<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            SevenOriginError::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for SevenOriginError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            SevenOriginError::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn seven<'a>() -> Result<(), Box<SevenOriginError<'a>>> {
    return Err(Box::new(SevenOriginError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum EightOriginError<'a> {
    Something {
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for EightOriginError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for EightOriginError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, _config: &ConfigGeneric) -> String {
        use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime;
        self.source_to_string_without_config_lifetime()
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for EightOriginError<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            EightOriginError::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for EightOriginError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetime<'a> {
        match self {
            EightOriginError::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn eight<'a>() -> Result<(), Box<EightOriginError<'a>>> {
    return Err(Box::new(EightOriginError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}
