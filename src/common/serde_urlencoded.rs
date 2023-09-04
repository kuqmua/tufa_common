#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SerdeUrlencodedParametersErrorNamed {
    UrlEncode {
        #[eo_display_with_serialize_deserialize]
        field: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub trait SerdeUrlencodedParameters {
    fn serde_urlencoded_parameters(
        &self,
    ) -> Result<std::string::String, SerdeUrlencodedParametersErrorNamed>;
}

pub trait SerdeUrlencodedParameter {
    fn serde_urlencoded_parameter(&self) -> std::string::String;
}
