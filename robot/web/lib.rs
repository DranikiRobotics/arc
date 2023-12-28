#[macro_export]
macro_rules! include_hbs {
    ($file:expr $(,)?) => (
        ::core::include_str!($file)
        .handlebars("debug", $crate::html::SERVER)
    )
}

pub async fn main() {

}
