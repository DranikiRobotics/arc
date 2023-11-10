/// A type variant, used in the `#[mass_impl(...)]` macro.
/// 
/// Input generally looks like this:
/// 
/// ```rust,ignore,no_run
/// $NAME = @RM SomeStruct
/// ```
/// 
/// Which essentially means:
/// - `$NAME` is the alias for `SomeStruct`
/// - `@R` means that `SomeStruct` can be passed by reference
/// - `@RM` means that `SomeStruct` can be passed by mutable reference
/// 
/// So it will create 3 bodies for the macro:
/// - `impl SomeTrait for SomeStruct`
/// - `impl SomeTrait for &SomeStruct`
/// - `impl SomeTrait for &mut SomeStruct`
pub struct TypeVariant {
    /// The alias for the type (e.g. `$NAME`)
    pub alias: syn::Ident,
    /// The type (e.g. `SomeStruct`)
    pub ty: syn::Ident,
    /// Whether or not the type can be passed by value
    pub allow_owned: bool,
    /// Whether or not the type can be passed by reference
    pub allow_ref: bool,
    /// Whether or not the type can be passed by mutable reference
    pub allow_mut: bool,
}

impl TypeVariant {
    /// Returns the number of passthroughs for this type variant.
    pub fn number_of_passthroughs(&self) -> i32 {
        let mut i =if self.allow_owned { 1 } else { 0 };
        if self.allow_ref { i += 1 };
        if self.allow_mut { i += 1 };
        i
    }
}

impl std::fmt::Debug for TypeVariant {
    /// Formats a `TypeVariant`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeVariant")
            .field("alias", &self.alias)
            .field("ty", &self.ty)
            .field("allow_owned", &self.allow_owned)
            .field("allow_ref", &self.allow_ref)
            .field("allow_mut", &self.allow_mut)
            .finish()
    }
}

impl syn::parse::Parse for TypeVariant {
    /// Turns this:
    /// 
    /// ```rust,ignore,no_run
    /// $NAME = @RM SomeStruct,
    /// ```
    /// 
    /// Into this:
    /// 
    /// ```rust,ignore,no_run
    /// TypeVariant {
    ///    alias: "NAME"
    ///    ty: SomeStruct,
    ///    allow_owned: true,
    ///    allow_ref: true,
    ///    allow_mut: true,
    /// }
    /// ```
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::*;
        let _ = input.parse::<Token![$]>()?;
        let alias = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let mut allow_owned = false;
        let mut allow_ref = false;
        let mut allow_mut = false;
        loop {
            if input.peek(Token![@]) {
                let _ = input.parse::<Token![@]>()?;
                let modifier = input.parse::<Ident>()?;
                let modifier = modifier.to_string();
                if modifier.contains('R') {
                    allow_ref = true;
                }
                if modifier.contains('M') {
                    allow_mut = true;
                }
                if modifier.contains('O') {
                    allow_owned = true;
                }
            } else {
                break;
            }
        };
        let ty = input.parse::<Ident>()?;
        Ok(TypeVariant {
            alias,
            ty,
            allow_owned,
            allow_ref,
            allow_mut,
        })
    }
}
