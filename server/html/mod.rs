use crate::prelude::*;

#[doc(hidden)]
pub mod assets;
#[doc(hidden)]
pub mod macros;
#[doc(hidden)]
pub mod error;

#[cfg(debug_assertions)]
const __SERVER: &str = concat!(
    "ARC v", env!("CARGO_PKG_VERSION"),
    "-", env!("GIT_HASH")
);
#[cfg(not(debug_assertions))]
const __SERVER: &str = concat!(
    "ARC v", env!("CARGO_PKG_VERSION")
);

/// Version of the software.
/// 
/// In debug builds, this is the version number and git hash.
/// In release builds, this is just the version number.
pub const SERVER: &str = __SERVER;

pub fn make_server_identifier() -> rocket::config::Ident {
    rocket::config::Ident::try_new(SERVER)
        .expect("Failed to create rocket ident")
}

pub type Response<T = String> = (rocket::http::ContentType, (rocket::http::Status, T));

pub type Result<O = Response,E = error::Error> = error::Result<O, E>;

#[doc(hidden)]
const SERVER_ERR: &str = concat!("The server encountered an internal",
" error and was unable to complete your request.");

#[rocket::catch(default)]
pub fn error_catcher(status: rocket::http::Status, _req: &rocket::Request) -> Response {
    // If it is a server error (500+)
    if (status.code - 400) >= 100 {
        return crate::res!(@HTML #500 error::ERROR_HTML
            .handlebars("message", "An Internal Server Error Occurred")
            .handlebars("error", SERVER_ERR)
            .handlebars("debug", SERVER)
        );
    }
    // Otherwise client error
    let error = format!("{}", status);
    crate::res!(@HTML #status; error::ERROR_HTML
        .handlebars("message", &error)
        .handlebars("error", &error)
        .handlebars("debug", SERVER)
    )
}
