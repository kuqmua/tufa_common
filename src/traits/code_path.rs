// use crate::config_mods::source_place_type::SourcePlaceType;
// use crate::traits::get_git_source_file_link::GetGitSourceFileLink;
// use crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime;

// pub trait CodePathLifetime<'a> {
//     fn get_code_path_lifetime(
//         &self,
//         source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
//     ) -> String;
//     fn get_project_code_path_lifetime(&self) -> String;
//     fn get_github_code_path_lifetime(&self) -> String; //theoretically can make it const fn
// }

// impl<'a, SelfGeneric> CodePathLifetime<'a> for SelfGeneric
// where
//     SelfGeneric: crate::traits::fields::GetFile
//         + crate::traits::fields::GetLine
//         + crate::traits::fields::GetColumn
//         + crate::traits::get_git_info::GetGitInfo<'a>
//         + crate::traits::get_git_source_file_link::GetGitSourceFileLinkLifetime<'a>,
// {
//     fn get_code_path_lifetime(&self, source_place_type: &SourcePlaceType) -> String {
//         match source_place_type {
//             SourcePlaceType::Source => self.get_project_code_path_lifetime(),
//             SourcePlaceType::Github => self.get_github_code_path_lifetime(),
//             SourcePlaceType::None => String::from(""), //todo maybe incorrect?
//         }
//     }
//     fn get_project_code_path_lifetime(&self) -> String {
//         format!(
//             "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
//             self.get_file(),
//             self.get_line(),
//             self.get_column()
//         )
//     }
//     //todo make it const fn
//     fn get_github_code_path_lifetime(&self) -> String {
//         let file = self.get_file().clone();
//         let backslash = "/";
//         match file.find(backslash) {
//             Some(index) => self
//                 .get_git_info()
//                 .clone()
//                 .get_git_source_file_link_lifetime(
//                     &file[index + backslash.len()..],
//                     *self.get_line(),
//                 ),
//             None => String::from("cant find backslash symbol in file path of location"),
//         }
//     }
// }
