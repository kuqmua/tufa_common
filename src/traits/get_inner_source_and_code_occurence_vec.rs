use itertools::Itertools;

pub trait GetInnerSourceAndCodeOccurenceVec<ConfigGeneric> {
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>;
}

pub trait GetInnerSourceAndCodeOccurenceVecHelper<ConfigGeneric> {
    fn get_inner_source_and_code_occurence_vec_helper(
        &self,
        config: &ConfigGeneric,
    ) -> (
        Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        >,
        Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>,
    );
}

impl<ConfigGeneric, HashMapKeyGeneric, HashMapValueGeneric>
    GetInnerSourceAndCodeOccurenceVecHelper<ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetTimezone,
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: GetInnerSourceAndCodeOccurenceVec<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString<ConfigGeneric>,
{
    fn get_inner_source_and_code_occurence_vec_helper(
        &self,
        config: &ConfigGeneric,
    ) -> (
        Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        >,
        Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>,
    ) {
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = vec![];
        let vec = self.iter().fold(
            Vec::with_capacity(self.len() + 1),
            |mut acc, (key, value)| {
                value
                    .get_inner_source_and_code_occurence_vec(config)
                    .into_iter()
                    .for_each(|mut e| {
                        e.source.iter().for_each(|hm| {
                            let mut new_hm = Vec::with_capacity(hm.len());
                            hm.iter().for_each(|(k, v)| {
                                let mut new_v = Vec::with_capacity(v.len() + 1);
                                v.iter().for_each(|v_element| {
                                    new_v.push(v_element.clone());
                                });
                                new_v.push(crate::common::source_and_code_occurence::Key {
                                    key: key.to_string(),
                                    uuid: uuid::Uuid::new_v4(),
                                });
                                new_hm.push((k.clone(), new_v.clone()));
                            });
                            sources_for_tracing.push(new_hm);
                        });
                        e.add_one();
                        acc.push(e);
                    });
                acc
            },
        );
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        (sources_for_tracing, vec)
    }
}

impl<ConfigGeneric, VecElementGeneric> GetInnerSourceAndCodeOccurenceVecHelper<ConfigGeneric>
    for Vec<VecElementGeneric>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType + crate::traits::fields::GetTimezone,
    VecElementGeneric: GetInnerSourceAndCodeOccurenceVec<ConfigGeneric>
        + crate::traits::get_code_occurence::GetCodeOccurenceAsString<ConfigGeneric>,
{
    fn get_inner_source_and_code_occurence_vec_helper(
        &self,
        config: &ConfigGeneric,
    ) -> (
        Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        >,
        Vec<crate::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>,
    ) {
        let mut sources_for_tracing: Vec<
            Vec<(
                crate::common::source_and_code_occurence::Source,
                Vec<crate::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::new();
        let vec = self.iter().fold(
            Vec::with_capacity(self.len() + 1),
            |mut acc, vec_element| {
                vec_element
                    .get_inner_source_and_code_occurence_vec(config)
                    .into_iter()
                    .for_each(|mut e| {
                        e.source.iter().for_each(|f| {
                            sources_for_tracing.push(f.clone());
                        });
                        e.add_one();
                        acc.push(e);
                    });
                acc
            },
        );
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        (sources_for_tracing, vec)
    }
}
