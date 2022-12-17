use crate::traits::code_path::CodePath;
use crate::traits::readable_time_string::ReadableTimeString;
use crate::traits::separator_symbol::SeparatorSymbol;

pub trait PrepareLogSourceAndCodeOccurence<ConfigGeneric> {
    fn prepare_log_source_and_code_occurence(&self, config_generic: &ConfigGeneric) -> String; //add Option<String>
}

impl<SelfGeneric, ConfigGeneric> PrepareLogSourceAndCodeOccurence<ConfigGeneric> for SelfGeneric
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    SelfGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_and_code_occurence(&self, config_generic: &ConfigGeneric) -> String {
        let capacity = self
            .get_code_occurence()
            .occurences
            .values()
            .fold(0, |mut acc, elem| {
                acc += elem.len();
                acc
            });
        let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
            Vec::with_capacity(capacity);
        self.get_code_occurence()
            .occurences
            .iter()
            .for_each(|(git_info, v)| {
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
        occurences.push(format!("{}{}", self.get_source(), log_type.symbol())); //here must be self.get_source_value().
                                                                                // occurences.push(format!("{}{}", self.get_source_value().prepare_log_source_and_code_occurence(), log_type.symbol()));
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

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric>
    PrepareLogSourceAndCodeOccurence<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_and_code_occurence(&self, config_generic: &ConfigGeneric) -> String {
        let log_type = config_generic.get_log_type();
        let mut prepared_log_handle =
            self.iter()
                .fold(String::from(""), |mut acc, (hashmap_key, hashmap_value)| {
                    let code_occurence = hashmap_value.get_code_occurence();
                    let capacity = code_occurence.occurences.values().fold(0, |mut acc, elem| {
                        acc += elem.len();
                        acc
                    });
                    let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
                        Vec::with_capacity(capacity);
                    code_occurence.occurences.iter().for_each(|(git_info, v)| {
                        v.iter().for_each(|e| {
                            vec.push(crate::common::code_occurence::OccurenceFilter {
                                increment: e.increment,
                                time: e.time_file_line_column.time,
                                occurence: e.time_file_line_column.get_code_path(
                                    git_info,
                                    config_generic.get_source_place_type(),
                                ),
                            })
                        })
                    });
                    //vec.reverse();//todo check reserve or not
                    vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                    let mut occurences = Vec::with_capacity(capacity + 1);
                    occurences.push(format!(
                        "[key: {}] {}{}",
                        hashmap_key,
                        hashmap_value.get_source(),
                        log_type.symbol()
                    ));
                    vec.into_iter().for_each(|e| {
                        occurences.push(format!(
                            " {} {}{}", //add first symbol here?
                            e.readable_time_string(),
                            e.occurence,
                            log_type.symbol()
                        ));
                    });
                    let mut occurence =
                        occurences.iter().fold(String::from(""), |mut acc, elem| {
                            acc.push_str(elem);
                            acc
                        });
                    log_type.pop_last(&mut occurence);
                    occurence = format!("{}{}", occurence, log_type.symbol());
                    acc.push_str(&occurence);
                    acc
                });
        prepared_log_handle.pop();
        prepared_log_handle
    }
}

// impl<VecElementGeneric, ConfigGeneric> PrepareLogSourceAndCodeOccurence<ConfigGeneric>
//     for Vec<VecElementGeneric>
// where
//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
//     VecElementGeneric: std::fmt::Display,
// {
//     fn prepare_log_source_and_code_occurence(&self, config_generic: ConfigGeneric) -> String {
//         let log_type = config_generic.get_log_type().symbol();
//         let mut prepared_log_handle = self.iter().fold(String::from(""), |mut acc, vec_element| {
//             format!("{}{}", occurence, log_type.symbol())
//         });
//         prepared_log_handle.pop();
//         prepared_log_handle
//     }
// }

impl<VecElementGeneric, ConfigGeneric> PrepareLogSourceAndCodeOccurence<ConfigGeneric>
    for Vec<VecElementGeneric>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    VecElementGeneric:
        crate::traits::get_source::GetSource + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_and_code_occurence(&self, config_generic: &ConfigGeneric) -> String {
        let log_type = config_generic.get_log_type();
        let mut prepared_log_handle = self.iter().fold(String::from(""), |mut acc, vec_element| {
            let code_occurence = vec_element.get_code_occurence();
            let capacity = code_occurence.occurences.values().fold(0, |mut acc, elem| {
                acc += elem.len();
                acc
            });
            let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
                Vec::with_capacity(capacity);
            code_occurence.occurences.iter().for_each(|(git_info, v)| {
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
            occurences.push(format!("{}{}", vec_element.get_source(), log_type.symbol()));
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
            occurence = format!("{}{}", occurence, log_type.symbol());
            acc.push_str(&occurence);
            acc
        });
        prepared_log_handle.pop();
        prepared_log_handle
    }
}
//
pub trait PrepareLogSourceInnerAndCodeOccurence<ConfigGeneric, SourceFieldGeneric> {
    fn prepare_log_source_inner_and_code_occurence(&self, config_generic: &ConfigGeneric)
        -> String; //add Option<String>
}

impl<SelfGeneric, ConfigGeneric, SourceFieldGeneric>
    PrepareLogSourceInnerAndCodeOccurence<ConfigGeneric, SourceFieldGeneric> for SelfGeneric
where
    SourceFieldGeneric:
        crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    SelfGeneric: crate::traits::get_source_value::GetSourceValue<SourceFieldGeneric>
        // crate::traits::get_source::GetSource
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_inner_and_code_occurence(
        &self,
        config_generic: &ConfigGeneric,
    ) -> String {
        let capacity = self
            .get_code_occurence()
            .occurences
            .values()
            .fold(0, |mut acc, elem| {
                acc += elem.len();
                acc
            });
        let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
            Vec::with_capacity(capacity);
        self.get_code_occurence()
            .occurences
            .iter()
            .for_each(|(git_info, v)| {
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
            "////{}{}////",
            self.get_source_value()
                .prepare_log_source_and_code_occurence(config_generic),
            log_type.symbol()
        )); //here must be self.get_source_value().
            // occurences.push(format!("{}{}", self.get_source_value().prepare_log_source_and_code_occurence(), log_type.symbol()));
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

impl<HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric, SourceFieldGeneric>
    PrepareLogSourceInnerAndCodeOccurence<ConfigGeneric, SourceFieldGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    SourceFieldGeneric:
        crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: crate::traits::get_source_value::GetSourceValue<SourceFieldGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_inner_and_code_occurence(
        &self,
        config_generic: &ConfigGeneric,
    ) -> String {
        let log_type = config_generic.get_log_type();
        let mut prepared_log_handle =
            self.iter()
                .fold(String::from(""), |mut acc, (hashmap_key, hashmap_value)| {
                    let code_occurence = hashmap_value.get_code_occurence();
                    let capacity = code_occurence.occurences.values().fold(0, |mut acc, elem| {
                        acc += elem.len();
                        acc
                    });
                    let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
                        Vec::with_capacity(capacity);
                    code_occurence.occurences.iter().for_each(|(git_info, v)| {
                        v.iter().for_each(|e| {
                            vec.push(crate::common::code_occurence::OccurenceFilter {
                                increment: e.increment,
                                time: e.time_file_line_column.time,
                                occurence: e.time_file_line_column.get_code_path(
                                    git_info,
                                    config_generic.get_source_place_type(),
                                ),
                            })
                        })
                    });
                    //vec.reverse();//todo check reserve or not
                    vec.sort_by(|a, b| a.increment.cmp(&b.increment));
                    let mut occurences = Vec::with_capacity(capacity + 1);
                    occurences.push(format!(
                        "[key: {}] {}{}",
                        hashmap_key,
                        hashmap_value
                            .get_source_value()
                            .prepare_log_source_and_code_occurence(config_generic),
                        log_type.symbol()
                    ));
                    vec.into_iter().for_each(|e| {
                        occurences.push(format!(
                            " {} {}{}", //add first symbol here?
                            e.readable_time_string(),
                            e.occurence,
                            log_type.symbol()
                        ));
                    });
                    let mut occurence =
                        occurences.iter().fold(String::from(""), |mut acc, elem| {
                            acc.push_str(elem);
                            acc
                        });
                    log_type.pop_last(&mut occurence);
                    occurence = format!("{}{}", occurence, log_type.symbol());
                    acc.push_str(&occurence);
                    acc
                });
        prepared_log_handle.pop();
        prepared_log_handle
    }
}

impl<VecElementGeneric, ConfigGeneric, SourceFieldGeneric>
    PrepareLogSourceInnerAndCodeOccurence<ConfigGeneric, SourceFieldGeneric>
    for Vec<VecElementGeneric>
where
    SourceFieldGeneric:
        crate::traits::prepare_log_source_and_code_occurence::PrepareLogSourceAndCodeOccurence<
            ConfigGeneric,
        >,
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetLogType,
    VecElementGeneric: crate::traits::get_source_value::GetSourceValue<SourceFieldGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn prepare_log_source_inner_and_code_occurence(
        &self,
        config_generic: &ConfigGeneric,
    ) -> String {
        let log_type = config_generic.get_log_type();
        let mut prepared_log_handle = self.iter().fold(String::from(""), |mut acc, vec_element| {
            let code_occurence = vec_element.get_code_occurence();
            let capacity = code_occurence.occurences.values().fold(0, |mut acc, elem| {
                acc += elem.len();
                acc
            });
            let mut vec: Vec<crate::common::code_occurence::OccurenceFilter> =
                Vec::with_capacity(capacity);
            code_occurence.occurences.iter().for_each(|(git_info, v)| {
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
            occurences.push(format!(
                "{}{}",
                vec_element
                    .get_source_value()
                    .prepare_log_source_and_code_occurence(config_generic),
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
            occurence = format!("{}{}", occurence, log_type.symbol());
            acc.push_str(&occurence);
            acc
        });
        prepared_log_handle.pop();
        prepared_log_handle
    }
}
