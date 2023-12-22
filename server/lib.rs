pub mod html;

#[macro_export]
macro_rules! inc { () => (use $crate::prelude::*; use $crate::*; ) }

pub mod prelude {
    pub use crate::html::assets::HtmlHandlebarsInjector;
    pub use crate::html::macros::{err, res};
    pub use ::rocket::{
        async_main, async_run, async_test, async_trait,
        build,
        catch, catchers,
        delete,
        export,
        get,
        head,
        launch,
        main,
        options,
        patch, post, put,
        routes, route,
        uri,
    };
    #[macro_export(local_inner_macros)]
    macro_rules! include_hbs {
        ($file:expr $(,)?) => (
            ::core::include_str!($file)
            .handlebars("debug", $crate::html::SERVER)
        )
    }
    pub use include_hbs;
}
