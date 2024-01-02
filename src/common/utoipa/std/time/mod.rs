#[derive(utoipa::ToSchema)]//todo check somehow what its equal to std::time::Duration
pub struct StdTimeDuration {
    pub secs: u64,
    pub nanos: u32,
}