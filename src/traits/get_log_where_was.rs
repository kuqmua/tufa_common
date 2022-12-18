use crate::config_mods::log_type::LogType;
use crate::config_mods::source_place_type::SourcePlaceType;
use crate::traits::get_bunyan_where_was::GetBunyanWhereWas;
use crate::traits::get_json_where_was::GetJsonWhereWas;
use crate::traits::get_where_was_one_or_many::GetWhereWasOriginOrWrapper;

pub trait GetLogWhereWas {
    fn get_log_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        error: String,
    ) -> String;
}

impl<T> GetLogWhereWas for T
where
    T: GetWhereWasOriginOrWrapper + GetJsonWhereWas + GetBunyanWhereWas,
{
    fn get_log_where_was(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        error: String,
    ) -> String {
        match log_type {
            LogType::Tracing => self.get_json_where_was(source_place_type, error),
            LogType::Stack => self.get_bunyan_where_was(source_place_type, error),
            LogType::None => String::from(""), //todo -incorrect?
        }
    }
}
