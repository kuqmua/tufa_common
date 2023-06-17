pub trait IntoActixWebHttpResponse<ConfigGeneric>
where
    ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone,
{
    fn into_actix_web_http_response(self, config: &ConfigGeneric) -> actix_web::HttpResponse;
}
