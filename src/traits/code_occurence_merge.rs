// pub trait CodeOccurenceMerge {
//     fn code_occurence_merge(
//         &self,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
//     ) -> String;
// }

// impl<T> CodePath for T
// where
//     T: GetFile + GetLine + GetColumn,
// {
//     fn get_code_path(
//         &self,
//         git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
//         source_place_type: &SourcePlaceType,
//     ) -> String {
//         match source_place_type {
//             SourcePlaceType::Source => self.get_project_code_path(),
//             SourcePlaceType::Github => self.get_github_code_path(git_info),
//             SourcePlaceType::None => String::from(""), //todo maybe incorrect?
//         }
//     }
// }
