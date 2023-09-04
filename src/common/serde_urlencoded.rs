#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SerdeUrlencodedParametersErrorNamed {
    UrlEncode {
        #[eo_display_with_serialize_deserialize]
        field: std::string::String,
        #[eo_error_occurence]
        url_encode: SerdeUrlencodedParameterErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub trait SerdeUrlencodedParameters {
    fn serde_urlencoded_parameters(
        &self,
    ) -> Result<std::string::String, SerdeUrlencodedParametersErrorNamed>;
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum SerdeUrlencodedParameterErrorNamed {
    UrlEncode {
        #[eo_display]
        url_encode: serde_urlencoded::ser::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub trait SerdeUrlencodedParameter {
    fn serde_urlencoded_parameter(
        &self,
    ) -> Result<std::string::String, SerdeUrlencodedParameterErrorNamed>;
}
