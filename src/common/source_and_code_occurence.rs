#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurence {
    pub source: String,
    pub code_occurence: crate::common::code_occurence::CodeOccurence,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceAndCodeOccurenceAsString {
    pub source: Option<SourceEnum>, //only original
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceHandle {
    pub source: SourceHandleEnum,
    pub code_occurence: String,
    pub increment: u64,
}

//
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceSourcesForTracing {
    pub sources_for_tracing: Vec<String>,
    pub code_occurence: String,
    pub increment: u64,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceKeysForTracing {
    pub source: Vec<String>,
    pub code_occurence: String,
    pub increment: u64,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceSourcesAndKeysForTracing {
    pub source: SourcesAndKeysForTracing,
    pub code_occurence: String,
    pub increment: u64,
}
//

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithCodeOccurenceFinder {
    pub source: SourceFinderEnum,
    pub code_occurence: String,
    pub increment: u64,
}

//
#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurenceAsStringVersionOne {
    pub source: Option<SourceWithKeys>,
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
    SourceWithKeys(SourceWithKeys),
    Source(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SourceFinderEnum {
    SourcesForTracing(Vec<String>),
    SourcesAndKeysForTracing(SourcesAndKeysForTracing),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SourceEnum {
    //todo - rename it
    SourceWithKeys(SourceWithKeys),
    Source(String),
    SourcesForTracing(Vec<String>),
    SourcesAndKeysForTracing(SourcesAndKeysForTracing),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourcesAndKeysForTracing {
    pub sources: Vec<String>,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceWithKeys {
    pub source: String,
    pub keys: Vec<String>,
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
