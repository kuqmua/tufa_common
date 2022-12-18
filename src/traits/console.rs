pub trait Console {
    fn console(&self, style: &ansi_term::Style, occurence: String);
}
