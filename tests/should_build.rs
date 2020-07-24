#[macro_use]
extern crate named_args;

#[macro_use]
extern crate typed_builder;

// Test with 3 arguments.
labeled!(fun_3({foo: usize, bar: usize, sna: String}) {
    println!("fun_3 (foo = {foo}, bar = {bar}, sna = {sna})",
        foo = foo,
        bar = bar,
        sna = sna);
});

// Test with default argument
labeled!(fun_default({foo: usize, bar: usize, sna: String = "default".to_string()}) {
    println!("fun_default (foo = {foo}, bar = {bar}, sna = {sna})",
        foo = foo,
        bar = bar,
        sna = sna);
});

// Test with 0 arguments.
labeled!(fun_0({}) {
    println!("fun_0 ()");
});


#[test]
fn it_works() {
    fun_3::labeled()
        .sna("My name is sna".to_string())
        .bar(1000)
        .foo(0)
        .build()
        .call();
    fun_default::labeled()
        .bar(1000)
        .foo(0)
        .build()
        .call();
    fun_0::labeled().build().call();
}
