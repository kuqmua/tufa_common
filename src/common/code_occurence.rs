// // #[derive(Debug, Clone)]
// pub struct CodeOccurenceWithArcUsage {
//     pub occurences: std::collections::HashMap<
//         std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes>,
//         Vec<crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn>,
//     >,
// }

// impl CodeOccurenceWithArcUsage {
//     pub fn new(
//         git_info: std::sync::Arc<crate::common::git::git_info::GitInformationWithoutLifetimes>,
//         file: String, //&'a str
//         line: u32,
//         column: u32,
//     ) -> Self {
//         Self {
//             occurences: std::collections::HashMap::from([(
//                 git_info,
//                 vec![crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn::new(file, line, column)],
//             )]),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct CodeOccurenceOldWay {
    pub git_info: crate::common::git::git_info::GitInformationWithoutLifetimes,
    pub time_file_line_column: crate::common::time_file_line_column::TimeFileLineColumn,
}

impl crate::traits::get_git_info::GetGitInfoWithoutLifetimes for CodeOccurenceOldWay {
    fn get_git_info_without_lifetimes(
        &self,
    ) -> &crate::common::git::git_info::GitInformationWithoutLifetimes {
        &self.git_info
    }
}

impl crate::traits::fields::GetFile for CodeOccurenceOldWay {
    fn get_file(&self) -> &String {
        &self.time_file_line_column.get_file()
    }
}

impl crate::traits::fields::GetLine for CodeOccurenceOldWay {
    fn get_line(&self) -> &u32 {
        &self.time_file_line_column.get_line()
    }
}

impl crate::traits::fields::GetColumn for CodeOccurenceOldWay {
    fn get_column(&self) -> &u32 {
        &self.time_file_line_column.get_column()
    }
}

#[derive(Debug, Clone)]
pub struct CodeOccurence {
    pub occurences: std::collections::HashMap<
        crate::common::git::git_info::GitInformationWithoutLifetimes,
        Vec<crate::common::increment_time_file_line_column::IncrementTimeFileLineColumn>,
    >,
}
