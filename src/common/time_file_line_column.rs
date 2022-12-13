#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: std::time::Duration,
    pub file_line_column: crate::common::file_line_column::FileLineColumn,
}

impl TimeFileLineColumn {
    pub fn new(file_line_column: crate::common::file_line_column::FileLineColumn) -> Self {
        Self {
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            file_line_column,
        }
    }
}

impl crate::traits::get_time::GetTime for TimeFileLineColumn {
    fn get_time(&self) -> std::time::Duration {
        self.time
    }
}

impl crate::traits::get_file::GetFile for TimeFileLineColumn {
    fn get_file(&self) -> &String {
        &self.file_line_column.get_file()
    }
}

impl crate::traits::get_line::GetLine for TimeFileLineColumn {
    fn get_line(&self) -> &u32 {
        &self.file_line_column.get_line()
    }
}

impl crate::traits::get_column::GetColumn for TimeFileLineColumn {
    fn get_column(&self) -> &u32 {
        &self.file_line_column.get_column()
    }
}
