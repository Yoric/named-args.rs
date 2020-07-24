#[macro_use]
extern crate typed_builder;
/*
labeled!(name({foo: usize, bar: usize}) -> () {
    prusizeln!("called {foo}{bar}",
        foo = foo,
        bar = bar);
});
// FIXME: Or perhaps
#[labelled_args]
fn name({foo: usize, bar: usize}) -> () {
    // ...
}
*/
// Compiles to

// FIXME: handle 0 args, 1 arg.
macro_rules! labeled {
    ($name:ident ({ $($args:ident : $types:ty),* }) $block: block ) => {
        pub mod $name {
            // A convoluted way to transform a type T into ()
            pub trait HasUnit<T> {
                type Unit;
            }
            pub struct IHaveUnit<T> {
                phantom: std::marker::PhantomData<T>
            }
            impl<T> HasUnit<T> for IHaveUnit<T> {
                type Unit = ();
            }
            pub fn build() -> ArgsBuilder<( $( <IHaveUnit<$types> as HasUnit<$types>>::Unit ),* )> {
                Args::builder()
            }
            #[derive(TypedBuilder)]
            pub struct Args {
                $($args : $types),*
            }
            impl Args {
                pub fn call(self) {
                    super::$name($(self.$args),*)
                }
            }
        }
        fn $name($($args: $types),*) {
            $block
        }
    };
}



#[cfg(test)]
mod tests {
    labeled!(name2({foo: usize, bar: usize, sna: String}) {
        println!("called {foo}{bar}{sna}",
            foo = foo,
            bar = bar,
            sna = sna);
    });

    #[test]
    fn it_works() {
        name2::build()
            .sna("My name is sna".to_string())
            .bar(1000)
            .foo(0)
            .build()
            .call();
    }
}
