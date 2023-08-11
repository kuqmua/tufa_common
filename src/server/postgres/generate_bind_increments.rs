pub trait GenerateBindIncrements {
    fn generate_bind_increments(&self, increment: &mut u64) -> std::string::String;
}
//todo rewrite it as impl with generics somehow?
pub fn generate_bind_increments_helper<T>(
    iterable: &[T],
    increment: &mut u64,
) -> std::string::String {
    let mut increments = iterable
        .iter()
        .fold(std::string::String::from(""), |mut acc, _| {
            *increment += 1;
            acc.push_str(&format!("${increment}, "));
            acc
        });
    if let false = increments.is_empty() {
        increments.pop();
        increments.pop();
    }
    increments
}
