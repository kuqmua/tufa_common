#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CheckAvailabilityErrorNamed<'a> {
    Net {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Postgres {
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<
            'a,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // Mongo {
    //     #[eo_error_occurence]
    //     mongo: crate::server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    NetAndPostgres {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    // NetAndMongo {
    //     #[eo_error_occurence]
    //     net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
    //     #[eo_error_occurence]
    //     mongo: crate::server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    // PostgresAndMongo {
    //     #[eo_error_occurence]
    //     postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
    //     #[eo_error_occurence]
    //     mongo: crate::server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
    // NetAndPostgresAndMongo {
    //     #[eo_error_occurence]
    //     net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
    //     #[eo_error_occurence]
    //     postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
    //     #[eo_error_occurence]
    //     mongo: crate::server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
    //     code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    // },
}

pub async fn check_availability<'a, SelfGeneric>(
    config: &'static (
        impl crate::traits::config_fields::GetStartingCheckLink
        + crate::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + crate::traits::config_fields::GetPostgresConnectionTimeout
        // + crate::traits::config_fields::GetMongoClient
        + std::marker::Send 
        + std::marker::Sync
    )
) -> Result<(), Box<crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed<'a>>>{
    match futures::join!(
        crate::server::net::net_check_availability::net_check_availability(config.get_starting_check_link()),
        crate::server::postgres::postgres_check_availability::postgres_check_availability(
            config.get_postgres_url(), 
            *config.get_postgres_connection_timeout()
        )
    ) {
        (Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Err(p)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Postgres {
            postgres: *p,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        (Err(n), Ok(_)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Net {
            net: *n,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        (Err(n), Err(p)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndPostgres {
            net: *n,
            postgres: *p,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
    }
    // match futures::join!(
    //     crate::server::net::net_check_availability::net_check_availability(config.get_starting_check_link()),
    //     crate::server::postgres::postgres_check_availability::postgres_check_availability(
    //         config.get_postgres_url(), 
    //         *config.get_postgres_connection_timeout()
    //     ),
    //     crate::server::mongo_integration::mongo_check_availability::mongo_check_availability(
    //         config,
    //         "logs"
    //     ),
    // ) {
    //     (Ok(_), Ok(_), Ok(_)) => Ok(()),
    //     (Ok(_), Ok(_), Err(m)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Mongo {
    //         mongo: *m,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Ok(_), Err(p), Ok(_)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Postgres {
    //         postgres: *p,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Ok(_), Err(p), Err(m)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::PostgresAndMongo {
    //         postgres: *p,
    //         mongo: *m,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Err(n), Ok(_), Ok(_)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Net {
    //         net: *n,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Err(n), Ok(_), Err(m)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndMongo {
    //         net: *n,
    //         mongo: *m,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Err(n), Err(p), Ok(_)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndPostgres {
    //         net: *n,
    //         postgres: *p,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    //     (Err(n), Err(p), Err(m)) => Err(Box::new(crate::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndPostgresAndMongo {
    //         net: *n,
    //         postgres: *p,
    //         mongo: *m,
    //         code_occurence: crate::code_occurence_tufa_common!(),
    //     })),
    // }
}