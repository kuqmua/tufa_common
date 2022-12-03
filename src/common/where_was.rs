use crate::common::git::git_info::GitInformation;
use crate::common::git::git_info::GitInformationWithoutLifetimes;
use crate::traits::where_was_trait::WhereWasTrait;

#[derive(Debug, Clone)]
pub struct WhereWas {
    pub time: std::time::Duration,
    // pub location: core::panic::Location<'static>, - location file() returns absolute path instead if relative in some cases like library or submodule
    pub file: String, //&'a str
    pub line: u32,
    pub column: u32,
    pub git_info: GitInformationWithoutLifetimes,
}

//cannot implement that, cause SourcePlaceType::None => String::from("") would be incorrect for tracing
// impl WhereWas {
//     pub fn get_place_type(
//         &self,
//         place_type: crate::config_mods::source_place_type::SourcePlaceType,
//     ) -> String {
//         match place_type {
//             tufa_common::config_mods::source_place_type::SourcePlaceType::Source => {
//                 self.file_line_column()
//             }
//             tufa_common::config_mods::source_place_type::SourcePlaceType::Github => {
//                 self.github_file_line_column(&crate::once_cell_globals::git_info::GIT_INFO)
//             }
//             tufa_common::config_mods::source_place_type::SourcePlaceType::None => String::from(""),
//         }
//     }
// }

// impl Display for WhereWas {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match CONFIG.is_debug_implementation_enable {
//             true => write!(f, "{:#?}", self),
//             false => match crate::once_cell_globals::config::CONFIG.source_place_type {
//                 tufa_common::config_mods::source_place_type::SourcePlaceType::Source => {
//                     write!(f, "{}", self.file_line_column())
//                 }
//                 tufa_common::config_mods::source_place_type::SourcePlaceType::Github => {
//                     write!(f, "{}", self.github_file_line_column(&crate::once_cell_globals::git_info::GIT_INFO))
//                 }
//                 tufa_common::config_mods::source_place_type::SourcePlaceType::None => {
//                     write!(f, "")
//                 }
//             },
//         }
//     }
// }

impl WhereWasTrait for WhereWas {
    fn readable_time(&self, timezone: i32) -> String {
        let datetime = chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.time)
            .with_timezone(&chrono::FixedOffset::east(timezone));
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    //todo make it const fn
    fn file_line_column(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    //todo make it const fn
    fn github_file_line_column(
        &self,
        git_info: &crate::common::git::git_info::GitInformationWithoutLifetimes,
    ) -> String {
        let file = self.file.clone();
        let backslash = "/";
        let index = file
            .find(backslash)
            .expect("cant find backslash symbol in file path of location"); //todo - bad code ?
        git_info.get_git_source_file_link(&file[index + backslash.len()..], self.line)
    }
}

#[derive(Debug, Clone)]
pub enum WhereWasOriginOrWrapper {
    None,
    One(WhereWasWithAddition),
    Many(Vec<WhereWasWithAddition>),
}

impl WhereWasOriginOrWrapper {
    pub fn into_vec(self) -> Vec<WhereWasWithAddition> {
        let mut vec = Vec::new();
        match self {
            crate::common::where_was::WhereWasOriginOrWrapper::One(where_was_with_addition) => {
                vec.push(where_was_with_addition);
            }
            crate::common::where_was::WhereWasOriginOrWrapper::Many(
                where_was_with_addition_vec,
            ) => {
                where_was_with_addition_vec.into_iter().for_each(|w| {
                    vec.push(w);
                });
            }
            crate::common::where_was::WhereWasOriginOrWrapper::None => (),
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
        source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    ) -> String {
        match source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                match &self.additional_info {
                    None => self.where_was.file_line_column(),
                    Some(additional) => {
                        format!("{} {}", additional, self.where_was.file_line_column())
                    }
                }
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                match &self.additional_info {
                    None => self
                        .where_was
                        .github_file_line_column(&self.where_was.git_info),
                    Some(additional) => format!(
                        "{} {}",
                        additional,
                        self.where_was
                            .github_file_line_column(&self.where_was.git_info)
                    ),
                }
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => String::from(""),
        }
    }
    // pub fn get_file_line_column_separated(
    //     &self,
    //     source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    //     git_info: &GitInformation,
    // ) {

    // }
}
