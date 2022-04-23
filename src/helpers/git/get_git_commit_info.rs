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
        let full_path = &format!("{}{}", path, "logs/HEAD");
        let file = File::open(Path::new(full_path))
            .unwrap_or_else(|e| panic!("cannot open HEAD file, error: \"{}\"", e));
        let mut buf_reader = BufReader::new(file);
        let mut git_logs_head_content = String::new();
        buf_reader
            .read_to_string(&mut git_logs_head_content)
            .unwrap_or_else(|e| panic!("cannot read to string from HEAD file, error: \"{}\"", e));
        let from_handle = "from ";
        let from_handle_index = git_logs_head_content
            .find(from_handle)
            .unwrap_or_else(|| panic!("no \"{}\" inside git_logs_head_content", from_handle));
        let git_extenstion_name = ".git";
        let dot_git_index = git_logs_head_content
            .find(git_extenstion_name)
            .unwrap_or_else(|| {
                panic!(
                    "no \"{}\" inside git_logs_head_content",
                    git_extenstion_name
                )
            });
        let repo_link =
            git_logs_head_content[from_handle_index + from_handle.len()..dot_git_index].to_string();
        let head_file_lines: Vec<&str> = git_logs_head_content.lines().collect::<Vec<&str>>();
        let last_head_file_line = head_file_lines
            .last()
            .expect("no last element inside git head file lines");
        let line_parts: Vec<&str> = last_head_file_line.split(' ').collect();
        let last_commit = line_parts[0].to_string();
        let commit_id = line_parts[1].to_string();
        let author = line_parts[2].to_string();
        let author_email = line_parts[3][1..line_parts[3].len() - 1].to_string();
        let commit_unix_time = line_parts[4].to_string();
        let commit_unix_time_index =
            last_head_file_line
                .find(&commit_unix_time)
                .unwrap_or_else(|| {
                    panic!(
                        "cannot find \"{}\" for the second time inside {}",
                        commit_unix_time, git_logs_head_content
                    )
                });
        let part_after_commit_unix_time =
            last_head_file_line[commit_unix_time_index + commit_unix_time.len() + 1..].to_string();
        let backslash_t = "\t";
        let backslash_t_index = part_after_commit_unix_time
            .find(backslash_t)
            .unwrap_or_else(|| {
                panic!(
                    "no \"{}\" inside \"{}\"",
                    backslash_t, part_after_commit_unix_time
                )
            });
        let message = part_after_commit_unix_time[backslash_t_index..].to_string();
        let timezone = part_after_commit_unix_time[..backslash_t_index].to_string();
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
