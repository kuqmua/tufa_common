pub trait FromFewCodeOccurencesHashMap<KeyGeneric, ValueGeneric> {
    fn from_few_code_occurences_hashmap(&self) -> CodeOccurence;
}

impl<KeyGeneric, ValueGeneric> FromFewCodeOccurencesHashMap<KeyGeneric, ValueGeneric>
    for HashMap<KeyGeneric, ValueGeneric>
where
    ValueGeneric: crate::traits::get_code_occurence::GetCodeOccurence,
{
    fn from_few_code_occurences_hashmap(&self) -> CodeOccurence {
        let mut parallel_counter = 0; //todo - add some field to code occurence?
        let mut formatted = self
            // .iter()
            .values()
            // .map(|(code_occurence)| code_occurence)
            // .collect::<Vec<String>>()
            // .iter()
            .fold(
                HashMap::<GitInformationWithoutLifetimes, Vec<IncrementPidTimeFileLineColumn>>::new(
                ),
                |mut acc, elem| {
                    let current_code_occurence = elem.get_code_occurence();
                    // acc.push_str(elem);
                    acc
                },
            );
        // if !formatted.is_empty() {
        //     formatted.pop();
        // }
        // formatted
        CodeOccurence {
            occurences: HashMap::new(),
        }
    }
}
