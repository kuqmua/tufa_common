pub fn get_redis_url(ip: &str, port: u16) -> String {
    format!("redis://{}:{}", ip, port)
}
