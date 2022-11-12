use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::server::http_request::http_request_error::HttpRequestError;
use crate::server::http_request::http_request_method::HttpRequestMethod;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[allow(clippy::too_many_arguments)]
pub async fn sync_http_request_client_request_builder_prep<
    //client generics
    UserAgentValueGeneric,
    CookieProviderGeneric,
    PoolIdleTimeoutDurationGeneric,
    Http2InitialStreamWindowSizeGeneric,
    Http2InitialConnectionWindowSizeGeneric,
    Http2MaxFrameSizeGeneric,
    LocalAddressGeneric,
    TcpKeepaliveGeneric,
    UsePreconfiguredTlsGeneric,
    //request builder generics
    HeaderKeyGeneric,
    HeaderValueGeneric,
    BasicAuthUsernameGeneric,
    BasicAuthPasswordGeneric,
    BearerAuthGeneric,
    BodyGeneric,
    QueryGeneric,
    FormGeneric,
    JsonGeneric,
>(
    url: &str,
    //client parameters
    user_agent_client_argument: Option<UserAgentValueGeneric>,
    default_headers_client_argument: Option<reqwest::header::HeaderMap>,
    cookie_store_client_argument: Option<bool>,
    cookie_provider_client_argument: Option<std::sync::Arc<CookieProviderGeneric>>,
    gzip_client_argument: Option<bool>,
    brotli_client_argument: Option<bool>,
    deflate_client_argument: Option<bool>,
    no_gzip_client_argument: Option<()>,
    no_brotli_client_argument: Option<()>,
    no_deflate_client_argument: Option<()>,
    redirect_client_argument: Option<reqwest::redirect::Policy>,
    referer_client_argument: Option<bool>,
    proxy_client_argument: Option<reqwest::Proxy>,
    no_proxy_client_argument: Option<()>,
    timeout_client_argument: Option<std::time::Duration>,
    connect_timeout_client_argument: Option<std::time::Duration>,
    connection_verbose_client_argument: Option<bool>,
    pool_idle_timeout_client_argument: Option<PoolIdleTimeoutDurationGeneric>,
    pool_max_idle_per_host_client_argument: Option<usize>,
    http1_title_case_headers_client_argument: Option<()>,
    http1_allow_obsolete_multiline_headers_in_responses_client_argument: Option<bool>,
    http1_only_client_argument: Option<()>,
    http09_responses_client_argument: Option<()>,
    http2_prior_knowledge_client_argument: Option<()>,
    http2_initial_stream_window_size_client_argument: Option<Http2InitialStreamWindowSizeGeneric>, //impl Into<Option<u32>>
    http2_initial_connection_window_size_client_argument: Option<
        Http2InitialConnectionWindowSizeGeneric,
    >,
    http2_adaptive_window_client_argument: Option<bool>,
    http2_max_frame_size_client_argument: Option<Http2MaxFrameSizeGeneric>,
    tcp_nodelay_client_argument: Option<bool>,
    local_address_client_argument: Option<LocalAddressGeneric>,
    tcp_keepalive_client_argument: Option<TcpKeepaliveGeneric>,
    add_root_certificate_client_argument: Option<reqwest::Certificate>,
    tls_built_in_root_certs_client_argument: Option<bool>,
    identity_client_argument: Option<reqwest::Identity>,
    danger_accept_invalid_hostnames_client_argument: Option<bool>,
    danger_accept_invalid_certs_client_argument: Option<bool>,
    min_tls_version_client_argument: Option<reqwest::tls::Version>,
    max_tls_version_client_argument: Option<reqwest::tls::Version>,
    use_native_tls_client_argument: Option<()>,
    use_rustls_tls_client_argument: Option<()>,
    use_preconfigured_tls_client_argument: Option<UsePreconfiguredTlsGeneric>,
    trust_dns_client_argument: Option<bool>,
    no_trust_dns_client_argument: Option<()>,
    https_only_client_argument: Option<bool>,
    resolve_client_argument: Option<(&str, std::net::SocketAddr)>,
    resolve_to_addrs_client_argument: Option<(&str, &[std::net::SocketAddr])>,
    //request builder parameters
    header_request_builder: Option<(HeaderKeyGeneric, HeaderValueGeneric)>,
    headers_request_builder: Option<reqwest::header::HeaderMap<reqwest::header::HeaderValue>>,
    basic_auth_request_builder: Option<(
        BasicAuthUsernameGeneric,
        Option<BasicAuthPasswordGeneric>,
    )>,
    bearer_auth_request_builder: Option<BearerAuthGeneric>,
    body_request_builder: Option<BodyGeneric>,
    timeout_request_builder: Option<std::time::Duration>,
    multipart_request_builder: Option<reqwest::blocking::multipart::Form>,
    query_request_builder: Option<QueryGeneric>,
    version_request_builder: Option<reqwest::Version>,
    form_request_builder: Option<FormGeneric>,
    json_request_builder: Option<JsonGeneric>,
    //
    method: HttpRequestMethod,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<reqwest::blocking::RequestBuilder, Box<HttpRequestError>>
where
    UserAgentValueGeneric: TryInto<reqwest::header::HeaderValue>,
    UserAgentValueGeneric: TryInto<reqwest::header::HeaderValue>,
    UserAgentValueGeneric::Error: Into<http::Error>,
    CookieProviderGeneric: reqwest::cookie::CookieStore + 'static,
    PoolIdleTimeoutDurationGeneric: Into<Option<std::time::Duration>>,
    Http2InitialStreamWindowSizeGeneric: Into<Option<u32>>,
    Http2InitialConnectionWindowSizeGeneric: Into<Option<u32>>,
    Http2MaxFrameSizeGeneric: Into<Option<u32>>,
    LocalAddressGeneric: Into<Option<std::net::IpAddr>>,
    TcpKeepaliveGeneric: Into<Option<std::time::Duration>>,
    UsePreconfiguredTlsGeneric: std::any::Any,
    reqwest::header::HeaderName: TryFrom<HeaderKeyGeneric>,
    <reqwest::header::HeaderName as TryFrom<HeaderKeyGeneric>>::Error: Into<http::Error>,
    reqwest::header::HeaderValue: TryFrom<HeaderValueGeneric>,
    <reqwest::header::HeaderValue as TryFrom<HeaderValueGeneric>>::Error: Into<http::Error>,
    BasicAuthUsernameGeneric: std::fmt::Display,
    BasicAuthPasswordGeneric: std::fmt::Display,
    BearerAuthGeneric: std::fmt::Display,
    BodyGeneric: Into<reqwest::blocking::Body>,
    QueryGeneric: serde::Serialize,
    FormGeneric: serde::Serialize,
    JsonGeneric: serde::Serialize,
{
    let mut client_builder = reqwest::blocking::Client::builder();
    if let Some(v) = user_agent_client_argument {
        client_builder = client_builder.user_agent(v);
    }
    if let Some(v) = default_headers_client_argument {
        client_builder = client_builder.default_headers(v);
    }
    if let Some(v) = cookie_store_client_argument {
        client_builder = client_builder.cookie_store(v);
    }
    if let Some(v) = cookie_provider_client_argument {
        client_builder = client_builder.cookie_provider(v);
    }
    if let Some(v) = gzip_client_argument {
        client_builder = client_builder.gzip(v);
    }
    if let Some(v) = brotli_client_argument {
        client_builder = client_builder.brotli(v);
    }
    if let Some(v) = deflate_client_argument {
        client_builder = client_builder.deflate(v);
    }
    if no_gzip_client_argument.is_some() {
        client_builder = client_builder.no_gzip();
    }
    if no_brotli_client_argument.is_some() {
        client_builder = client_builder.no_brotli();
    }
    if no_deflate_client_argument.is_some() {
        client_builder = client_builder.no_deflate();
    }
    if let Some(v) = redirect_client_argument {
        client_builder = client_builder.redirect(v);
    }
    if let Some(v) = referer_client_argument {
        client_builder = client_builder.referer(v);
    }
    if let Some(v) = proxy_client_argument {
        client_builder = client_builder.proxy(v);
    }
    if no_proxy_client_argument.is_some() {
        client_builder = client_builder.no_proxy();
    }
    if let Some(v) = timeout_client_argument {
        client_builder = client_builder.timeout(v);
    }
    if let Some(v) = connect_timeout_client_argument {
        client_builder = client_builder.connect_timeout(v);
    }
    if let Some(v) = connection_verbose_client_argument {
        client_builder = client_builder.connection_verbose(v);
    }
    if let Some(v) = pool_idle_timeout_client_argument {
        client_builder = client_builder.pool_idle_timeout(v);
    }
    if let Some(v) = pool_max_idle_per_host_client_argument {
        client_builder = client_builder.pool_max_idle_per_host(v);
    }
    if http1_title_case_headers_client_argument.is_some() {
        client_builder = client_builder.http1_title_case_headers();
    }
    if let Some(v) = http1_allow_obsolete_multiline_headers_in_responses_client_argument {
        client_builder = client_builder.http1_allow_obsolete_multiline_headers_in_responses(v);
    }
    if http1_only_client_argument.is_some() {
        client_builder = client_builder.http1_only();
    }
    if http09_responses_client_argument.is_some() {
        client_builder = client_builder.http09_responses();
    }
    if http2_prior_knowledge_client_argument.is_some() {
        client_builder = client_builder.http2_prior_knowledge();
    }
    if let Some(v) = http2_initial_stream_window_size_client_argument {
        client_builder = client_builder.http2_initial_stream_window_size(v);
    }
    if let Some(v) = http2_initial_connection_window_size_client_argument {
        client_builder = client_builder.http2_initial_connection_window_size(v);
    }
    if let Some(v) = http2_adaptive_window_client_argument {
        client_builder = client_builder.http2_adaptive_window(v);
    }
    if let Some(v) = http2_max_frame_size_client_argument {
        client_builder = client_builder.http2_max_frame_size(v);
    }
    // not implemented for blocking
    // if let Some(v) = http2_keep_alive_interval_client_argument {
    //     client_builder = client_builder.http2_keep_alive_interval(v);
    // }
    // not implemented for blocking
    // if let Some(v) = http2_keep_alive_timeout_client_argument {
    //     client_builder = client_builder.http2_keep_alive_timeout(v);
    // }
    // not implemented for blocking
    // if let Some(v) = http2_keep_alive_while_idle_client_argument {
    //     client_builder = client_builder.http2_keep_alive_while_idle(v);
    // }
    if let Some(v) = tcp_nodelay_client_argument {
        client_builder = client_builder.tcp_nodelay(v);
    }
    if let Some(v) = local_address_client_argument {
        client_builder = client_builder.local_address(v);
    }
    if let Some(v) = tcp_keepalive_client_argument {
        client_builder = client_builder.tcp_keepalive(v);
    }
    if let Some(v) = add_root_certificate_client_argument {
        client_builder = client_builder.add_root_certificate(v);
    }
    if let Some(v) = tls_built_in_root_certs_client_argument {
        client_builder = client_builder.tls_built_in_root_certs(v);
    }
    if let Some(v) = identity_client_argument {
        client_builder = client_builder.identity(v);
    }
    if let Some(v) = danger_accept_invalid_hostnames_client_argument {
        client_builder = client_builder.danger_accept_invalid_hostnames(v);
    }
    if let Some(v) = danger_accept_invalid_certs_client_argument {
        client_builder = client_builder.danger_accept_invalid_certs(v);
    }
    if let Some(v) = min_tls_version_client_argument {
        client_builder = client_builder.min_tls_version(v);
    }
    if let Some(v) = max_tls_version_client_argument {
        client_builder = client_builder.max_tls_version(v);
    }
    if use_native_tls_client_argument.is_some() {
        client_builder = client_builder.use_native_tls();
    }
    if use_rustls_tls_client_argument.is_some() {
        client_builder = client_builder.use_rustls_tls();
    }
    if let Some(v) = use_preconfigured_tls_client_argument {
        client_builder = client_builder.use_preconfigured_tls(v);
    }
    if let Some(v) = trust_dns_client_argument {
        client_builder = client_builder.trust_dns(v);
    }
    if no_trust_dns_client_argument.is_some() {
        client_builder = client_builder.no_trust_dns();
    }
    if let Some(v) = https_only_client_argument {
        client_builder = client_builder.https_only(v);
    }
    if let Some(v) = resolve_client_argument {
        client_builder = client_builder.resolve(v.0, v.1);
    }
    if let Some(v) = resolve_to_addrs_client_argument {
        client_builder = client_builder.resolve_to_addrs(v.0, v.1);
    }
    match client_builder.build() {
        Err(e) => Err(Box::new(HttpRequestError::init_error_with_possible_trace(
            e,
            WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
            source_place_type,
            &GIT_INFO,
            should_trace,
        ))),
        Ok(client_handle) => {
            let mut request_builder_handle = method.get_sync_request_builder(client_handle, url); //do something with get
            if let Some(v) = header_request_builder {
                request_builder_handle = request_builder_handle.header(v.0, v.1);
            }
            if let Some(v) = headers_request_builder {
                request_builder_handle = request_builder_handle.headers(v);
            }
            if let Some(v) = basic_auth_request_builder {
                request_builder_handle = request_builder_handle.basic_auth(v.0, v.1);
            }
            if let Some(v) = bearer_auth_request_builder {
                request_builder_handle = request_builder_handle.bearer_auth(v);
            }
            if let Some(v) = body_request_builder {
                request_builder_handle = request_builder_handle.body(v);
            }
            if let Some(v) = timeout_request_builder {
                request_builder_handle = request_builder_handle.timeout(v);
            }
            if let Some(v) = multipart_request_builder {
                request_builder_handle = request_builder_handle.multipart(v);
            }
            if let Some(v) = query_request_builder {
                request_builder_handle = request_builder_handle.query(&v);
            }
            if let Some(v) = version_request_builder {
                request_builder_handle = request_builder_handle.version(v);
            }
            if let Some(v) = form_request_builder {
                request_builder_handle = request_builder_handle.form(&v);
            }
            if let Some(v) = json_request_builder {
                request_builder_handle = request_builder_handle.json(&v);
            }
            // not implemented for blocking
            // if fetch_mode_no_cors_request_builder.is_some() {
            //     request_builder_handle = request_builder_handle.fetch_mode_no_cors();
            // }
            Ok(request_builder_handle)
        }
    }
}
