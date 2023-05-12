pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

#[derive(Debug)]
pub enum ApplicationBuildErrorEnum {
    TcpListenerBind {
        source: std::io::Error,
    },
    TcpListenerLocalAddress {
        source: std::io::Error,
    },
    ApplicationRun {
        source: Box<ApplicationRunErrorEnum>,
    },
}

impl Application {
    pub async fn build<'a>(configuration: crate::repositories_types::tufa_server::configuration::Settings<'a>) -> Result<Self, Box<ApplicationBuildErrorEnum>> {
        let connection_pool = get_connection_pool(&configuration.database);
        let listener = match std::net::TcpListener::bind(&format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        )) {
            Ok(listener) => listener,
            Err(e) => {
                return Err(Box::new(ApplicationBuildErrorEnum::TcpListenerBind {
                    source: e,
                }))
            }
        };
        let port = match listener.local_addr() {
            Ok(address) => address,
            Err(e) => {
                return Err(Box::new(
                    ApplicationBuildErrorEnum::TcpListenerLocalAddress { source: e },
                ))
            }
        }
        .port();
        let server = match run(
            listener,
            connection_pool,
            configuration.email_client.client(),
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await
        {
            Ok(server) => server,
            Err(e) => {
                return Err(Box::new(ApplicationBuildErrorEnum::ApplicationRun {
                    source: e,
                }))
            }
        };
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &crate::repositories_types::tufa_server::configuration::DatabaseSettings) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Debug)]
pub enum ApplicationRunErrorEnum {
    NewRedisSessionStore { source: std::string::String },
    HttpServerListen { source: std::io::Error },
}

async fn run(
    listener: std::net::TcpListener,
    db_pool: sqlx::PgPool,
    email_client: crate::repositories_types::tufa_server::email_client::EmailClient,
    base_url: String,
    hmac_secret: secrecy::Secret<String>,
    redis_uri: secrecy::Secret<String>,
) -> Result<actix_web::dev::Server, Box<ApplicationRunErrorEnum>> {
    let db_pool = actix_web::web::Data::new(db_pool);
    let email_client = actix_web::web::Data::new(email_client);
    let base_url = actix_web::web::Data::new(ApplicationBaseUrl(base_url));
    let secret_key = actix_web::cookie::Key::from({
        use secrecy::ExposeSecret;
        hmac_secret.expose_secret()
    }.as_bytes());
    let message_store = actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = actix_web_flash_messages::FlashMessagesFramework::builder(message_store).build();
    let redis_store = match actix_session::storage::RedisSessionStore::new({
        use secrecy::ExposeSecret;
        redis_uri.expose_secret()
    }).await {
        Ok(redis_session_store) => redis_session_store,
        Err(e) => {
            return Err(Box::new(ApplicationRunErrorEnum::NewRedisSessionStore {
                source: e.to_string(),
            }))
        }
    };
    let server = match actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(message_framework.clone())
            .wrap(actix_session::SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(
                actix_cors::Cors::default()
                    .supports_credentials()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allow_any_method()
                    .allow_any_header()
                    .expose_any_header()
                    .max_age(3600),
            ) //todo concrete host \ domain
            .route("/", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::home::home))
            .service(
                actix_web::web::scope("/admin")
                    .wrap(actix_web_lab::middleware::from_fn(crate::repositories_types::tufa_server::authentication::reject_anonymous_users))
                    .route("/dashboard", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::admin_dashboard))
                    // .route("/newsletters", web::get().to(crate::repositories_types::tufa_server::routes::publish_newsletter_form))
                    .route("/newsletters", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::publish_newsletter))
                    .route("/password", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::change_password_form))
                    .route("/password", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::change_password))
                    .route("/logout", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::log_out)),
            )
            .route("/login", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::login::login_form))
            .route("/login", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::login::login))
            .route("/health_check", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::health_check))
            .service(
                actix_web::web::scope("/api")
                .service(
                    actix_web::web::scope("/html")//or maybe .md ?
                    .route("/git_info", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::git::git_info_html::git_info_html))
                )
                .service(
                    actix_web::web::scope("/json")
                    .route("/git_info", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::git::git_info_json::git_info_json))
                    .route("/json_example", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::json_example::json_example))
                    .route("/json_example_post", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::json_example_post::json_example_post))
                )
            )
            .route("/subscriptions", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::subscribe))
            .route("/subscriptions/confirm", actix_web::web::get().to(crate::repositories_types::tufa_server::routes::confirm))
            .route("/newsletters", actix_web::web::post().to(crate::repositories_types::tufa_server::routes::publish_newsletter))
            .route(
                "/get_providers_posts",
                actix_web::web::post().to(crate::repositories_types::tufa_server::routes::get_providers_posts_route::get_providers_posts_route),
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(actix_web::web::Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)
    {
        Ok(server) => server,
        Err(e) => {
            return Err(Box::new(ApplicationRunErrorEnum::HttpServerListen {
                source: e,
            }))
        }
    }
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub secrecy::Secret<String>);
