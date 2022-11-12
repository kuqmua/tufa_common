use crate::common::git::git_info::GitInformation;
use compile_time_git_info::CompileTimeGitInfoTufaCommon;

#[derive(Debug, CompileTimeGitInfoTufaCommon)]
pub struct GitInfoGlobalStaticConst {}

// use std::io::Read;
// struct Buf<const N: usize>([u8; N]);

// const fn len(strs: &[&str]) -> usize {
//     let mut result = 0;
//     let mut remaining = strs;
//     while let [current, tail @ ..] = remaining {
//         result += current.len();
//         remaining = tail;
//     }
//     result
// }

// const fn concat<const N: usize>(strs: &[&str]) -> Buf<N> {
//     let mut buffer = [0; N];
//     let mut position_in_buffer: usize = 0;
//     let mut remaining = strs;
//     while let [current, tail @ ..] = remaining {
//         let x = current.as_bytes();
//         let mut i = 0;
//         while i < x.len() {
//             buffer[position_in_buffer] = x[i];
//             position_in_buffer += 1;
//             i += 1;
//         }
//         remaining = tail;
//     }
//     Buf(buffer)
// }

// macro_rules! my_concat {
//     ($($x: expr),+ $(,)?) => {{
//         const STRS: &[&str] = &[$($x), +];
//         const LEN: usize = len(STRS);
//         const CONCAT_BUF: Buf<LEN> = concat(STRS);
//         unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0)}
//     }}
// }
// const fn generate() -> GitInformation<'static> {
//     const STRS: &[&str] = &["../.git/modules/src/{}/", "tufa_common"];
//     const LEN: usize = len(STRS);
//     const CONCAT_BUF: Buf<LEN> = concat(STRS);
//     const PATH: &str = unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0) };
//     // let first_guess = my_concat!("../.git/modules/src/{}/", repo_name);
//     // let first_guess = my_concat!("../.git/modules/src/{}/", repo_name
//     const F: bool = !std::path::Path::new(&PATH).is_dir();
//     // if !std::path::Path::new(&PATH).is_dir() {
//     //     panic!("is not a dir");
//     // };
//     // let full_path = &format!("{}{}", path, "logs/HEAD");
//     // let file = std::fs::File::open(std::path::Path::new(full_path))
//     //     .unwrap_or_else(|e| panic!("cannot open logs/HEAD file, error: \"{}\"", e));
//     // let mut buf_reader = std::io::BufReader::new(file);
//     // let mut git_logs_head_content = String::new();
//     // buf_reader
//     //     .read_to_string(&mut git_logs_head_content)
//     //     .unwrap_or_else(|e| panic!("cannot read_to_string from HEAD file, error: \"{}\"", e));
//     // let from_handle = "from ";
//     // let from_handle_index = git_logs_head_content
//     //     .find(from_handle)
//     //     .unwrap_or_else(|| panic!("no \"{}\" inside git_logs_head_content", from_handle));
//     // let git_extenstion_name = ".git";
//     // let dot_git_index = git_logs_head_content
//     //     .find(git_extenstion_name)
//     //     .unwrap_or_else(|| {
//     //         panic!(
//     //             "no \"{}\" inside git_logs_head_content",
//     //             git_extenstion_name
//     //         )
//     //     });
//     // // let repo_link_without_quotes = repo_link.replace('"', "\""); //bad, bad decision
//     // let repo_link_token_stream = git_logs_head_content
//     //     .get(from_handle_index + from_handle.len()..dot_git_index)
//     //     .unwrap_or_else(|| panic!("failed to get slice from git_logs_head_content"))
//     //     .to_string();
//     // //wtf
//     // let head_file_lines: Vec<&str> = git_logs_head_content.lines().collect::<Vec<&str>>();
//     // let last_head_file_line = head_file_lines
//     //     .last()
//     //     .expect("no last element inside git head file lines");
//     // let line_parts: Vec<&str> = last_head_file_line.split(' ').collect();
//     // let commit_id = line_parts
//     //     .get(1)
//     //     .unwrap_or_else(|| panic!("failed to get 1 element from line_parts as commit_id"))
//     //     .to_string();
//     // let author = line_parts
//     //     .get(2)
//     //     .unwrap_or_else(|| panic!("failed to get 2 element from line_parts as author"))
//     //     .to_string();
//     // let unhandled_author_email = line_parts
//     //     .get(3)
//     //     .unwrap_or_else(|| {
//     //         panic!("failed to get 3 element from line_parts as slice for author_email")
//     //     })
//     //     .to_string();
//     // let author_email = unhandled_author_email
//     //     .get(1..unhandled_author_email.len() - 1)
//     //     .unwrap_or_else(|| panic!("failed to get slice from line_parts as author_email"))
//     //     .to_string();
//     // let commit_unix_time = line_parts
//     //     .get(4)
//     //     .unwrap_or_else(|| panic!("failed to get 4 element from line_parts as commit_unix_time"))
//     //     .to_string();
//     // let commit_unix_time_index = last_head_file_line
//     //     .find(&commit_unix_time)
//     //     .unwrap_or_else(|| {
//     //         panic!(
//     //             "cannot find \"{}\" for the second time inside {}",
//     //             commit_unix_time, git_logs_head_content
//     //         )
//     //     });
//     // let part_after_commit_unix_time = last_head_file_line
//     //     .get(commit_unix_time_index + commit_unix_time.len() + 1..)
//     //     .unwrap_or_else(|| {
//     //         panic!("failed to get slice from last_head_file_line as part_after_commit_unix_time")
//     //     })
//     //     .to_string();
//     // let backslash_t = "\t";
//     // let backslash_t_index = part_after_commit_unix_time
//     //     .find(backslash_t)
//     //     .unwrap_or_else(|| {
//     //         panic!(
//     //             "no \"{}\" inside \"{}\"",
//     //             backslash_t, part_after_commit_unix_time
//     //         )
//     //     });
//     // let timezone = part_after_commit_unix_time
//     //     .get(..backslash_t_index)
//     //     .unwrap_or_else(|| {
//     //         panic!("failed to get slice from part_after_commit_unix_time as timezone")
//     //     })
//     //     .to_string();
//     // let message = part_after_commit_unix_time
//     //     .get(backslash_t_index + 1..)
//     //     .unwrap_or_else(|| {
//     //         panic!("failed to get slice from part_after_commit_unix_time as message")
//     //     });
//     GitInformation {
//         commit_id: "",
//         repo_link: "",
//         author: "",
//         author_email: "",
//         commit_unix_time: "",
//         timezone: "",
//         message: "",
//     }
// }
