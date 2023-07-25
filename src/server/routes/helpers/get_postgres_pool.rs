pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}
