#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum ThreeWrapperErrorWithDeserialize<'a> {
    Something {
        //todo how to implement from for it?
        #[serde(borrow)]
        inner_error: ThreeWrapperErrorWithDeserializeEnumWithDeserialize<'a>,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for ThreeWrapperErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for ThreeWrapperErrorWithDeserialize<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            ThreeWrapperErrorWithDeserialize::Something { inner_error, code_occurence: _code_occurence } => inner_error.to_string_without_config_lifetime_with_deserialize(),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for ThreeWrapperErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            ThreeWrapperErrorWithDeserialize::Something {
                inner_error: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum ThreeWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    #[serde(borrow)]
    FourWrapper(FourWrapperErrorWithDeserialize<'a>),
}

impl<'a> std::fmt::Display for ThreeWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize<'a>
    for ThreeWrapperErrorWithDeserializeEnumWithDeserialize<'a>
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            ThreeWrapperErrorWithDeserializeEnumWithDeserialize::FourWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
        }
    }
}

pub fn three_with_deserialize<'a>() -> Result<(), Box<ThreeWrapperErrorWithDeserialize<'a>>> {
    if let Err(e) = four_with_deserialize() {
        let f = ThreeWrapperErrorWithDeserialize::Something {
            inner_error: ThreeWrapperErrorWithDeserializeEnumWithDeserialize::FourWrapper(*e),
            code_occurence:
                crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
                    &crate::global_variables::compile_time::git_info::GIT_INFO,
                    file!(),
                    line!(),
                    column!(),
                ),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum FourWrapperErrorWithDeserialize<'a> {
    Something {
        //todo how to implement from for it?
        #[serde(borrow)]
        inner_errors: std::collections::HashMap<
            String,
            FourWrapperErrorWithDeserializeEnumWithDeserialize<'a>,
        >,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for FourWrapperErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for FourWrapperErrorWithDeserialize<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            FourWrapperErrorWithDeserialize::Something { inner_errors, code_occurence: _code_occurence } => inner_errors.few_to_string_without_config_lifetime_with_deserialize(),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for FourWrapperErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            FourWrapperErrorWithDeserialize::Something {
                inner_errors: _inner_errors,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum FourWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    #[serde(borrow)]
    FiveWrapper(FiveWrapperErrorWithDeserialize<'a>),
    #[serde(borrow)]
    SixWrapper(SixWrapperErrorWithDeserialize<'a>),
}

impl<'a> std::fmt::Display for FourWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize<'a>
    for FourWrapperErrorWithDeserializeEnumWithDeserialize<'a>
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            FourWrapperErrorWithDeserializeEnumWithDeserialize::FiveWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
            FourWrapperErrorWithDeserializeEnumWithDeserialize::SixWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
        }
    }
}

pub fn four_with_deserialize<'a>() -> Result<(), Box<FourWrapperErrorWithDeserialize<'a>>> {
    match (five_with_deserialize(), six_with_deserialize()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = FourWrapperErrorWithDeserialize::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorWithDeserializeEnumWithDeserialize::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorWithDeserializeEnumWithDeserialize::SixWrapper(*s),
                    ),
                ]),
                code_occurence:
                    crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
                        &crate::global_variables::compile_time::git_info::GIT_INFO,
                        file!(),
                        line!(),
                        column!(),
                    ),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum FiveWrapperErrorWithDeserialize<'a> {
    Something {
        //todo how to implement from for it?
        #[serde(borrow)]
        inner_errors: std::collections::HashMap<
            String,
            FiveWrapperErrorWithDeserializeEnumWithDeserialize<'a>,
        >,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for FiveWrapperErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a> crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for FiveWrapperErrorWithDeserialize<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            FiveWrapperErrorWithDeserialize::Something { inner_errors, code_occurence: _code_occurence } => inner_errors.few_to_string_without_config_lifetime_with_deserialize(),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for FiveWrapperErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            FiveWrapperErrorWithDeserialize::Something {
                inner_errors: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum FiveWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    #[serde(borrow)]
    FiveOneOrigin(FiveOneOriginErrorWithDeserialize<'a>),
}

impl<'a> std::fmt::Display for FiveWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize<'a>
    for FiveWrapperErrorWithDeserializeEnumWithDeserialize<'a>
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            FiveWrapperErrorWithDeserializeEnumWithDeserialize::FiveOneOrigin(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
        }
    }
}

pub fn five_with_deserialize<'a>() -> Result<(), Box<FiveWrapperErrorWithDeserialize<'a>>> {
    if let Err(e) = five_one_with_deserialize() {
        let f = FiveWrapperErrorWithDeserialize::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                FiveWrapperErrorWithDeserializeEnumWithDeserialize::FiveOneOrigin(*e),
            )]),
            code_occurence:
                crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
                    &crate::global_variables::compile_time::git_info::GIT_INFO,
                    file!(),
                    line!(),
                    column!(),
                ),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum FiveOneOriginErrorWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for FiveOneOriginErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for FiveOneOriginErrorWithDeserialize<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            FiveOneOriginErrorWithDeserialize::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for FiveOneOriginErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            FiveOneOriginErrorWithDeserialize::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn five_one_with_deserialize<'a>() -> Result<(), Box<FiveOneOriginErrorWithDeserialize<'a>>> {
    return Err(Box::new(FiveOneOriginErrorWithDeserialize::Something {
        error: String::from("five_one error"),
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum SixWrapperErrorWithDeserialize<'a> {
    Something {
        //todo how to implement from for it?
        #[serde(borrow)]
        inner_errors: Vec<SixWrapperErrorWithDeserializeEnumWithDeserialize<'a>>,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for SixWrapperErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<
        'a,
    > for SixWrapperErrorWithDeserialize<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        use crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            SixWrapperErrorWithDeserialize::Something {
                inner_errors,
                code_occurence: _code_occurence,
            } => inner_errors.few_to_string_without_config_lifetime_with_deserialize(),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for SixWrapperErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            SixWrapperErrorWithDeserialize::Something {
                inner_errors: _inner_errors,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum SixWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    #[serde(borrow)]
    SevenWrapper(SevenOriginErrorWithDeserialize<'a>),
    #[serde(borrow)]
    EightWrapper(EightOriginErrorWithDeserialize<'a>),
}

impl<'a> std::fmt::Display for SixWrapperErrorWithDeserializeEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize<'a>
    for SixWrapperErrorWithDeserializeEnumWithDeserialize<'a>
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            SixWrapperErrorWithDeserializeEnumWithDeserialize::SevenWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
            SixWrapperErrorWithDeserializeEnumWithDeserialize::EightWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
        }
    }
}

pub fn six_with_deserialize<'a>() -> Result<(), Box<SixWrapperErrorWithDeserialize<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight_with_deserialize());
    let res = thread_join_handle.join().unwrap();
    match (seven_with_deserialize(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = SixWrapperErrorWithDeserialize::Something {
                inner_errors: vec![
                    SixWrapperErrorWithDeserializeEnumWithDeserialize::SevenWrapper(*seven_error),
                    SixWrapperErrorWithDeserializeEnumWithDeserialize::EightWrapper(*eight_error),
                ],
                code_occurence:
                    crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
                        &crate::global_variables::compile_time::git_info::GIT_INFO,
                        file!(),
                        line!(),
                        column!(),
                    ),
            };
            return Err(Box::new(f));
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum SevenOriginErrorWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for SevenOriginErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for SevenOriginErrorWithDeserialize<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            SevenOriginErrorWithDeserialize::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for SevenOriginErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            SevenOriginErrorWithDeserialize::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn seven_with_deserialize<'a>() -> Result<(), Box<SevenOriginErrorWithDeserialize<'a>>> {
    return Err(Box::new(SevenOriginErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum EightOriginErrorWithDeserialize<'a> {
    Something {
        error: String,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for EightOriginErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a>
    for EightOriginErrorWithDeserialize<'a>
{
    fn source_to_string_without_config_lifetime(&self) -> String {
        match self {
            EightOriginErrorWithDeserialize::Something {
                error,
                code_occurence: _code_occurence,
            } => format!("{}", error),
        }
    }
}

impl<'a> crate::traits::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for EightOriginErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            EightOriginErrorWithDeserialize::Something {
                error: _error,
                code_occurence,
            } => code_occurence,
        }
    }
}

pub fn eight_with_deserialize<'a>() -> Result<(), Box<EightOriginErrorWithDeserialize<'a>>> {
    return Err(Box::new(EightOriginErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}
