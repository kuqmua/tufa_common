#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CheckAvailabilityError<'a> {
    Net {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Postgres {
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<
            'a,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Mongo {
        #[eo_error_occurence]
        mongo: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndPostgres {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndMongo {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        #[eo_error_occurence]
        mongo: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresAndMongo {
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
        #[eo_error_occurence]
        mongo: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NetAndPostgresAndMongo {
        #[eo_error_occurence]
        net: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        #[eo_error_occurence]
        postgres: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
        #[eo_error_occurence]
        mongo: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}