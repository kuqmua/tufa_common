pub mod code_occurence_prepare_for_log;
pub mod error_log;
pub mod form_error_path;
pub mod get_code_occurence;
pub mod hashmap_display_foreign_type_display_foreign_type_to_hashmap_string_string;
pub mod hashmap_display_foreign_type_display_foreign_type_to_string;
pub mod hashmap_display_foreign_type_impl_display_to_hashmap_string_impl_display;
pub mod hashmap_display_foreign_type_impl_display_to_string;
pub mod hashmap_display_foreign_type_to_string_with_config_to_string;
pub mod hashmap_display_foreign_type_to_string_without_config_to_string;
pub mod hashmap_impl_display_display_foreign_type_to_hashmap_impl_display_string;
pub mod hashmap_impl_display_display_foreign_type_to_string;
pub mod hashmap_impl_display_impl_display_to_string;
pub mod hashmap_impl_display_to_string_with_config_to_string;
pub mod hashmap_impl_display_to_string_without_config_to_string;
pub mod helpers;
pub mod lines_space_backslash;
pub mod source_to_string_with_config;
pub mod source_to_string_without_config;
#[cfg(test)]
pub mod test;
pub mod to_string_with_config;
pub mod to_string_without_config;
pub mod vec_display_foreign_type_into_vec_string;
pub mod vec_display_foreign_type_to_string;
pub mod vec_impl_display_to_string;
pub mod vec_to_string_with_config_to_string;
pub mod vec_to_string_without_config_to_string;
//todo - rename impl_display to display
//todo - rename to_vec -> into_vec and to_hashmap s-> into_hashmap (takes self, not &self)
