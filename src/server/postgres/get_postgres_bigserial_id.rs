pub trait GetPostgresBigserialId {
    fn get_postgres_bigserial_id(&self) -> &i64; //todo postgres bigserial max = i64::MAX, but invalid in i64 < 0
}
