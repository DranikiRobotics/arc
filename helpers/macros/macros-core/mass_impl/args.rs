//! Argument parsing for the `mass_impl` macro.

pub use super::variants::TypeVariant;

/// The configuration for the `mass_impl` macro.
pub struct MassImplMacroConfig {
    /// The type variants to implement the trait for.
    pub type_variants: Vec<TypeVariant>,
}

impl std::fmt::Debug for MassImplMacroConfig {
    /// Formats a `TypeVariant`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MassImplMacroConfig")
            .field("type_variants", &self.type_variants)
            .field("number_of_passthroughs", &self.number_of_passthroughs())
            .finish()
    }
}

impl MassImplMacroConfig {
    /// Returns the number of passthroughs for this type variant.
    pub fn number_of_passthroughs(&self) -> i32 {
        let mut i = 1;
        for variant in &self.type_variants {
            i *= variant.number_of_passthroughs();
        }
        i
    }
}

// parses this:
// $SELF = @R @M Vec2D,
// $OTHER = @R @M Vec2D
impl syn::parse::Parse for MassImplMacroConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut type_variants = Vec::new();
        let vars =
            syn::punctuated::Punctuated::<TypeVariant, syn::Token![,]>::parse_terminated(input)?;
        for var in vars {
            type_variants.push(var);
        }
        Ok(MassImplMacroConfig { type_variants })
    }
}
