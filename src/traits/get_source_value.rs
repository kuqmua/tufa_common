pub trait GetSourceValue<SelfGeneric> {
    fn get_source_value(&self) -> &SelfGeneric;
}
