use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PidTimeFileLineColumn {
    pub time: std::time::Duration,
    pub file_line_column: crate::common::file_line_column::FileLineColumn,
    pub process_id: u32,
    pub hostname: String,
}

impl PidTimeFileLineColumn {
    pub fn new_file_line_column(
        file: String, //&'a str
        line: u32,
        column: u32,
    ) -> Self {
        let hostname_handle = match hostname::get() {
            Ok(os_string) => format!("{os_string:?}"),
            Err(_) => String::from("\"hostname::get() failed \""),
        };
        Self {
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
            file_line_column: crate::common::file_line_column::FileLineColumn {
                file,
                line,
                column,
            },
            process_id: std::process::id(),
            hostname: hostname_handle,
        }
    }
}

impl crate::traits::get_process_id::GetProcessId for PidTimeFileLineColumn {
    fn get_process_id(&self) -> &u32 {
        &self.process_id
    }
}

impl crate::traits::get_time::GetTime for PidTimeFileLineColumn {
    fn get_time(&self) -> std::time::Duration {
        self.time
    }
}

impl crate::traits::fields::GetFile for PidTimeFileLineColumn {
    fn get_file(&self) -> &String {
        &self.file_line_column.get_file()
    }
}

impl crate::traits::fields::GetLine for PidTimeFileLineColumn {
    fn get_line(&self) -> &u32 {
        &self.file_line_column.get_line()
    }
}

impl crate::traits::fields::GetColumn for PidTimeFileLineColumn {
    fn get_column(&self) -> &u32 {
        &self.file_line_column.get_column()
    }
}
