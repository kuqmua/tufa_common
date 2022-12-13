pub trait CodeOccurenceNewErrorWithOneAddition<SourceGeneric> {
    fn new_error_with_one_addition(
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
        source_generic: &SourceGeneric,
    ) -> Self;
}

pub trait CodeOccurenceNew {
    fn new(
        git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self;
}

pub trait CodeOccurenceLog<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric> {
    fn log(&self, source: &SourceGeneric, config_generic: ConfigGeneric);
}

use crate::traits::code_path::CodePath;
use crate::traits::console::Console;
use crate::traits::readable_time_string::ReadableTimeString;
use crate::traits::separator_symbol::SeparatorSymbol;

impl<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric>
    crate::traits::code_occurence_methods::CodeOccurenceLog<
        ConfigGeneric,
        ErrorColorBoldGeneric,
        SourceGeneric,
    > for crate::common::code_occurence::CodeOccurence
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
    SourceGeneric: crate::traits::get_source::GetSource,
    Self: CodeOccurenceWithSourceToString<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric>,
{
    fn log(&self, source_generic: &SourceGeneric, config_generic: ConfigGeneric) {
        let log_type = config_generic.get_log_type();
        let error_color_bold = config_generic.get_error_color_bold();
        log_type.console(
            error_color_bold,
            self.code_occurence_with_source_to_string(source_generic, &config_generic),
        )
    }
}

pub trait CodeOccurenceWithSourceToString<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric> {
    fn code_occurence_with_source_to_string(
        &self,
        source: &SourceGeneric,
        config_generic: &ConfigGeneric,
    ) -> String;
}

impl<ConfigGeneric, ErrorColorBoldGeneric, SourceGeneric>
    crate::traits::code_occurence_methods::CodeOccurenceWithSourceToString<
        ConfigGeneric,
        ErrorColorBoldGeneric,
        SourceGeneric,
    > for crate::common::code_occurence::CodeOccurence
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
    SourceGeneric: crate::traits::get_source::GetSource,
{
    fn code_occurence_with_source_to_string(
        &self,
        source_generic: &SourceGeneric,
        config_generic: &ConfigGeneric,
    ) -> String {
        let capacity = self.occurences.values().fold(0, |mut acc, elem| {
            acc += elem.len();
            acc
        });
        let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
            Vec::with_capacity(capacity);
        self.occurences.iter().for_each(|(git_info, v)| {
            v.iter().for_each(|e| {
                vec.push(crate::common::code_occurence::OccurenceFilter {
                    increment: e.increment,
                    time: e.time_file_line_column.time,
                    occurence: e
                        .time_file_line_column
                        .get_code_path(git_info, config_generic.get_source_place_type()),
                })
            })
        });
        //vec.reverse();//todo check reserve or not
        vec.sort_by(|a, b| a.increment.cmp(&b.increment));
        let mut occurences = Vec::with_capacity(capacity + 1);
        let log_type = config_generic.get_log_type();
        occurences.push(format!(
            "{}{}",
            source_generic.get_source(),
            log_type.symbol()
        ));
        vec.into_iter().for_each(|e| {
            occurences.push(format!(
                "{} {}{}",
                e.readable_time_string(),
                e.occurence,
                log_type.symbol()
            ));
        });
        let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
            acc.push_str(elem);
            acc
        });
        log_type.pop_last(&mut occurence);
        occurence
    }
}

pub trait CodeOccurenceToString<ConfigGeneric, ErrorColorBoldGeneric> {
    fn code_occurence_to_string(&self, config_generic: &ConfigGeneric) -> String;
}

impl<ConfigGeneric, ErrorColorBoldGeneric>
    crate::traits::code_occurence_methods::CodeOccurenceToString<
        ConfigGeneric,
        ErrorColorBoldGeneric,
    > for crate::common::code_occurence::CodeOccurence
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetLogType
        + crate::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>,
{
    fn code_occurence_to_string(&self, config_generic: &ConfigGeneric) -> String {
        let capacity = self.occurences.values().fold(0, |mut acc, elem| {
            acc += elem.len();
            acc
        });
        let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
            Vec::with_capacity(capacity);
        self.occurences.iter().for_each(|(git_info, v)| {
            v.iter().for_each(|e| {
                vec.push(crate::common::code_occurence::OccurenceFilter {
                    increment: e.increment,
                    time: e.time_file_line_column.time,
                    occurence: e
                        .time_file_line_column
                        .get_code_path(git_info, config_generic.get_source_place_type()),
                })
            })
        });
        //vec.reverse();//todo check reserve or not
        vec.sort_by(|a, b| a.increment.cmp(&b.increment));
        let mut occurences = Vec::with_capacity(capacity + 1);
        let log_type = config_generic.get_log_type();
        vec.into_iter().for_each(|e| {
            occurences.push(format!(
                "{} {}{}",
                e.readable_time_string(),
                e.occurence,
                log_type.symbol()
            ));
        });
        let mut occurence = occurences.iter().fold(String::from(""), |mut acc, elem| {
            acc.push_str(elem);
            acc
        });
        log_type.pop_last(&mut occurence);
        occurence
    }
}
