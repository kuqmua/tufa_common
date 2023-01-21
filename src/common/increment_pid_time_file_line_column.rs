#[derive(Debug, Clone)]
pub struct IncrementPidTimeFileLineColumn {
    pub increment: u64,                                      //potential overflow?
    pub concurrent_or_parallel_execution_index: Option<u64>, //for information about parallel error result like inside join_all!() or join!()
    pub pid_time_file_line_column: crate::common::pid_time_file_line_column::PidTimeFileLineColumn,
}

impl IncrementPidTimeFileLineColumn {
    pub fn new(
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            increment: 0, //potential overflow?
            concurrent_or_parallel_execution_index: None,
            pid_time_file_line_column:
                crate::common::pid_time_file_line_column::PidTimeFileLineColumn::new(
                    crate::common::file_line_column::FileLineColumn { file, line, column },
                ),
        }
    }
}

impl crate::traits::get_process_id::GetProcessId for IncrementPidTimeFileLineColumn {
    fn get_process_id(&self) -> &u32 {
        &self.pid_time_file_line_column.get_process_id()
    }
}

impl crate::traits::get_time::GetTime for IncrementPidTimeFileLineColumn {
    fn get_time(&self) -> std::time::Duration {
        self.pid_time_file_line_column.get_time()
    }
}

impl crate::traits::fields::GetFile for IncrementPidTimeFileLineColumn {
    fn get_file(&self) -> &String {
        &self.pid_time_file_line_column.get_file()
    }
}

impl crate::traits::fields::GetLine for IncrementPidTimeFileLineColumn {
    fn get_line(&self) -> &u32 {
        &self.pid_time_file_line_column.get_line()
    }
}

impl crate::traits::fields::GetColumn for IncrementPidTimeFileLineColumn {
    fn get_column(&self) -> &u32 {
        &self.pid_time_file_line_column.get_column()
    }
}
