pub trait IntoSerdeSerializeDeserialize{}

pub trait PostgresqlFilter{}

pub trait PostgresqlOrder{}

pub trait PostgresqlLimit{}

pub trait PostgersqlColumn<'a>:
    std::fmt::Debug
    + IntoSerdeSerializeDeserialize
    + utoipa::ToSchema<'a>
    + PostgresqlFilter
    + PostgresqlOrder
    + PostgresqlLimit
{}