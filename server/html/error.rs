use crate::prelude::*;

/// The result type used by this crate.
pub type Result<O = (), E = Error> = core::result::Result<O, E>;

/// An enum of all possible errors that can occur in this crate.
#[non_exhaustive]
pub enum Error {
    /// An IO error occurred.
    Io(std::io::Error),
    /// A UTF-8 parsing error occurred.
    Utf8(std::string::FromUtf8Error),
    /// An error occurred that isn't covered by the other variants.
    Other(Box<dyn ToString + 'static>)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<Box<dyn ToString + 'static>> for Error {
    fn from(e: Box<dyn ToString + 'static>) -> Self {
        Self::Other(e)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(e: &'a str) -> Self {
        Self::Other(Box::new(e.to_string()))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Other(Box::new(e))
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io({:?})", e),
            Self::Utf8(e) => write!(f, "Utf8({:?})", e),
            Self::Other(e) => write!(f, "Other({:?})", e.to_string()),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Other(e) => write!(f, "{}", e.to_string()),
            other => write!(f, "{}", other),
        }
    }
}

impl std::error::Error for Error {}

/// The HTML for the error page.
pub const ERROR_HTML: &'static str = include_str!("./error.html");

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let error = format!("{}", self);
        let html = ERROR_HTML
            .handlebars("message", "An Internal Server Error Occurred")
            .handlebars("error", &error)
            .handlebars("debug", &super::SERVER);
        rocket::Response::build_from(html.respond_to(request)?)
            .header(rocket::http::ContentType::HTML)
            .status(rocket::http::Status::InternalServerError)
            .ok()
    }
}
