use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use std::io::Write;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingFromCrate,
    ImplGetWhereWasOriginOrWrapperFromCrate,
    ImplGetSourceFromCrate,
    ImplDisplayForError,
)]
pub struct WriteBytesIntoFileSyncOriginError {
    pub source: std::io::Error,
    pub where_was: WhereWas,
}

pub fn write_bytes_into_file_sync(
    path: &std::path::Path,
    bytes: &[u8],
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<WriteBytesIntoFileSyncOriginError>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(
                WriteBytesIntoFileSyncOriginError::init_error_with_possible_trace(
                    e,
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::common::where_was::GitInfoForWhereWas {
                            commit_id: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                            ),
                            repo_link: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                            ),
                            author: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.author,
                            ),
                            author_email: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .author_email,
                            ),
                            commit_unix_time: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .commit_unix_time,
                            ),
                            timezone: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                            ),
                            message: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.message,
                            ),
                        },
                    },
                    source_place_type,
                    should_trace,
                ),
            ));
        }
    }
    match std::fs::File::create(path) {
        Err(e) => {
            return Err(Box::new(
                WriteBytesIntoFileSyncOriginError::init_error_with_possible_trace(
                    e,
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::common::where_was::GitInfoForWhereWas {
                            commit_id: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                            ),
                            repo_link: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                            ),
                            author: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.author,
                            ),
                            author_email: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .author_email,
                            ),
                            commit_unix_time: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .commit_unix_time,
                            ),
                            timezone: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                            ),
                            message: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.message,
                            ),
                        },
                    },
                    source_place_type,
                    should_trace,
                ),
            ));
        }
        Ok(mut file) => {
            if let Err(e) = file.write_all(bytes) {
                return Err(Box::new(
                    WriteBytesIntoFileSyncOriginError::init_error_with_possible_trace(
                        e,
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::common::where_was::GitInfoForWhereWas {
                                commit_id: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_id,
                                ),
                                repo_link: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .repo_link,
                                ),
                                author: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author,
                                ),
                                author_email: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author_email,
                                ),
                                commit_unix_time: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_unix_time,
                                ),
                                timezone: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .timezone,
                                ),
                                message: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .message,
                                ),
                            },
                        },
                        source_place_type,
                        should_trace,
                    ),
                ));
            }
            if let Err(e) = file.sync_all() {
                return Err(Box::new(
                    WriteBytesIntoFileSyncOriginError::init_error_with_possible_trace(
                        e,
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::common::where_was::GitInfoForWhereWas {
                                commit_id: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_id,
                                ),
                                repo_link: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .repo_link,
                                ),
                                author: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author,
                                ),
                                author_email: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author_email,
                                ),
                                commit_unix_time: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_unix_time,
                                ),
                                timezone: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .timezone,
                                ),
                                message: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .message,
                                ),
                            },
                        },
                        source_place_type,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
