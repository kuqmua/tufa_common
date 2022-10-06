use crate::helpers::git::git_info::GitInformation;
use crate::traits::where_was_trait::WhereWasTrait;
use chrono::prelude::DateTime;
use chrono::FixedOffset;

#[derive(Debug, Clone)]
pub struct WhereWas {
    pub time: DateTime<FixedOffset>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

//cannot implement that, cause SourcePlaceType::None => String::from("") would be incorrect for tracing
// impl WhereWas {
//     pub fn get_place_type(
//         &self,
//         place_type: crate::config_mods::source_place_type::SourcePlaceType,
//     ) -> String {
//         match place_type {
//             tufa_common::config::source_place_type::SourcePlaceType::Source => {
//                 self.file_line_column()
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::Github => {
//                 self.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data)
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::None => String::from(""),
//         }
//     }
// }

// impl Display for WhereWas {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match CONFIG.is_debug_implementation_enable {
//             true => write!(f, "{:#?}", self),
//             false => match crate::lazy_static::config::CONFIG.source_place_type {
//                 tufa_common::config::source_place_type::SourcePlaceType::Source => {
//                     write!(f, "{}", self.file_line_column())
//                 }
//                 tufa_common::config::source_place_type::SourcePlaceType::Github => {
//                     write!(f, "{}", self.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data))
//                 }
//                 tufa_common::config::source_place_type::SourcePlaceType::None => {
//                     write!(f, "")
//                 }
//             },
//         }
//     }
// }

impl WhereWasTrait for WhereWas {
    fn readable_time(&self) -> String {
        self.time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    fn file_line_column(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    fn github_file_line_column(&self, git_info: &GitInformation) -> String {
        git_info.get_git_source_file_link(self.file, self.line)
    }
}

#[derive(Debug, Clone)]
pub enum WhereWasOneOrMany {
    One(WhereWasWithAddition),
    Many(Vec<WhereWasWithAddition>),
}

impl WhereWasOneOrMany {
    pub fn into_vec(self) -> Vec<WhereWasWithAddition> {
        let mut vec = Vec::new();
        match self {
            crate::where_was::WhereWasOneOrMany::One(where_was_with_addition) => {
                vec.push(where_was_with_addition);
            }
            crate::where_was::WhereWasOneOrMany::Many(where_was_with_addition_vec) => {
                where_was_with_addition_vec.into_iter().for_each(|w| {
                    vec.push(w);
                });
            }
        }
        vec
    }
}

#[derive(Debug, Clone)]
pub struct WhereWasWithAddition {
    pub additional_info: Option<String>,
    pub where_was: WhereWas,
}

impl WhereWasWithAddition {
    pub fn get_file_line_column(
        &self,
        source_place_type: &crate::config::source_place_type::SourcePlaceType,
        git_info: &GitInformation,
    ) -> String {
        match source_place_type {
            crate::config::source_place_type::SourcePlaceType::Source => {
                match &self.additional_info {
                    None => self.where_was.file_line_column(),
                    Some(additional) => {
                        format!("{} {}", additional, self.where_was.file_line_column())
                    }
                }
            }
            crate::config::source_place_type::SourcePlaceType::Github => {
                match &self.additional_info {
                    None => self.where_was.github_file_line_column(git_info),
                    Some(additional) => format!(
                        "{} {}",
                        additional,
                        self.where_was.github_file_line_column(git_info)
                    ),
                }
            }
            crate::config::source_place_type::SourcePlaceType::None => String::from(""),
        }
    }
    // pub fn get_file_line_column_separated(
    //     &self,
    //     source_place_type: &crate::config::source_place_type::SourcePlaceType,
    //     git_info: &GitInformation,
    // ) {

    // }
}
