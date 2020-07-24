#[macro_use]
extern crate typed_builder;

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



#[cfg(test)]
mod tests {
    // Test with 3 arguments.
    labeled!(name_3({foo: usize, bar: usize, sna: String}) {
        println!("name_3 (foo = {foo}, bar = {bar}, sna = {sna})",
            foo = foo,
            bar = bar,
            sna = sna);
    });

    // Test with default argument
    labeled!(name_default({foo: usize, bar: usize, sna: String = "default".to_string()}) {
        println!("name_default (foo = {foo}, bar = {bar}, sna = {sna})",
            foo = foo,
            bar = bar,
            sna = sna);
    });

    // Test with 0 arguments.
    labeled!(name_0({}) {
        println!("name_0 ()");
    });

    #[test]
    fn it_works() {
        name_3::labeled()
            .sna("My name is sna".to_string())
            .bar(1000)
            .foo(0)
            .build()
            .call();
        name_default::labeled()
            .bar(1000)
            .foo(0)
            .build()
            .call();
        name_0::labeled().build().call();
    }
}
