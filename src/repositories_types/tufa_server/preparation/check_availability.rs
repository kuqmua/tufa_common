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
    Mongo {
        #[eo_error_occurence]
        mongo: crate::repositories_types::tufa_server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndPostgres {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndMongo {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        mongo: crate::repositories_types::tufa_server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresAndMongo {
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        mongo: crate::repositories_types::tufa_server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndPostgresAndMongo {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityErrorNamed<'a>,
        #[eo_error_occurence]
        mongo: crate::repositories_types::tufa_server::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}