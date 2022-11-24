use ansi_term::Colour;

pub trait PrintTypeTrait {
    fn is_prints_enabled(&self) -> bool;
    fn get_color(&self) -> Colour;
}
