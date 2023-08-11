pub trait GenerateBindIncrements {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String;
}

impl<SelfGeneric> GenerateBindIncrements for SelfGeneric
where
    SelfGeneric: crate::server::routes::helpers::get_inner_length::GetInnerLength,
{
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String {
        let mut increments = std::string::String::from("");
        for _ in 0..self.get_inner_length() {
            *increment += 1;
            increments.push_str(&format!("${increment}, "));
        }
        if let false = increments.is_empty() {
            increments.pop();
            increments.pop();
        }
        increments
    }
}
