//! Early prototype of a macro designed to provide labeled arguments (aka named arguments
//! aka positional arguments) to your Rust code.
//!
//! Instead of:
//! ```
//! fn do_rect(width: f64, height: f64) {
//!     // ...
//! }
//! const WIDTH : f64 = 10.0;
//! const HEIGHT: f64 = 30.0;
//! do_rect(WIDTH, HEIGHT); // Oops, confused width and height.
//! ```
//!
//! Let's do this:
//! ```
//! #[macro_use]
//! extern crate named_args;
//! #[macro_use]
//! extern crate typed_builder;
//!
//! labeled!(do_rect({width: f64, height: f64}) {
//!   // ...
//! });
//!
//! fn main() {
//!   const WIDTH : f64 = 10.0;
//!   const HEIGHT: f64 = 30.0;
//!   do_rect::labeled()
//!       .width(WIDTH)   // No confusion here. Order doesn't matter.
//!       .height(HEIGHT) // No confusion here. Order doesn't matter.
//!       .build()
//!       .call();
//! }
//! ```
//!
//! We can even add optional arguments:
//!
//! ```
//! #[macro_use]
//! extern crate named_args;
//! #[macro_use]
//! extern crate typed_builder;
//!
//! labeled!(do_rect({width: f64, height: f64, opacity : f64 = 1.0}) {
//!   // ...
//! });
//!
//! fn main() {
//!   const WIDTH : f64 = 10.0;
//!   const HEIGHT: f64 = 30.0;
//!   // With the optional argument:
//!   do_rect::labeled()
//!       .width(WIDTH)   // No confusion here. Order doesn't matter.
//!       .opacity(0.)    // Order still doesn't matter.
//!       .height(HEIGHT) // No confusion here. Order doesn't matter.
//!       .build()
//!       .call();
//!
//!   // Or without:
//!   do_rect::labeled()
//!       .width(WIDTH)   // No confusion here. Order doesn't matter.
//!       .height(HEIGHT) // No confusion here. Order doesn't matter.
//!       .build()
//!       .call();
//!}
//! ```
//!
//! Omitting a required field will cause a type error:
//! ```compile_fail
//! #[macro_use]
//! extern crate named_args;
//! #[macro_use]
//! extern crate typed_builder;
//!
//! labeled!(do_rect({width: f64, height: f64, opacity : f64 = 1.0}) {
//!   // ...
//! });
//!
//! fn main() {
//!   const WIDTH : f64 = 10.0;
//!   const HEIGHT: f64 = 30.0;
//!
//!   do_rect::labeled()
//!       .width(WIDTH)   // No confusion here. Order doesn't matter.
//!       .build()        // <-- Build error here.
//!       .call();
//!}
//!```
#[macro_export]
macro_rules! labeled {
    ($name:ident ({ $($args:ident : $types:ty $( = $default:expr)? ),* $(,)? }) $(-> $ret:ty)? $block: block ) => {
        pub mod $name {

            /// A convoluted way to transform a type T into ()
            pub trait HasUnit<T> {
                type Unit;
            }
            pub struct IHaveUnit<T> {
                phantom: std::marker::PhantomData<T>
            }
            impl<T> HasUnit<T> for IHaveUnit<T> {
                type Unit = ();
            }

            /// Start building the call.
            pub fn labeled() -> ArgsBuilder<( $( <IHaveUnit<$types> as HasUnit<$types>>::Unit ),* )> {
                Args::builder()
            }

            /// A data structure containing the args.
            ///
            /// We use `TypedBuilder` to generate the FSM that guarantees
            /// statically that we're typing properly.
            #[derive(TypedBuilder)]
            pub struct Args {
                $(
                    $( #[builder(default=$default)] )?
                    $args : $types
                ),*
            }
            impl Args {
                pub fn call(self) $(-> $ret:ty)? {
                    super::$name($(self.$args),*)
                }
            }
        }
        fn $name($($args: $types),*) $(-> $ret:ty)? {
            $block
        }
    };
}
