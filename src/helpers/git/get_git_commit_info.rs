use crate::constants::GIT_COMMIT_ID_LENGTH;
use crate::constants::GIT_PATH_FROM_SUBMODULE;
use crate::helpers::git::git_info::GitInformation;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

impl GitInformation {
    // #[deny(
    //     clippy::indexing_slicing,
    //     clippy::unwrap_used,
    //     clippy::integer_arithmetic,
    //     clippy::float_arithmetic
    // )]
    pub fn get_git_commit_info(repo_git_path: &str, repo_name: &str) -> GitInformation {
        let path: String;
        let git_folder_name = ".git";
        if Path::new(&format!("{}{}/", git_folder_name, repo_git_path)).is_dir() {
            path = format!("{}.git/", repo_git_path); //for docker image or run not as tufa_project repo, as git clone tufa_server
        } else if Path::new(&format!("{}.git/", GIT_PATH_FROM_SUBMODULE)).is_dir() {
            path = format!("{}.git/modules/{}/", GIT_PATH_FROM_SUBMODULE, repo_name);
        } else {
            panic!("error: no .git folder inside current and parent dir(this message should be displayed only on compile time)")
        }
        let commit_id: String;
        let repo_link: String;
        let last_commit: String;
        let author: String;
        let author_email: String;
        let commit_unix_time: String;
        let timezone: String;
        let message: String;
        match File::open(Path::new(&format!("{}{}", path, "logs/HEAD"))) {
            Err(e) => panic!("error: {:#?}", e),
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut git_logs_head_content = String::new();
                if let Err(e) = buf_reader.read_to_string(&mut git_logs_head_content) {
                    panic!("error: {:#?}", e);
                }
                let from_handle = "from ";
                match git_logs_head_content.find(from_handle) {
                    None => panic!("no \"{}\" inside git_logs_head_content", from_handle),
                    Some(index) => {
                        let git_extenstion_name = ".git";
                        match git_logs_head_content.find(git_extenstion_name) {
                            None => panic!(
                                "no \"{}\" inside git_logs_head_content",
                                git_extenstion_name
                            ),
                            Some(dot_git_index) => {
                                repo_link = git_logs_head_content
                                    [index + from_handle.len()..dot_git_index]
                                    .to_string();
                                let head_file_lines: Vec<&str> =
                                    git_logs_head_content.lines().collect::<Vec<&str>>();
                                let last_head_file_line = head_file_lines
                                    .last()
                                    .expect("no last element inside git head file lines");
                                let line_parts: Vec<&str> =
                                    last_head_file_line.split(' ').collect();
                                last_commit = line_parts[0].to_string();
                                commit_id = line_parts[1].to_string();
                                author = line_parts[2].to_string();
                                author_email = line_parts[3].to_string();
                                commit_unix_time = line_parts[4].to_string();
                                match last_head_file_line.find(&commit_unix_time) {
                                    None => panic!(
                                        "cannot find \"{}\" for the second time inside {}",
                                        commit_unix_time, git_logs_head_content
                                    ),
                                    Some(commit_unix_time_index) => {
                                        let part_after_commit_unix_time = last_head_file_line
                                            [commit_unix_time_index + commit_unix_time.len() + 1..]
                                            .to_string();
                                        let space_symbol = ' ';
                                        let backslash_t = "\t";
                                        let first_space_index = part_after_commit_unix_time
                                            .find(space_symbol)
                                            .expect(&format!(
                                                "no \"{}\" symbol inside \"{}\"",
                                                space_symbol, part_after_commit_unix_time
                                            ));
                                        let unhandled_timezone = part_after_commit_unix_time
                                            [1..first_space_index]
                                            .to_string();
                                        match unhandled_timezone.find(backslash_t) {
                                            None => panic!(
                                                "no \"{}\" inside \"{}\"",
                                                backslash_t, unhandled_timezone
                                            ),
                                            Some(backslash_t_index) => {
                                                timezone = unhandled_timezone[..backslash_t_index]
                                                    .to_string();
                                            }
                                        }
                                        match part_after_commit_unix_time.find(backslash_t) {
                                            None => panic!(
                                                "no \"{}\" inside \"{}\"",
                                                backslash_t, part_after_commit_unix_time
                                            ),
                                            Some(backslash_t_index) => {
                                                message = part_after_commit_unix_time
                                                    [backslash_t_index..]
                                                    .to_string();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        GitInformation {
            commit_id,
            repo_link,
            last_commit,
            author,
            author_email,
            commit_unix_time,
            timezone,
            message,
        }
    }
}
