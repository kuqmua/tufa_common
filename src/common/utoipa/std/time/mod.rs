#[derive(utoipa::ToSchema)]//todo check somehow what its equal to std::time::Duration
pub struct StdTimeDuration {
    secs: u64,
    nanos: u32,
}