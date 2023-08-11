pub trait GenerateBindIncrements {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String;
}
