/// A macro to create a new error.
/// 
/// This macro is used to create a new error and return it from a function.
/// 
/// # Example
/// 
/// ```rust
/// 
#[macro_export]
macro_rules! err {
    ($($args:tt)*) => {{
        $crate::__internals::log::error!($($args)*);
        return Err(format!($($args)*).into());
    }};
}

pub use err;

#[macro_export]
macro_rules! res {
    (@$content_type:expr; #$code:expr; $body:expr) => ($crate::res!(!
        $content_type; $code; $body
    ));
    (@$content_type:expr; #$code:literal $body:expr) => ($crate::res!(!
        $content_type; $crate::html::macros::get_status($code); $body
    ));
    (@$content_type:expr; #$code:ident $body:expr) => ($crate::res!(!
        $content_type; $crate::html::macros::Status::$code; $body
    ));

    (@$content_type:ident #$code:expr; $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $code; $body
    ));
    (@$content_type:ident #$code:literal $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $crate::html::macros::get_status($code); $body
    ));
    (@$content_type:ident #$code:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $crate::html::macros::Status::$code; $body
    ));


    (#$code:expr; @$content_type:expr; $body:expr) => ($crate::res!(@@
        $content_type; $code; $body
    ));
    (#$code:literal @$content_type:expr; $body:expr) => ($crate::res!(@@
        $content_type; $crate::html::macros::get_status($code); $body
    ));
    (#$code:ident @$content_type:expr; $body:expr) => ($crate::res!(!
        $content_type; $crate::html::macros::Status::$code; $body
    ));

    (#$code:expr; @$content_type:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type; $code; $body
    ));
    (#$code:literal @$content_type:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $crate::html::macros::get_status($code); $body
    ));
    (#$code:ident @$content_type:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $crate::html::macros::Status::$code; $body
    ));

    (#$code:expr; $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::Plain; $code; $body
    ));
    (#$code:literal $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::Plain;
        $crate::html::macros::get_status($code); $body
    ));
    (#$code:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::Plain;
        $crate::html::macros::Status::$code; $body
    ));


    (@$content_type:ident $body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::$content_type;
        $crate::html::macros::Status::Ok; $body
    ));
    (@$content_type:expr; $body:expr) => ($crate::res!(!
        $content_type; $crate::html::macros::Status::Ok; $body
    ));


    ($body:expr) => ($crate::res!(!
        $crate::html::macros::ContentType::Plain;
        $crate::html::macros::Status::Ok;
        $body
    ));
    (! $content_type: expr; $code: expr; $body: expr ) => (
        ($content_type, ($code, $body))
    )
}

pub use res;

pub use rocket::http::ContentType;
pub use rocket::http::Status;
pub const fn get_status(code: u16) -> Status {
    match code {
        100 => Status::Continue,
        101 => Status::SwitchingProtocols,
        102 => Status::Processing,
        // 103 => Status::EarlyHints,

        200 => Status::Ok,
        201 => Status::Created,
        202 => Status::Accepted,
        203 => Status::NonAuthoritativeInformation,
        204 => Status::NoContent,
        205 => Status::ResetContent,
        206 => Status::PartialContent,
        207 => Status::MultiStatus,
        208 => Status::AlreadyReported,
        226 => Status::ImUsed,

        300 => Status::MultipleChoices,
        301 => Status::MovedPermanently,
        302 => Status::Found,
        303 => Status::SeeOther,
        304 => Status::NotModified,
        305 => Status::UseProxy,
        // 306 => Status::SwitchProxy,
        307 => Status::TemporaryRedirect,
        308 => Status::PermanentRedirect,

        400 => Status::BadRequest,
        401 => Status::Unauthorized,
        402 => Status::PaymentRequired,
        403 => Status::Forbidden,
        404 => Status::NotFound,
        405 => Status::MethodNotAllowed,
        406 => Status::NotAcceptable,
        407 => Status::ProxyAuthenticationRequired,
        408 => Status::RequestTimeout,
        409 => Status::Conflict,
        410 => Status::Gone,
        411 => Status::LengthRequired,
        412 => Status::PreconditionFailed,
        413 => Status::PayloadTooLarge,
        414 => Status::UriTooLong,
        415 => Status::UnsupportedMediaType,
        416 => Status::RangeNotSatisfiable,
        417 => Status::ExpectationFailed,
        418 => Status::ImATeapot,
        421 => Status::MisdirectedRequest,
        422 => Status::UnprocessableEntity,
        423 => Status::Locked,
        424 => Status::FailedDependency,
        // 425 => Status::TooEarly,
        426 => Status::UpgradeRequired,
        428 => Status::PreconditionRequired,
        429 => Status::TooManyRequests,
        431 => Status::RequestHeaderFieldsTooLarge,
        451 => Status::UnavailableForLegalReasons,

        500 => Status::InternalServerError,
        501 => Status::NotImplemented,
        502 => Status::BadGateway,
        503 => Status::ServiceUnavailable,
        504 => Status::GatewayTimeout,
        505 => Status::HttpVersionNotSupported,
        506 => Status::VariantAlsoNegotiates,
        507 => Status::InsufficientStorage,
        508 => Status::LoopDetected,
        510 => Status::NotExtended,
        511 => Status::NetworkAuthenticationRequired,

        _ => Status::new(code)
    }
}

pub use ::log;
