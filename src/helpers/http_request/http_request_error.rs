use crate::lazy_static::config::CONFIG;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplGetSourceWithoutMethodFromTufaCommon,
)]
pub struct HttpRequestClientRequestBuilderPrepError {
    source: reqwest::Error,
    where_was: WhereWas,
}

//
// #[derive(
//     Debug,
//     ImplGetSourceWithMethodFromTufaCommon,
//     ImplDisplayForSimpleErrorEnum,
//     ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
// )]
// pub enum HttpRequestGetErrorEnum {
//     Bytes(BytesError),
//     ContentLength(ContentLengthError),
//     Json(JsonError),
//     RemoteAddr(RemoteAddrError),
//     Text(TextError),
//     TextWithCharset(TextWithCharsetError),
//     Version(VersionError),
// }

//
// match async_text(
//     // https://docs.rs/reqwest/0.11.12/reqwest/struct.RequestBuilder.html
//     request_builder_handle,
//     false,
// )
// .await
// {
//     Err(e) => Err(Box::new(HttpRequestClientRequestBuilderPrepError::init_error_with_possible_trace(
//         HttpRequestClientRequestBuilderPrepErrorEnum::RequestBuilder(HttpRequestGetErrorEnum::Text(*e)),
//         WhereWas {
//             time: std::time::SystemTime::now()
//                 .duration_since(std::time::UNIX_EPOCH)
//                 .expect("cannot convert time to unix_epoch"),
//             location: *core::panic::Location::caller(),
//         },
//         &CONFIG.source_place_type,
//         &GIT_INFO.data,
//         should_trace,
//     ))),
//     Ok(string_content) => Ok(string_content),
// }
//

// user_agent_value: UserAgentValueGeneric,
// default_headers_value: reqwest::header::HeaderMap,
// cookie_store_value: bool,
// cookie_provider_value: std::sync::Arc<CookieProviderGeneric>,
// gzip_value: bool,
// brotli_value: bool,
// deflate_value: bool,
// redirect_value: reqwest::redirect::Policy,
// referer_value: bool,
// proxy_value: reqwest::Proxy,
// timeout_value: std::time::Duration,
// connect_timeout_value: std::time::Duration,
// connection_verbose_value: bool,
// pool_idle_timeout_value: PoolIdleTimeoutDurationGeneric,
// pool_max_idle_per_host_value: usize,
// http1_allow_obsolete_multiline_headers_in_responses_value: bool,
// http2_initial_stream_window_size_value: impl Into<Option<u32>>,
// http2_initial_connection_window_size_value: impl Into<Option<u32>>,
// http2_adaptive_window_value: bool,
// http2_max_frame_size_value: impl Into<Option<u32>>,
// http2_keep_alive_interval_value: impl Into<Option<std::time::Duration>>,
// http2_keep_alive_timeout_value: std::time::Duration,
// http2_keep_alive_while_idle_value: bool,
// tcp_nodelay_value: bool,
// local_address_value: LocalAddressGeneric,
// tcp_keepalive_value: TcpKeepaliveGeneric,
// add_root_certificate_value: reqwest::Certificate,
// tls_built_in_root_certs_value: bool,
// identity_value: reqwest::Identity,
// danger_accept_invalid_hostnames_value: bool,
// danger_accept_invalid_certs_value: bool,
// min_tls_version_value: reqwest::tls::Version,
// max_tls_version_value: reqwest::tls::Version,
// use_preconfigured_tls_value: impl std::any::Any,
// trust_dns_value: bool,
// https_only_value: bool,
// resolve_value: (&str, std::net::SocketAddr),
// resolve_to_addrs_value: (&str, &[std::net::SocketAddr]),

//https://docs.rs/reqwest/0.11.12/reqwest/struct.ClientBuilder.html
// reqwest::Client::builder()
//     .user_agent(user_agent_value)
//     .default_headers(default_headers_value)
//     .cookie_store(cookie_store_value)
//     .cookie_provider(cookie_provider_value)
//     .gzip(gzip_value)
//     .brotli(brotli_value)
//     .deflate(deflate_value)
//     .no_gzip()
//     .no_brotli()
//     .no_deflate()
//     .redirect(redirect_value)
//     .referer(referer_value)
//     .proxy(proxy_value)
//     .no_proxy()
//     .timeout(timeout_value)
//     .connect_timeout(connect_timeout_value)
//     .connection_verbose(connection_verbose_value)
//     .pool_idle_timeout(pool_idle_timeout_value)
//     .pool_max_idle_per_host(pool_max_idle_per_host_value)
//     .http1_title_case_headers()
//     .http1_allow_obsolete_multiline_headers_in_responses(
//         http1_allow_obsolete_multiline_headers_in_responses_value,
//     )
//     .http1_only()
//     .http09_responses()
//     .http2_prior_knowledge()
//     .http2_initial_stream_window_size(http2_initial_stream_window_size_value)
//     .http2_initial_connection_window_size(http2_initial_connection_window_size_value)
//     .http2_adaptive_window(http2_adaptive_window_value)
//     .http2_max_frame_size(http2_max_frame_size_value)
//     .http2_keep_alive_interval(http2_keep_alive_interval_value)
//     .http2_keep_alive_timeout(http2_keep_alive_timeout_value)
//     .http2_keep_alive_while_idle(http2_keep_alive_while_idle_value)
//     .tcp_nodelay(tcp_nodelay_value)
//     .local_address(local_address_value)
//     .tcp_keepalive(tcp_keepalive_value)
//     .add_root_certificate(add_root_certificate_value)
//     .tls_built_in_root_certs(tls_built_in_root_certs_value)
//     .identity(identity_value)
//     .danger_accept_invalid_hostnames(danger_accept_invalid_hostnames_value)
//     .danger_accept_invalid_certs(danger_accept_invalid_certs_value)
//     .min_tls_version(min_tls_version_value)
//     .max_tls_version(max_tls_version_value)
//     .use_native_tls()
//     .use_rustls_tls()
//     .use_preconfigured_tls(use_preconfigured_tls_value)
//     .trust_dns(trust_dns_value)
//     .no_trust_dns()
//     .https_only(https_only_value)
//     .resolve(resolve_value.0, resolve_value.1)
//     .resolve_to_addrs(resolve_to_addrs_value.0, resolve_to_addrs_value.1),

// pub async fn async_http_request(
//     should_trace: bool,
// ) -> Result<reqwest::Client, Box<HttpRequestClientRequestBuilderPrepError>> {
//     match async_client_builder(reqwest::Client::builder(), false).await {
//         Err(e) => Err(Box::new(HttpRequestClientRequestBuilderPrepError::init_error_with_possible_trace(
//             HttpRequestClientRequestBuilderPrepErrorEnum::ClientBuilder(*e),
//             WhereWas {
//                 time: std::time::SystemTime::now()
//                     .duration_since(std::time::UNIX_EPOCH)
//                     .expect("cannot convert time to unix_epoch"),
//                 location: *core::panic::Location::caller(),
//             },
//             &CONFIG.source_place_type,
//             &GIT_INFO.data,
//             should_trace,
//         ))),
//         Ok(request_builder) => Ok(request_builder),
//     }
// }

// pub async fn sync_http_request(
//     should_trace: bool,
// ) -> Result<reqwest::blocking::Client, Box<HttpRequestClientRequestBuilderPrepError>> {
//     match sync_client_builder(reqwest::blocking::Client::builder(), false) {
//         Err(e) => Err(Box::new(HttpRequestClientRequestBuilderPrepError::init_error_with_possible_trace(
//             HttpRequestClientRequestBuilderPrepErrorEnum::ClientBuilder(*e),
//             WhereWas {
//                 time: std::time::SystemTime::now()
//                     .duration_since(std::time::UNIX_EPOCH)
//                     .expect("cannot convert time to unix_epoch"),
//                 location: *core::panic::Location::caller(),
//             },
//             &CONFIG.source_place_type,
//             &GIT_INFO.data,
//             should_trace,
//         ))),
//         Ok(request_builder) => Ok(request_builder),
//     }
// }
