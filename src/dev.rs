use crate::common::source_and_code_occurence;
use crate::traits::error_display::{ErrorDisplayInner, ToStringHandle};
use crate::traits::get_code_occurence::GetCodeOccurenceAsString;
use crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVecHelper;
use crate::traits::get_source::GetSourceAsString;
use crate::traits::separator_symbol::SeparatorSymbol;
use itertools::Itertools;
use std::collections::HashMap;
use std::vec;

#[derive(Debug)]
pub struct ThreeWrapperError {
    source: ThreeWrapperErrorEnum,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for ThreeWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}{}",
            self.source,
            config.symbol(),
            self.get_code_occurence_as_string(config)
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for ThreeWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for ThreeWrapperError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let vec = self.source.get_inner_source_and_code_occurence_vec(config);
        let mut new_vec = Vec::with_capacity(vec.len() + 1);
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::with_capacity(
            vec.iter()
                .map(|e| e.source.len())
                .collect::<Vec<usize>>()
                .iter()
                .sum(),
        );
        vec.into_iter().for_each(|mut v| {
            v.source.iter().for_each(|f| {
                sources_for_tracing.push(f.clone());
            });
            v.add_one();
            new_vec.push(v);
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        new_vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        new_vec
    }
}

pub fn three(should_trace: bool) -> Result<(), Box<ThreeWrapperError>> {
    if let Err(e) = four(false) {
        return Err(Box::new(ThreeWrapperError {
            source: ThreeWrapperErrorEnum::FourWrapper(*e),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        }));
    };
    Ok(())
}

#[derive(Debug)]
pub enum ThreeWrapperErrorEnum {
    FourWrapper(FourWrapperError),
}

impl std::fmt::Display for ThreeWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone + crate::traits::fields::GetSourcePlaceType,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for ThreeWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            ThreeWrapperErrorEnum::FourWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
        }
    }
}

#[derive(Debug)]
pub struct FourWrapperError {
    source: HashMap<String, FourWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for FourWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}",
            self.source.to_string_handle(config),
            self.get_code_occurence_as_string(config)
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FourWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for FourWrapperError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let (sources_for_tracing, mut vec) = self
            .source
            .get_inner_source_and_code_occurence_vec_helper(config);
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
}

#[derive(Debug)]
pub enum FourWrapperErrorEnum {
    FiveWrapper(FiveWrapperError),
    SixWrapper(SixWrapperError),
}

impl std::fmt::Display for FourWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FourWrapperErrorEnum::FiveWrapper(e) => write!(f, "{}", e),
            FourWrapperErrorEnum::SixWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_source_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone + crate::traits::fields::GetSourcePlaceType,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => i.get_code_occurence_as_string(config),
            FourWrapperErrorEnum::SixWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for FourWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            FourWrapperErrorEnum::FiveWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
            FourWrapperErrorEnum::SixWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
        }
    }
}

pub fn four(should_trace: bool) -> Result<(), Box<FourWrapperError>> {
    match (five(false), six(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            return Err(Box::new(FourWrapperError {
                source: HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        FourWrapperErrorEnum::FiveWrapper(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        FourWrapperErrorEnum::SixWrapper(*s),
                    ),
                ]),
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            }));
        }
    }
}

#[derive(Debug)]
pub struct FiveWrapperError {
    source: HashMap<String, FiveWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for FiveWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}",
            self.source.to_string_handle(config),
            self.get_code_occurence_as_string(config)
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for FiveWrapperError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let (sources_for_tracing, mut vec) = self
            .source
            .get_inner_source_and_code_occurence_vec_helper(config);
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
}

#[derive(Debug)]
pub enum FiveWrapperErrorEnum {
    FiveOneOrigin(FiveOneOriginError),
}

impl std::fmt::Display for FiveWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone + crate::traits::fields::GetSourcePlaceType,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => i.get_code_occurence_as_string(config),
        }
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for FiveWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            FiveWrapperErrorEnum::FiveOneOrigin(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
        }
    }
}

pub fn five(should_trace: bool) -> Result<(), Box<FiveWrapperError>> {
    if let Err(e) = five_one(false) {
        return Err(Box::new(FiveWrapperError {
            source: HashMap::from([
                (
                    String::from("five_one_hashmap key"),
                    FiveWrapperErrorEnum::FiveOneOrigin(*e),
                )
            ]),
            code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
               String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        }));
    }
    Ok(())
}

#[derive(Debug)]
pub struct FiveOneOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

//must be written as proc_macro
impl std::fmt::Display for FiveOneOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl crate::traits::error_display::OriginSourceToString for FiveOneOriginError {
    fn origin_source_to_string(&self) -> String {
        format!("{}", self.source)
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for FiveOneOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.source)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for FiveOneOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for FiveOneOriginError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(config),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
}

pub fn five_one(should_trace: bool) -> Result<(), Box<FiveOneOriginError>> {
    return Err(Box::new(FiveOneOriginError {
        source: String::from("five_one error"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    }));
}

#[derive(Debug)]
pub struct SixWrapperError {
    source: Vec<SixWrapperErrorEnum>,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for SixWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        let symbol = config.symbol();
        write!(
            f,
            "{}{}{}",
            self.source.to_string_handle(config),
            symbol,
            self.get_code_occurence_as_string(config)
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SixWrapperError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for SixWrapperError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let (sources_for_tracing, mut vec) = self
            .source
            .get_inner_source_and_code_occurence_vec_helper(config);
        vec.push(
            crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
}

pub fn six(should_trace: bool) -> Result<(), Box<SixWrapperError>> {
    match (seven(false), eight(false)) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            return Err(Box::new(SixWrapperError {
                source: vec![SixWrapperErrorEnum::SevenWrapper(*seven_error), SixWrapperErrorEnum::EightWrapper(*eight_error)],
                code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
                    git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                        String::from(file!()),
                        line!(),
                        column!(),
                    ),
                }
            }));
        }
    }
}

#[derive(Debug)]
pub enum SixWrapperErrorEnum {
    SevenWrapper(SevenOriginError),
    EightWrapper(EightOriginError),
}

impl std::fmt::Display for SixWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SixWrapperErrorEnum::SevenWrapper(e) => write!(f, "{}", e),
            SixWrapperErrorEnum::EightWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            //todo - if wrapper - with config, if origin - without
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_source_as_string(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone + crate::traits::fields::GetSourcePlaceType,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => i.get_code_occurence_as_string(config),
            SixWrapperErrorEnum::EightWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for SixWrapperErrorEnum
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            SixWrapperErrorEnum::SevenWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
            SixWrapperErrorEnum::EightWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
        }
    }
}

#[derive(Debug)]
pub struct SevenOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

//must be written as proc_macro
impl std::fmt::Display for SevenOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl crate::traits::error_display::OriginSourceToString for SevenOriginError {
    fn origin_source_to_string(&self) -> String {
        format!("{}", self.source)
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for SevenOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.source)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for SevenOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for SevenOriginError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(config),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
}

pub fn seven(should_trace: bool) -> Result<(), Box<SevenOriginError>> {
    let f = SevenOriginError {
        source: String::from("error_seven"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    };
    return Err(Box::new(f));
}

#[derive(Debug)]
pub struct EightOriginError {
    source: String,
    code_occurence: crate::common::code_occurence::CodeOccurenceOldWay,
}

//must be written as proc_macro
impl std::fmt::Display for EightOriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.error_display_inner(once_cell::sync::Lazy::force(
                &crate::global_variables::runtime::config::CONFIG
            ))
        )
    }
}

impl crate::traits::error_display::OriginSourceToString for EightOriginError {
    fn origin_source_to_string(&self) -> String {
        format!("{}", self.source)
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for EightOriginError
where
    ConfigGeneric: crate::traits::fields::GetLogType + crate::traits::fields::GetSourcePlaceType,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        format!("{}", self.source)
    }
}

impl crate::traits::get_code_occurence::GetCodeOccurenceOldWay for EightOriginError {
    fn get_code_occurence_old_way(&self) -> &crate::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
    }
}

impl<ConfigGeneric>
    crate::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for EightOriginError
where
    ConfigGeneric: crate::traits::fields::GetTimezone
        + crate::traits::fields::GetLogType
        + crate::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        vec![
            crate::dev::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: vec![vec![(
                    crate::common::source_and_code_occurence::Source {
                        source: self.get_source_as_string(config),
                        uuid: uuid::Uuid::new_v4(),
                    },
                    vec![],
                )]],
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        ]
    }
}

pub fn eight(should_trace: bool) -> Result<(), Box<EightOriginError>> {
    return Err(Box::new(EightOriginError {
        source: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceOldWay {
            git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                String::from(file!()),
                line!(),
                column!(),
            ),
        }
    }));
}
