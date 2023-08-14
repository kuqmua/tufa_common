// pub struct TypedSession(actix_session::Session);

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum InsertUserIdErrorNamed {
//     SessionInsert {
//         #[eo_display]
//         session_insert: actix_session::SessionInsertError,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// impl TypedSession {
//     const USER_ID_KEY: &'static str = "user_id";
//     pub fn renew(&self) {
//         self.0.renew();
//     }
//     pub fn insert_user_id<'a>(
//         &self,
//         user_id: uuid::Uuid,
//     ) -> Result<(), InsertUserIdErrorNamed> {
//         match self.0.insert(Self::USER_ID_KEY, user_id) {
//             Err(e) => Err(InsertUserIdErrorNamed::SessionInsert {
//                 session_insert: e,
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             }),
//             Ok(_) => Ok(()),
//         }
//     }
//     pub fn get_user_id(&self) -> Result<Option<uuid::Uuid>, actix_session::SessionGetError> {
//         self.0.get(Self::USER_ID_KEY)
//     }
//     pub fn log_out(self) {
//         self.0.purge()
//     }
// }

// impl actix_web::FromRequest for TypedSession {
//     // This is a complicated way of saying
//     // "We return the same error returned by the
//     // implementation of `actix_web::FromRequest` for `Session`".
//     type Error = <actix_session::Session as actix_web::FromRequest>::Error;
//     // Rust does not yet support the `async` syntax in traits.
//     // From request expects a `Future` as return type to allow for extractors
//     // that need to perform asynchronous operations (e.g. a HTTP call)
//     // We do not have a `Future`, because we don't perform any I/O,
//     // so we wrap `TypedSession` into `std::future::Ready` to convert it into a `Future` that
//     // resolves to the wrapped value the first time it's polled by the executor.
//     type Future = std::future::Ready<Result<TypedSession, Self::Error>>;
//     fn from_request(
//         req: &actix_web::HttpRequest,
//         _payload: &mut actix_web::dev::Payload,
//     ) -> Self::Future {
//         std::future::ready(Ok(TypedSession({
//             use actix_session::SessionExt;
//             req.get_session()
//         })))
//     }
// }
