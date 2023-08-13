pub trait StdStrFromStrWithLifetime<'a>: Sized {
    type Err;
    fn std_str_from_str_with_lifetime(s: &'a str) -> Result<Self, Self::Err>;
}
