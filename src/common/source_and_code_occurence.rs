#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurence {
    pub source: String,
    pub code_occurence: crate::common::code_occurence::CodeOccurence,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SourceAndCodeOccurenceAsString {
    pub source: Vec<Vec<(String, Vec<String>)>>, //only original
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub struct SourceTracingVec {
//     pub vec: Vec<std::collections::HashMap<String, Vec<String>>>,
// }

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub struct SourceTracingVecHm {
//     pub vec: Vec<(String, Vec<String>)>,
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceSourceWithKeys {
//     pub source: SourceWithKeys,
//     pub code_occurence: String,
//     pub increment: u64,
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceSource {
//     pub source: String,
//     pub code_occurence: String,
//     pub increment: u64,
// }

// impl SourceWithCodeOccurenceSource {
//     pub fn to_string(&self, symbol: &str) -> String {
//         format!("{}{}{}{}", self.source, symbol, self.code_occurence, symbol)
//     }
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceSourcesForTracing {
//     pub source: Vec<String>,
//     pub code_occurence: String,
//     pub increment: u64,
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceSourcesAndKeysForTracing {
//     pub source: SourcesAndKeysForTracing,
//     pub code_occurence: String,
//     pub increment: u64,
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceHandle {
    pub source: SourceHandleEnum,
    pub code_occurence: String,
    pub increment: u64,
}

// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceHandlePrepLog {
//     pub source: SourceHandleEnum,
//     pub prep_log: String,
//     pub increment: u64, //maybe not need
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceFinder {
    pub source: SourceFinderEnum,
    pub code_occurence: String,
    pub increment: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceFinderPrepLog {
    pub source: SourceFinderEnum,
    pub prep_log: String,
    pub increment: u64, //maybe not need
}
// //
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceFinderPrepLogSourcesForTracing {
//     pub source: Vec<String>,
//     pub prep_log: String,
//     pub increment: u64, //maybe not need
// }
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct SourceWithCodeOccurenceFinderPrepLogSourcesAndKeysForTracing {
//     pub source: SourcesAndKeysForTracing,
//     pub prep_log: String,
//     pub increment: u64, //maybe not need
// }
//     SourcesForTracing(),
//     SourcesAndKeysForTracing(),

//

//
#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurenceAsStringVersionOne {
    pub source: Option<std::collections::HashMap<String, Vec<String>>>,
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}
#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurenceAsStringVersionTwo {
    pub source: Option<String>,
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}
//

impl SourceAndCodeOccurenceAsString {
    //todo later - optimize it
    pub fn add_one(&self) -> Self {
        SourceAndCodeOccurenceAsString {
            source: self.source.clone(),
            code_occurence: self.code_occurence.clone(),
            increment: self.increment + 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SourceHandleEnum {
    //todo - rename it
    SourceWithKeys(std::collections::HashMap<String, Vec<String>>),
    Source(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SourceFinderEnum {
    SourcesForTracing(Vec<String>), //todo - add here code_occurence
    SourcesAndKeysForTracing(Vec<std::collections::HashMap<String, Vec<String>>>), //todo - add here code_occurence
}

// #[derive(Debug, Clone)]
// pub struct LogInfoStackPart {
//     pub inners:
//         Option<Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>>,
// }

// #[derive(Debug, Clone)]
// pub struct InnerSourceAndCodeOccurenceWithCounter {
//     pub inner: String,
//     pub increment: u64,
// }

// impl crate::traits::get_source::GetSource for SourceAndCodeOccurence {
//     fn get_source(&self) -> String {
//         self.source
//     }
// }

// impl crate::traits::get_code_occurence::GetCodeOccurence for SourceAndCodeOccurence {
//     fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence {
//         &self.code_occurence
//     }
// }

// impl

//     ConfigGeneric: crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetLogType
//         + crate::traits::get_color::ErrorColorBold,
