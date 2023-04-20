#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CheckAvailabilityError<'a> {
    Net {
        #[eo_error_occurence]
        error: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Postgres {
        #[eo_error_occurence]
        error: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<
            'a,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Mongo {
        #[eo_error_occurence]
        error: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Many {
        #[eo_vec_error_occurence]
        inner_errors: Vec<CheckAvailabilityErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CheckAvailabilityErrorEnum<'a> {
    Net(crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>),
    Postgres(
        crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
    ),
    Mongo(crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>),
}
