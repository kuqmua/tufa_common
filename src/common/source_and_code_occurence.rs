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

impl SourceAndCodeOccurenceAsString {
    pub fn add_one(&mut self) {
        self.increment = self.increment + 1;
    }
}
