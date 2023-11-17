use crate::TokenStream;
use quote::quote;

/// A macro for changing the Signature of a function.
/// depending on feature flags.
pub fn func_mod<T: Into<TokenStream>>(cfg: T, input: T) -> TokenStream {
    let input = input.into();
    let f = match syn::parse2::<syn::ItemFn>(input) {
        Err(e) => return e.to_compile_error().into(),
        Ok(f) => f,
    };

    // Basic info
    let name = &f.sig.ident;
    let ret = &f.sig.output;
    let inputs = &f.sig.inputs;
    let body = &f.block;
    let attrs = &f.attrs;
    let vis = &f.vis;

    // Configuration
    let cfg = match Config::parse(cfg) {
        Err(e) => return e.to_compile_error().into(),
        Ok(cfg) => cfg,
    };

    // Signatures
    let abi = &f.sig.abi;
    let asyncness = &f.sig.asyncness;
    let constness = &f.sig.constness;
    let unsafety = &f.sig.unsafety;
    let generics = &f.sig.generics;
    let where_clause = &f.sig.generics.where_clause;

    let base_sig = quote!(
        #constness #asyncness #unsafety #abi fn #name #generics(#inputs) #ret #where_clause {
            #body
        }
    );

    let mut sig = quote!(#base_sig);

    if let Some(c) = cfg.constness {
        let c_token = c.token;
        let c_precode = c.precode;
        if let Some(a) = cfg.asyncness {
            let a_token = a.token;
            let a_precode = a.precode;
            if let Some(u) = cfg.unsafety {
                let u_token = u.token;
                let u_precode = u.precode;
                sig = quote!(
                    #(#attrs)*
                    #[cfg(all(#c_precode, #a_precode, #u_precode))]
                    #vis #c_token #a_token #u_token #sig

                    #(#attrs)*
                    #[cfg(all(#c_precode, #a_precode, not(#u_precode)))]
                    #vis #c_token #a_token #sig

                    #(#attrs)*
                    #[cfg(all(#c_precode, not(#a_precode), #u_precode))]
                    #vis #c_token #u_token #sig

                    #(#attrs)*
                    #[cfg(all(#c_precode, not(#a_precode), not(#u_precode)))]
                    #vis #c_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), #a_precode, #u_precode))]
                    #vis #a_token #u_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), #a_precode, not(#u_precode)))]
                    #vis #a_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), not(#a_precode), #u_precode))]
                    #vis #u_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), not(#a_precode), not(#u_precode)))]
                    #vis #sig
                );
            } else {
                sig = quote!(
                    #(#attrs)*
                    #[cfg(all(#c_precode, #a_precode))]
                    #vis #c_token #a_token #sig

                    #(#attrs)*
                    #[cfg(all(#c_precode, not(#a_precode)))]
                    #vis #c_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), #a_precode))]
                    #vis #a_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), not(#a_precode)))]
                    #vis #sig
                );
            }
        } else {
            if let Some(u) = cfg.unsafety {
                let u_token = u.token;
                let u_precode = u.precode;
                sig = quote!(
                    #(#attrs)*
                    #[cfg(all(#c_precode, #u_precode))]
                    #vis #c_token #u_token #sig

                    #(#attrs)*
                    #[cfg(all(#c_precode, not(#u_precode)))]
                    #vis #c_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), #u_precode))]
                    #vis #u_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#c_precode), not(#u_precode)))]
                    #vis #sig
                );
            } else {
                sig = quote!(
                    #(#attrs)*
                    #[cfg(#c_precode)]
                    #vis #c_token #sig

                    #(#attrs)*
                    #[cfg(not(#c_precode))]
                    #vis #sig
                );
            }
        }
    } else {
        if let Some(a) = cfg.asyncness {
            let a_token = a.token;
            let a_precode = a.precode;
            if let Some(u) = cfg.unsafety {
                let u_token = u.token;
                let u_precode = u.precode;
                sig = quote!(
                    #(#attrs)*
                    #[cfg(all(#a_precode, #u_precode))]
                    #vis #a_token #u_token #sig

                    #(#attrs)*
                    #[cfg(all(#a_precode, not(#u_precode)))]
                    #vis #a_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#a_precode), #u_precode))]
                    #vis #u_token #sig

                    #(#attrs)*
                    #[cfg(all(not(#a_precode), not(#u_precode)))]
                    #vis #sig
                );
            } else {
                sig = quote!(
                    #(#attrs)*
                    #[cfg(#a_precode)]
                    #vis #a_token #sig

                    #(#attrs)*
                    #[cfg(not(#a_precode))]
                    #vis #sig
                );
            }
        } else {
            if let Some(u) = cfg.unsafety {
                let u_token = u.token;
                let u_precode = u.precode;
                sig = quote!(
                    #(#attrs)*
                    #[cfg(#u_precode)]
                    #vis #u_token #sig

                    #(#attrs)*
                    #[cfg(not(#u_precode))]
                    #vis #sig
                );
            } else {
                sig = quote!(
                    #(#attrs)*
                    #vis #sig
                );
            }
        }
    }

    sig.into()
}

#[derive(Default)]
struct Config {
    constness: Option<ConfigItem<syn::Token![const]>>,
    asyncness: Option<ConfigItem<syn::Token![async]>>,
    unsafety: Option<ConfigItem<syn::Token![unsafe]>>,
}

struct ConfigItem<Token: syn::parse::Parse> {
    token: Token,
    precode: syn::Meta,
}

impl<Token: syn::parse::Parse> syn::parse::Parse for ConfigItem<Token> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let token = input.parse::<Token>()?;
        input.parse::<syn::Token![=>]>()?;
        let precode = input.parse::<syn::Meta>()?;
        Ok(ConfigItem { token, precode })
    }
}

impl Config {
    /// Parses the configuration.
    ///
    /// # Example
    ///
    /// This example shows how if the feature `"unstable"` is enabled,
    /// the function will be marked as `const`.
    ///
    /// ```rust,no_run,ignore
    /// #[inline]
    /// #[func_mod(
    ///     const => #[cfg(feature = "unstable")]
    /// )]
    /// fn deg_to_rad(deg: f64) -> f64 {
    ///     deg * 0.017
    /// }
    /// ```
    fn parse<T: Into<TokenStream>>(cfg: T) -> syn::Result<Self> {
        let cfg: TokenStream = cfg.into();
        syn::parse2::<Config>(cfg)
    }
}

impl syn::parse::Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cfg = Config::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![const]) {
                cfg.constness = Some(input.parse::<ConfigItem<syn::Token![const]>>()?);
            }
            if lookahead.peek(syn::Token![async]) {
                cfg.asyncness = Some(input.parse::<ConfigItem<syn::Token![async]>>()?);
            }
            if lookahead.peek(syn::Token![unsafe]) {
                cfg.unsafety = Some(input.parse::<ConfigItem<syn::Token![unsafe]>>()?);
            }
        }
        Ok(cfg)
    }
}
