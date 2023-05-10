pub fn get_redis_url(
    config: &(
        impl crate::traits::fields::GetRedisIp
        + crate::traits::fields::GetRedisPort
    )
) -> String {
    format!("redis://{}:{}", config.get_redis_ip(), config.get_redis_port())
}