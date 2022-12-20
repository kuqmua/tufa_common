#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurence {
    pub source: String,
    pub code_occurence: crate::common::code_occurence::CodeOccurence,
}

#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurenceAsString {
    pub source: Option<SourceEnum>, //only original
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}

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

#[derive(Debug, Clone)]
pub enum SourceEnum {
    SourceWithKeys(SourceWithKeys),
    Source(String),
}

#[derive(Debug, Clone)]
pub struct SourceWithKeys {
    pub keys: Vec<String>,
    pub source: String,
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
