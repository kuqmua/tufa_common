pub trait SeparatorSymbolTrait {
    fn symbol(&self) -> &str;
    fn pop_last(&self, string: &mut String);
}
