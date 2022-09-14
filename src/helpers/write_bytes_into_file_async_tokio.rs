// use crate::config_mods::lazy_static_config::CONFIG;
// use crate::where_was::WhereWas;
// use chrono::DateTime;
// use chrono::FixedOffset;
// use chrono::Local;
// use chrono::Utc;
// use impl_display::ImplDisplay;
// use std::path::Path;
// use tokio::io::AsyncWriteExt;

// #[derive(Debug)]
// pub struct WriteBytesIntoFileAsyncTokioError {
//     source: WriteBytesIntoFileAsyncTokioErrorEnum,
//     where_was: WhereWas,
// }

// #[derive(Debug)]
// pub enum WriteBytesIntoFileAsyncTokioErrorEnum {
//     StdFsCreateDirAll(std::io::Error),
//     TokioFsFileOpen(std::io::Error),
//     FileWriteAll(std::io::Error),
// }

// //
// // #[derive(
// //     Debug,
// //     ImplGetSourceForParentErrorStruct,
// //     // ImplGetWhereWasForErrorStruct,
// //     ImplDisplayForErrorStruct,
// //     InitError,
// //     InitErrorWithTracingForOriginalErrorStruct,
// // )]
// // pub struct MongoCheckAvailabilityError {
// //     source: MongoCheckAvailabilityErrorEnum,
// //     where_was: WhereWas,
// // }

// // impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
// //     for MongoCheckAvailabilityError
// // {
// //     fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
// //         tufa_common::where_was::WhereWasOneOrMany::One(
// //             tufa_common::where_was::WhereWasWithAddition {
// //                 additional_info: None,
// //                 where_was: self.where_was.clone(),
// //             },
// //         )
// //     }
// // }

// // impl MongoCheckAvailabilityError {
// //     fn init(
// //         source: MongoCheckAvailabilityErrorEnum,
// //         where_was: WhereWas,
// //         should_trace: bool,
// //     ) -> Self {
// //         match should_trace {
// //             true => Self::with_tracing(source, where_was),
// //             false => Self::new(source, where_was),
// //         }
// //     }
// // }
// //
// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub async fn write_bytes_into_file_async_tokio(
//     path: &Path,
//     bytes: &[u8],
// ) -> Result<(), Box<WriteBytesIntoFileAsyncTokioError>> {
//     if let Some(prefix) = path.parent() {
//         if let Err(e) = std::fs::create_dir_all(prefix) {
//             return Err(Box::new(WriteBytesIntoFileAsyncTokioError {
//                 source: WriteBytesIntoFileAsyncTokioErrorEnum::StdFsCreateDirAll(e),
//                 where_was: WhereWas {
//                     time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
//                         .with_timezone(&FixedOffset::east(CONFIG.timezone)),
//                     file: file!(),
//                     line: line!(),
//                     column: column!(),
//                 },
//             }));
//         }
//     }
//     match tokio::fs::File::open(path).await {
//         Err(e) => Err(Box::new(WriteBytesIntoFileAsyncTokioError {
//             source: WriteBytesIntoFileAsyncTokioErrorEnum::TokioFsFileOpen(e),
//             where_was: WhereWas {
//                 time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
//                     .with_timezone(&FixedOffset::east(CONFIG.timezone)),
//                 file: file!(),
//                 line: line!(),
//                 column: column!(),
//             },
//         })),
//         Ok(mut file) => {
//             if let Err(e) = file.write_all(bytes).await {
//                 return Err(Box::new(WriteBytesIntoFileAsyncTokioError {
//                     source: WriteBytesIntoFileAsyncTokioErrorEnum::FileWriteAll(e),
//                     where_was: WhereWas {
//                         time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
//                             .with_timezone(&FixedOffset::east(CONFIG.timezone)),
//                         file: file!(),
//                         line: line!(),
//                         column: column!(),
//                     },
//                 }));
//             }
//             Ok(())
//         }
//     }
// }
