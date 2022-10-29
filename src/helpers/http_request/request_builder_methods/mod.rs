pub mod bytes;
pub mod bytes_stream; //async only
pub mod content_length;
pub mod cookies;
pub mod copy_to; //sync only
pub mod error_for_status;
pub mod extensions;
pub mod extensions_mut;
pub mod headers;
pub mod headers_mut;
pub mod json;
pub mod remote_addr;
pub mod status;
pub mod text;
pub mod text_with_charset;
pub mod upgrade; //async only
pub mod url;
pub mod version;
// pub mod error_for_status_ref;//dont need it i think
