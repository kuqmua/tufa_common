#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommitExtractor {}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    // type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
//     crate::repositories_types::tufa_server::routes::api::cats::TryReadMany,
//     crate::repositories_types::tufa_server::routes::api::cats::TryReadOne,
//     crate::repositories_types::tufa_server::routes::api::cats::TryCreateOne,
//     crate::repositories_types::tufa_server::routes::api::cats::TryCreateMany,
//     crate::repositories_types::tufa_server::routes::api::cats::TryReadManyWithBody
// )]
pub enum ProjectCommitExtractorCheckErrorNamed {
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

//todo make a proc macro for it or maybe put it into error occurence?
impl crate::common::to_default_stringified_json::ToDefaultStringifiedJson
    for ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
{
    fn to_default_stringified_json(&self) -> std::string::String {
        match self {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::ProjectCommitExtractorNotEqual { project_commit_not_equal: _, project_commit_to_use: _, code_occurence: _ } => std::string::String::from(
                "{\"ProjectCommitExtractorNotEqual\":{\"project_commit_not_equal\":\"\",\"project_commit_to_use\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::ProjectCommitExtractorToStrConversion { project_commit_to_str_conversion: _, code_occurence: _ } => std::string::String::from(
                "{\"ProjectCommitExtractorToStrConversion\":{\"project_commit_to_str_conversion\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::NoProjectCommitExtractorHeader { no_project_commit_header: _, code_occurence: _ } => std::string::String::from(
                "{\"NoProjectCommitExtractorHeader\":{\"no_project_commit_header\":\"no_project_commit_header\",\"code_occurence\":{\"file\":\"\",\"line\":124,\"column\":53,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
        }
    }
}
//////////
impl std::convert::From<ProjectCommitExtractorCheckErrorNamed>
    for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariants
{
    fn from(val: ProjectCommitExtractorCheckErrorNamed) -> Self {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamed>
    for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariants
{
    fn from(val: ProjectCommitExtractorCheckErrorNamed) -> Self {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamed>
    for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariants
{
    fn from(val: ProjectCommitExtractorCheckErrorNamed) -> Self {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamed>
    for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariants
{
    fn from(val: ProjectCommitExtractorCheckErrorNamed) -> Self {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryReadManyWithBodyResponseVariants
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryReadManyWithSerializeDeserialize
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryReadOneWithSerializeDeserialize
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryCreateOneWithSerializeDeserialize
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryCreateManyWithSerializeDeserialize
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl std :: convert :: From < ProjectCommitExtractorCheckErrorNamed > for
crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
TryReadManyWithBodyWithSerializeDeserialize
{
    fn from(val : ProjectCommitExtractorCheckErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            } => Self :: ProjectCommitExtractorNotEqual
            {
                project_commit_not_equal, project_commit_to_use,
                code_occurence
            }, ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
            :: ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence } => Self ::
            ProjectCommitExtractorToStrConversion
            { project_commit_to_str_conversion, code_occurence },
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence } => Self ::
            NoProjectCommitExtractorHeader
            { no_project_commit_header, code_occurence }
        }
    }
}
impl From<&ProjectCommitExtractorCheckErrorNamed> for http::StatusCode {
    fn from(val: &ProjectCommitExtractorCheckErrorNamed) -> Self {
        match &val {
            ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {
                project_commit_not_equal: _,
                project_commit_to_use: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion {
                project_commit_to_str_conversion: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader {
                no_project_commit_header: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
        }
    }
}
#[allow(clippy::enum_variant_names, dead_code)]
enum ProjectCommitExtractorCheckErrorNamedStatusCodesChecker {
    ProjectCommitExtractorNotEqualTvfrr400BadRequest,
    ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
    NoProjectCommitExtractorHeaderTvfrr400BadRequest,
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamedStatusCodesChecker>
    for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyStatusCodesChecker
{
    fn from(val: ProjectCommitExtractorCheckErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest => crate
            :: repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamedStatusCodesChecker>
    for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneStatusCodesChecker
{
    fn from(val: ProjectCommitExtractorCheckErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadOneStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest => crate
            :: repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadOneStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadOneStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamedStatusCodesChecker>
    for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneStatusCodesChecker
{
    fn from(val: ProjectCommitExtractorCheckErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateOneStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest => crate
            :: repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateOneStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateOneStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest
        }
    }
}
impl std::convert::From<ProjectCommitExtractorCheckErrorNamedStatusCodesChecker>
    for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyStatusCodesChecker
{
    fn from(val: ProjectCommitExtractorCheckErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateManyStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest => crate
            :: repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateManyStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryCreateManyStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest
        }
    }
}
impl std :: convert :: From <
ProjectCommitExtractorCheckErrorNamedStatusCodesChecker > for crate ::
repositories_types :: tufa_server :: routes :: api :: cats ::
TryReadManyWithBodyStatusCodesChecker
{
    fn from(val : ProjectCommitExtractorCheckErrorNamedStatusCodesChecker,) ->
    Self
    {
        match val
        {
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyWithBodyStatusCodesChecker ::
            ProjectCommitExtractorNotEqualTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest => crate
            :: repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyWithBodyStatusCodesChecker ::
            ProjectCommitExtractorToStrConversionTvfrr400BadRequest,
            ProjectCommitExtractorCheckErrorNamedStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest => crate ::
            repositories_types :: tufa_server :: routes :: api :: cats ::
            TryReadManyWithBodyStatusCodesChecker ::
            NoProjectCommitExtractorHeaderTvfrr400BadRequest
        }
    }
}
