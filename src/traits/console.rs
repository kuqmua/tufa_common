pub trait ConsoleTrait {
    fn console(&self, style: ansi_term::Style, occurence: String);
}
