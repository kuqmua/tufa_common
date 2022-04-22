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
        let commit_id: String;
        let repo_link: String;
        match File::open(Path::new(&format!("{}{}", path, "logs/HEAD"))) {
            Err(e) => panic!("error1"),
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut orig_head_content = String::new();
                match buf_reader.read_to_string(&mut orig_head_content) {
                    Err(e) =>  panic!("error2"),
                    Ok(_) => {
                        print!("{}",orig_head_content);
                        //
                        let end_line_to_find = "from ";
                        match orig_head_content.find(end_line_to_find) {
                            None => {
                                panic!("none");
                                // branch = format!(
                                //     "error: no line break character inside '{orig_head_content}'"
                                // );
                                // repo_link = format!(
                                //     "error: no line break character inside '{orig_head_content}'"
                                // );
                            }
                            Some(index) => {
                                match orig_head_content.find(".git") {
                                    None => panic!("none"),
                                    Some(dot_git_index) => {
                                        repo_link = orig_head_content[index
                                                            + "from ".len()
                                                            ..dot_git_index]
                                                            .to_string();
                                        println!("repo_link {}", repo_link);
                                        let v: Vec<(usize, &str)> = orig_head_content.match_indices(" kuqmua").collect();
                                        println!("v {:#?}", v.last());
                                        let innd = v.last().expect("no last element").0;
                                        // fac3c3cc376a7dc5441dc3f91782f2a202fbe8e9
                                        commit_id = orig_head_content[innd - "fac3c3cc376a7dc5441dc3f91782f2a202fbe8e9".len()..innd].to_string();
                                        println!("{}/tree/{}/", repo_link,  commit_id);
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
        GitInformation {
            commit_id,
            repo_link,
        }
    }
}
