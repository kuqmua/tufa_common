#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum CheckAvailabilityError<'a> {
    Net {
        error: crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Postgres {
        error: crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<
            'a,
        >,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Mongo {
        error: crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    Many {
        inner_errors: Vec<CheckAvailabilityErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum CheckAvailabilityErrorEnum<'a> {
    Net(crate::server::net::net_check_availability::NetCheckAvailabilityError<'a>),
    Postgres(
        crate::server::postgres::postgres_check_availability::PostgresCheckAvailabilityError<'a>,
    ),
    Mongo(crate::server::mongo::mongo_check_availability::MongoCheckAvailabilityError<'a>),
}
