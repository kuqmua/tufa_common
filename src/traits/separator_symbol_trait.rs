pub trait SeparatorSymbolTrait {
    fn symbol(&self) -> &str;
    fn pop_last(&self, vec: &mut Vec<String>);
}
