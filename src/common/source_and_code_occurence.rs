#[derive(Debug, Clone)]
pub struct SourceAndCodeOccurence {
    pub source: String,
    pub code_occurence: crate::common::code_occurence::CodeOccurence,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SourceAndCodeOccurenceAsString {
    //add for source and key some unique id generated by time
    pub source: Vec<Vec<(Source, Vec<Key>)>>,
    // pub source: SourceType, //only original
    pub code_occurence: String,
    pub increment: u64, //i think its incorrect
                        // maybe add another field like paralel index?
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Source {
    pub source: String,
    pub uuid: uuid::Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Key {
    pub key: String,
    pub uuid: uuid::Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SourceType {
    Origin(String),
    Wrapper(
        Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        >,
    ),
}

impl SourceAndCodeOccurenceAsString {
    pub fn add_one(&mut self) {
        self.increment = self.increment + 1;
    }
}
