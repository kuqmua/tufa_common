// #[derive(Debug, Clone)]
// pub struct IncrementTimeFileLineColumn {
//     pub increment: u64,                                      //potential overflow?
//     pub concurrent_or_parallel_execution_index: Option<u64>, //for information about parallel error result like inside join_all!() or join!()
//     pub time_file_line_column: TimeFileLineColumn,
// }

// impl IncrementTimeFileLineColumn {
//     pub fn new(
//         file: String, //&'a str
//         line: u32,
//         column: u32,
//     ) -> Self {
//         Self {
//             increment: 0, //potential overflow?
//             concurrent_or_parallel_execution_index: None,
//             time_file_line_column: TimeFileLineColumn::new(FileLineColumn { file, line, column }),
//         }
//     }
// }

// impl crate::traits::get_time::GetTime for IncrementTimeFileLineColumn {
//     fn get_time(&self) -> std::time::Duration {
//         self.time_file_line_column.get_time()
//     }
// }

// impl crate::traits::get_file::GetFile for IncrementTimeFileLineColumn {
//     fn get_file(&self) -> &String {
//         &self.time_file_line_column.get_file()
//     }
// }

// impl crate::traits::get_line::GetLine for IncrementTimeFileLineColumn {
//     fn get_line(&self) -> u32 {
//         self.time_file_line_column.get_line()
//     }
// }

// impl crate::traits::get_column::GetColumn for IncrementTimeFileLineColumn {
//     fn get_column(&self) -> u32 {
//         self.time_file_line_column.get_column()
//     }
// }
