use crate::traits::console::Console;

pub trait ConfigLog {
    fn log(&self, log: String);
}

impl<SelfGeneric> ConfigLog for SelfGeneric
where
    SelfGeneric: crate::traits::fields::GetLogType + crate::traits::get_color::ErrorColorBold,
{
    fn log(&self, log: String) {
        let log_type = self.get_log_type();
        log_type.console(&self.get_error_color_bold(), log);
    }
}
