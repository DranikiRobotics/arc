use crate::prelude::*;

/// Basic Handlebars support for HTML.
/// 
/// This trait is implemented for `&str` and `String`.
/// 
/// Note, this does **NOT** support the full specifcation of Handlebars.
/// 
/// It can only replace `{{__KEY__}}` with the value of `key`.
/// 
/// # Example
/// 
/// ```rust
/// # use arc_robot as robot;
/// const HTML: &str = "<h1>{{__TITLE__}}</h1>";
/// let html = HTML.handlebars("title", "Hello, World!");
/// assert_eq!(html, "<h1>Hello, World!</h1>");
/// ```
pub trait HtmlHandlebarsInjector<Output = Self> {
    /// Replaces `{{__KEY__}}` with the value of `key`.
    fn handlebars<K: AsRef<str>, V: AsRef<str>>(self, key: K, value: V) -> Output;
}

impl HtmlHandlebarsInjector<String> for &str {
    fn handlebars<K: AsRef<str>, V: AsRef<str>>(self, key: K, value: V) -> String {
        self.replace(&format!("{{{{__{}__}}}}", key.as_ref().to_uppercase()), value.as_ref())
    }
}

impl HtmlHandlebarsInjector for String {
    fn handlebars<K: AsRef<str>, V: AsRef<str>>(self, key: K, value: V) -> String {
        self.replace(&format!("{{{{__{}__}}}}", key.as_ref().to_uppercase()), value.as_ref())
    }
}

/// `style.css` file.
#[get("/style.css")]
pub fn style() -> crate::html::Response<&'static str> {
    res!(@CSS #200 include_str!("./style.css"))
}

/// `SourceCodePro-Bold.ttf` file.
#[get("/SourceCodePro-Bold.ttf")]
pub fn font() -> crate::html::Response<&'static [u8]> {
    res!(@TTF #200 include_bytes!("./SourceCodePro-Bold.ttf"))
}

/// `favicon.ico` file.
#[get("/favicon.ico?<s>")]
pub fn favicon(s: Option<u32>) -> crate::html::Response<&'static [u8]> {
    let s256 = include_bytes!("../../docs/logo256.ico");
    let s192 = include_bytes!("../../docs/logo192.ico");
    let s64 = include_bytes!("../../docs/logo64.ico");
    res!(@Icon #200 match s {
        Some(192) => s192,
        Some(64) => s64,
        _ => s256,
    })
}

/// `logo_white_a.png` file.
#[get("/logo_white_a.png")]
pub fn logo_white_a() -> crate::html::Response<&'static [u8]> {
    res!(@PNG #200 include_bytes!("../../docs/logo_white_a.png"))
}
