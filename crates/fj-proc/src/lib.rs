mod expand;
mod parse;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Paren, Expr, ExprCall,
    ExprPath, FnArg, ItemFn, PathArguments, PathSegment, Stmt,
};

/// Define a function-based model.
///
/// The simplest model function takes no parameters and returns a hard-coded
/// `fj::Shape`.
///
/// ``` rust ignore
/// # use fj_proc::model;
/// use fj::{Circle, Sketch, Shape};
/// #[model]
/// fn model() -> Shape {
///     let circle = Circle::from_radius(10.0);
///     Sketch::from_circle(circle).into()
/// }
/// ```
///
/// For convenience, you can also return anything that could be converted into
/// a `fj::Shape` (e.g. a `fj::Sketch`).
///
/// ``` rust ignore
/// # use fj_proc::model;
/// use fj::{Circle, Sketch};
/// #[model]
/// fn model() -> Sketch {
///     let circle = Circle::from_radius(10.0);
///     Sketch::from_circle(circle)
/// }
/// ```
///
/// The return type is checked at compile time. That means something like this
/// won't work because `()` can't be converted into a `fj::Shape`.
///
/// ``` rust ignore
/// # use fj_proc::model;
/// #[model]
/// fn model() { todo!() }
/// ```
///
/// The model function's arguments can be anything that implement
/// [`std::str::FromStr`].
///
/// ``` rust ignore
/// # use fj_proc::model;
/// #[model]
/// fn cylinder(height: f64, label: String, is_horizontal: bool) -> fj::Shape { todo!() }
/// ```
///
/// Constraints and default values can be added to an argument using the
/// `#[param]` attribute.
///
/// ``` rust ignore
/// use fj::syntax::*;
///
/// #[fj::model]
/// pub fn spacer(
///     #[param(default = 1.0, min = inner * 1.01)] outer: f64,
///     #[param(default = 0.5, max = outer * 0.99)] inner: f64,
///     #[param(default = 1.0)] height: f64,
/// ) -> fj::Shape {
///     let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
///     let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));
///
///     let footprint = outer_edge.difference(&inner_edge);
///     let spacer = footprint.sweep([0., 0., height]);
///
///     spacer.into()
/// }
/// ```
///
/// For more complex situations, model functions are allowed to return any
/// error type that converts into a model error.
///
/// ``` rust ignore
/// #[fj::model]
/// pub fn model() -> Result<fj::Shape, std::env::VarError> {
///     let home_dir = std::env::var("HOME")?;
///
///     todo!("Do something with {home_dir}")
/// }
///
/// fn assert_convertible(e: std::env::VarError) -> fj::models::Error { e.into() }
/// ```
#[proc_macro_attribute]
pub fn model(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemFn);

    match parse::parse(&item) {
        Ok(init) => {
            let mut item = without_param_attrs(item);

            // Yes, all of this is to add `fj::abi::initialize_panic_handling();` to the top of the function.
            item.block.stmts.insert(
                0,
                Stmt::Semi(
                    Expr::Call(ExprCall {
                        attrs: vec![],
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments: {
                                    let mut segments = Punctuated::new();

                                    segments.push(PathSegment {
                                        ident: Ident::new(
                                            "fj",
                                            Span::call_site(),
                                        ),
                                        arguments: PathArguments::None,
                                    });

                                    segments.push(PathSegment {
                                        ident: Ident::new(
                                            "abi",
                                            Span::call_site(),
                                        ),
                                        arguments: PathArguments::None,
                                    });

                                    segments.push(PathSegment {
                                        ident: Ident::new(
                                            "initialize_panic_handling",
                                            Span::call_site(),
                                        ),
                                        arguments: PathArguments::None,
                                    });

                                    segments
                                },
                            },
                        })),
                        paren_token: Paren {
                            span: Span::call_site(),
                        },
                        args: Punctuated::new(),
                    }),
                    syn::token::Semi::default(),
                ),
            );

            let tokens = quote::quote! {
                #item
                #init

            };

            eprintln!("TOKENS: {}", tokens);

            tokens.into()
        }
        Err(e) => e.into_compile_error().into(),
    }
}

/// Strip out any of our `#[param(...)]` attributes so the item will compile.
fn without_param_attrs(mut item: ItemFn) -> ItemFn {
    for input in &mut item.sig.inputs {
        let attrs = match input {
            FnArg::Receiver(r) => &mut r.attrs,
            FnArg::Typed(t) => &mut t.attrs,
        };
        attrs.retain(|attr| !attr.path.is_ident("param"));
    }

    item
}
