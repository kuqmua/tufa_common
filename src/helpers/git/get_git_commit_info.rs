use crate::helpers::git::git_info::GitInformation;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use crate::constants::GIT_PATH_FROM_SUBMODULE;

impl GitInformation {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_git_commit_info(repo_git_path: &str, repo_name: &str) -> GitInformation {
        //todo: use only .git/logs/HEAD file for parsing
        let path: String;
        if Path::new(&format!("{}.git/", repo_git_path)).is_dir() {//for docker image or run not as tufa_project repo, as git clone tufa_server
            path = format!("{}.git/", repo_git_path);
        }
        else if Path::new(&format!("{}.git/", GIT_PATH_FROM_SUBMODULE)).is_dir() {
            path = format!("{}.git/modules/{}/", GIT_PATH_FROM_SUBMODULE, repo_name);
        }
        else {
            panic!("no .git folder inside current and parent dir. this message should be displayed only on compile time error")
        }
        //must not panic
        //todo: write a message on start in case of error get config info
        //todo: make it parallel or async
        let orig_head_string_path = format!("{}{}", path, "ORIG_HEAD");
        //todo: make it different for all submodules/repos (no .git folder inside submodule)
        //todo: can be two version - just only this repo or this repo as submodule
        let orig_head_path = Path::new(&orig_head_string_path);
        let commit_id: String;
        match File::open(orig_head_path) {
            Err(e) => commit_id = e.to_string(),
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut orig_head_content = String::new();
                match buf_reader.read_to_string(&mut orig_head_content) {
                    Err(e) => commit_id = e.to_string(),
                    Ok(_) => commit_id = orig_head_content.replace('\n', ""),
                }
            }
        }
        let fetch_head_string_path = format!("{}{}", path, "FETCH_HEAD");//todo: can be two version - just only this repo or this repo as submodule
        let fetch_head_path = Path::new(&fetch_head_string_path);
        let branch: String;
        let repo_link: String;
        match File::open(fetch_head_path) {
            Err(e) => {
                branch = e.to_string();
                repo_link = e.to_string();
            }
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut fetch_head_content = String::new();
                match buf_reader.read_to_string(&mut fetch_head_content) {
                    Err(e) => {
                        branch = e.to_string();
                        repo_link = e.to_string();
                    }
                    Ok(_) => {
                        let end_line_to_find = "\n";
                        match fetch_head_content.find(end_line_to_find) {
                            None => {
                                branch = format!(
                                    "error: no line break character inside '{fetch_head_content}'"
                                );
                                repo_link = format!(
                                    "error: no line break character inside '{fetch_head_content}'"
                                );
                            }
                            Some(index) => {
                                let first_line = fetch_head_content[..index].to_string();
                                let first_slice_to_find = "branch '";
                                let second_slice_to_find = "' of ";
                                match first_line.find(first_slice_to_find) {
                                    None => {
                                        branch = format!(
                                            "error: no '{first_slice_to_find}' inside '{first_line}'"
                                        );
                                        repo_link = format!(
                                            "error: no '{first_slice_to_find}' inside '{first_line}'"
                                        );
                                    }
                                    Some(first_branch_index) => {
                                        match first_line.find(second_slice_to_find) {
                                            None => {
                                                branch = format!("error: no '{second_slice_to_find}' inside '{first_line}'");
                                                repo_link = format!("error: no '{second_slice_to_find}' inside '{first_line}'");
                                            }
                                            Some(second_branch_index) => {
                                                branch = first_line[first_branch_index
                                                    + first_slice_to_find.len()
                                                    ..second_branch_index]
                                                    .to_string();
                                                repo_link = first_line[second_branch_index
                                                    + second_slice_to_find.len()..]
                                                    .to_string();
                                                //was .replace(':', "/")
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
            branch,
            repo_link,
        }
    }
}
