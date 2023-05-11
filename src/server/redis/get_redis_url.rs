pub fn get_redis_url(
    config: &(
        impl crate::traits::config_fields::GetRedisIp
        + crate::traits::config_fields::GetRedisPort
    )
) -> String {
    format!("redis://{}:{}", config.get_redis_ip(), config.get_redis_port())
}