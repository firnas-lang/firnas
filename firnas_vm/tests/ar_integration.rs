#![cfg(feature = "ar")]

use firnas_compiler::compiler::*;
use firnas_ext;
use firnas_vm::virtual_machine::*;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn evaluate(code: &str, extensions: firnas_ext::Extensions) -> Result<Vec<String>, String> {
    let func_or_err = Compiler::compile(String::from(code), extensions);

    match func_or_err {
        Ok(func) => {
            let mut vm = VirtualMachine::default();
            let res = vm.interpret(func);
            match res {
                Ok(()) => Ok(vm.get_output()),
                Err(VmError::Runtime(err)) => Err(err),
            }
        }
        Err(Error::Lexical(err)) => Err(err.what),
        Err(Error::Parse(err)) => Err(err.what),
        Err(Error::Semantic(err)) => Err(err.what),
        Err(Error::Internal(err)) => Err(err),
    }
}

fn check_output(code: &str, extensions: firnas_ext::Extensions, expected_output: &[String]) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => assert_eq!(output, expected_output),
        Err(err) => panic!("{}", err),
    }
}

fn check_output_default(code: &str, expected_output: &[String]) {
    check_output(code, firnas_ext::Extensions::default(), expected_output);
}

fn check_output_lists(code: &str, expected_output: &[String]) {
    check_output(
        code,
        firnas_ext::Extensions {
            lists: true,
            ..Default::default()
        },
        expected_output,
    );
}

fn check_error(code: &str, extensions: firnas_ext::Extensions, f: &dyn Fn(&str) -> ()) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => panic!("{:?}", output),
        Err(err) => f(&err),
    }
}

fn check_error_default(code: &str, f: &dyn Fn(&str) -> ()) {
    check_error(code, firnas_ext::Extensions::default(), f);
}

#[test]
fn it_should_print_var_value() {
    let code = "دع س = ٢؛ اطبع س؛";
    check_output_default(code, &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_in_scope() {
    let code = "{ دع س = ٢؛ اطبع س؛ }";
    check_output_default(code, &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_after_mutation() {
    check_output_default(
        r#"
دع س = ٢؛
دع ص = ٣؛
اطبع س * ص + ٤؛
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_print_var_value_after_mutation_in_scope() {
    check_output_default(
        r#"
{
    دع س = ٢؛
    دع ص = ٣؛
    اطبع س * ص + ٤؛
}
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_return_inf_when_dividing_by_zero() {
    let code = r"اطبع ٢ \ ٠؛";
    check_output_default(code, &vec_of_strings!["inf"]);
}

#[test]
fn it_should_set_items_global() {
    check_output_default(
        r#"
دع فطور = "تمر"؛
دع مشروب = "لبن"؛
فطور = فطور + " مع " + مشروب؛
اطبع فطور؛
"#,
        &vec_of_strings!["تمر مع لبن"],
    );
}

#[test]
fn it_should_set_items_in_scope() {
    check_output_default(
        r#"
{
    دع فطور = "تمر"؛
    دع مشروب = "لبن"؛
    فطور = "تمر مع لبن"؛
    اطبع فطور؛
}
"#,
        &vec_of_strings!["تمر مع لبن"],
    );
}

#[test]
fn it_should_fail_on_read_from_own_initializer() {
    check_error_default(
        r#"
{
    دع س = "خارجي"؛
    {
        دع س = س؛
    }
}
"#,
        &|err: &str| assert!(err.starts_with("Cannot read local variable in its own initializer.")),
    )
}

#[test]
fn test_if_stmt() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛

لو(س)
{
    اطبع س؛
}

لو(ص)
{
    اطبع ص؛
}
"#,
        &vec_of_strings!["1"],
    );
}

#[test]
fn test_if_then_else_1() {
    check_output_default(
        r#"
دع س = ٠؛
لو(س) {
    اطبع "مرحباً"؛
} اخر {
    اطبع "ودعاً"؛
}
"#,
        &vec_of_strings!["ودعاً"],
    );
}

#[test]
fn test_if_then_else_2() {
    check_output_default(
        r#"
دع س = ١؛
لو(س) {
    اطبع "مرحباً"؛
} اخر {
    اطبع "ودعاً"؛
}
"#,
        &vec_of_strings!["مرحباً"],
    );
}

#[test]
fn test_print_locals() {
    check_output_default(
        r#"
{
    دع س = ٠؛
    دع ص = ١؛
    اطبع س؛
    اطبع ص؛
}
"#,
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_print_globals() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛
اطبع س؛
اطبع ص؛
"#,
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_and_1() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = خطا؛
لو(س و ص) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_2() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = خطا؛
لو(ص و س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_3() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = صحيح؛
لو(ص و س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_1() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = صحيح؛
لو(س او ص) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_2() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = صحيح؛
لو(ص او س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_3() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = خطا؛
لو(ص او س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_while() {
    check_output_default(
        r#"
{
    دع س = ٠؛
    دع مجموع = ٠؛
    طالما (س < ١٠٠) {
        س = س + ١؛
        مجموع = مجموع + س؛
    }
    اطبع مجموع؛
}
"#,
        &vec_of_strings!["5050"],
    );
}

#[test]
fn test_for() {
    fn fact(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        return n * fact(n - 1);
    }

    check_output_default(
        r#"
{
    دع عاملي = ١؛
    من (دع س = ١؛ س <= ١٠؛ س = س + ١) {
        عاملي = عاملي * س؛
    }
    اطبع عاملي؛
}
"#,
        &vec_of_strings![format!("{}", fact(10))],
    );
}

// #[test]
// fn test_functions_1() {
//     check_output_default(
//         r#"
// fun areWeHavingItYet() {
//     print "Yes we are!";
// }

// print areWeHavingItYet;
// "#,
//         &vec_of_strings!["<fn 'areWeHavingItYet'>"],
//     )
// }

// #[test]
// fn test_functions_2() {
//     check_output_default(
//         r#"
// fun f(x, y) {
//     print x + y;
// }

// print f;
// "#,
//         &vec_of_strings!["<fn 'f'>"],
//     )
// }

// #[test]
// fn test_functions_3() {
//     check_output_default(
//         r#"
// fun f(x, y) {
//     return x + y;
// }

// print f;
// "#,
//         &vec_of_strings!["<fn 'f'>"],
//     )
// }

// #[test]
// fn test_functions_4() {
//     check_output_default(
//         r#"
// fun f() {
//     return;
// }

// print f();
// "#,
//         &vec_of_strings!["nil"],
//     )
// }

// #[test]
// fn test_functions_5() {
//     check_error_default("return 42;", &|err: &str| {
//         assert_eq!(err, "Cannot return from top-level code.")
//     })
// }

// #[test]
// fn test_functions_6() {
//     check_output_default(
//         r#"
// fun f(x, y) {
//     return x + y;
// }

// print f(1,2);
// "#,
//         &vec_of_strings!["3"],
//     );
// }

// #[test]
// fn test_functions_7() {
//     check_output_default(
//         r#"
// fun g(x) {
//     return 2 * x;
// }

// fun f(x, y) {
//     return g(x) + y;
// }

// print f(1,2);
// "#,
//         &vec_of_strings!["4"],
//     );
// }

// #[test]
// fn test_functions_8() {
//     check_output_default(
//         r#"
// var x = 2;
// fun f(x) {
//     print 2 * x;
// }

// f(x);
// print x;
// "#,
//         &vec_of_strings!["4", "2"],
//     );
// }

// #[test]
// fn test_functions_9() {
//     fn fact(n: i32) -> i32 {
//         if n <= 1 {
//             return 1;
//         }
//         return n * fact(n - 1);
//     }

//     check_output_default(
//         r#"
// fun fact(n) {
//     if (n <= 1) { return 1; }
//     return n * fact(n - 1);
// }

// print fact(10);
// "#,
//         &vec_of_strings![format!("{}", fact(10))],
//     );
// }

// #[test]
// fn test_functions_10() {
//     check_output_default(
//         r#"
// fun isEven(n) {
//     if (n = 0) { return true; }
//     return isOdd(n - 1);
// }

// fun isOdd(n) {
//     if (n = 1) { return true; }
//     return isEven(n - 1);
// }

// print isEven(10);
// "#,
//         &vec_of_strings!["true"],
//     );
// }

// #[test]
// fn test_native_functions() {
//     let res = evaluate(
//         r#"
// fun fib(n) {
//     if (n < 2) return n;
//     return fib(n - 2) + fib(n - 1);
// }

// var start = clock();
// print fib(5);
// print clock() - start;
// print 42;
// "#,
//         firnas_ext::Extensions::default(),
//     );

//     match res {
//         Ok(output) => {
//             assert_eq!(output.len(), 3);
//             assert_eq!(output[0], "5");
//             assert_eq!(output[2], "42");
//         }
//         Err(err) => {
//             panic!("{:?}", err);
//         }
//     }
// }

// #[test]
// fn test_get_upval_on_stack() {
//     check_output_default(
//         r#"
// fun outer() {
//     var x = "outside";
//     fun inner() {
//         print x;
//     }
//     inner();
// }
// outer();
// "#,
//         &vec_of_strings!["outside"],
//     );
// }

// #[test]
// fn test_set_upval_on_stack() {
//     check_output_default(
//         r#"
// fun outer() {
//     var x = "before";
//     fun inner() {
//         x = "assigned";
//     }
//     inner();
//     print x;
// }
// outer();
// "#,
//         &vec_of_strings!["assigned"],
//     );
// }

// #[test]
// fn test_closing_upvals_after_return() {
//     check_output_default(
//         r#"
// fun outer() {
//     var x = "outside";
//     fun inner() {
//         print x;
//     }

//     return inner;
// }

// var closure = outer();
// closure();
// "#,
//         &vec_of_strings!["outside"],
//     );
// }

// #[test]
// fn test_closing_upvals_after_scope() {
//     check_output_default(
//         r#"
// var closure;
// {
//     var x = "outside";
//     fun inner() {
//         print x;
//     }

//     closure = inner;
// }

// closure();
// "#,
//         &vec_of_strings!["outside"],
//     );
// }

// #[test]
// fn test_classes_1() {
//     check_output_default(
//         r#"
// class Brioche {}
// print Brioche;
// "#,
//         &vec_of_strings!["<class 'Brioche'>"],
//     );
// }

// #[test]
// fn test_classes_instances_1() {
//     check_output_default(
//         r#"
// class Brioche {}
// var instance = Brioche();
// print instance;
// "#,
//         &vec_of_strings!["<Brioche instance>"],
//     );
// }

// #[test]
// fn test_setattr_1() {
//     check_output_default(
//         r#"
// class Foo {}
// var foo = Foo();
// foo.attr = 42;
// print foo.attr;
// "#,
//         &vec_of_strings!["42"],
//     );
// }

// #[test]
// fn test_setattr_2() {
//     check_output_default(
//         r#"
// class Toast {}
// var toast = Toast();
// print toast.jam = "grape";
// "#,
//         &vec_of_strings!["grape"],
//     );
// }

// #[test]
// fn test_setattr_3() {
//     check_output_default(
//         r#"
// class Pair {}
// var pair = Pair();
// pair.first = 1;
// pair.second = 2;
// print pair.first + pair.second;
// "#,
//         &vec_of_strings!["3"],
//     );
// }

// #[test]
// fn test_bound_methods_1() {
//     check_output_default(
//         r#"
// class Foo {
//     bar() {
//         return 42;
//     }
// }

// var foo = Foo();
// print foo.bar;
// "#,
//         &vec_of_strings!["<bound method of Foo instance>"],
//     );
// }

// #[test]
// fn test_calling_bound_methods_no_this() {
//     check_output_default(
//         r#"
// class Scone {
//     topping(first, second) {
//         print "scone with " + first + " and " + second;
//     }
// }

// var scone = Scone();
// scone.topping("berries", "cream");
// "#,
//         &vec_of_strings!["scone with berries and cream"],
//     );
// }

// #[test]
// fn test_calling_bound_methods_with_this_1() {
//     check_output_default(
//         r#"
// class Nested {
//     method() {
//         print this;
//     }
// }

// Nested().method();
// "#,
//         &vec_of_strings!["<Nested instance>"],
//     );
// }

// #[test]
// fn test_calling_bound_methods_with_this_2() {
//     check_output_default(
//         r#"
// class Nested {
//     method() {
//         fun function() {
//             print this;
//         }

//         function();
//     }
// }

// Nested().method();
// "#,
//         &vec_of_strings!["<Nested instance>"],
//     );
// }

// #[test]
// fn test_multiple_method_definitions() {
//     check_output_default(
//         r#"
// class Brunch {
//     bacon() {}
//     eggs() {}
// }
// print Brunch().bacon();
// "#,
//         &vec_of_strings!["nil"],
//     );
// }

// #[test]
// fn test_init_1() {
//     check_output_default(
//         r#"
// class Brunch {
//     init(x) {this.x = x;}
//     eggs(y) {return this.x + y;}
// }
// print Brunch(2).eggs(3);
// "#,
//         &vec_of_strings!["5"],
//     );
// }

// #[test]
// fn test_invoking_fields() {
//     check_output_default(
//         r#"
// class Oops {
//     init() {
//         fun f() {
//             print "not a method";
//         }

//         this.field = f;
//     }
// }

// var oops = Oops();
// oops.field();
// "#,
//         &vec_of_strings!["not a method"],
//     );
// }

// #[test]
// fn test_inheritance_1() {
//     check_output_default(
//         r#"
// class A {
//     f() {
//         return "cat";
//     }
// }
// class B < A {}
// var b = B();
// print b.f();
// "#,
//         &vec_of_strings!["cat"],
//     );
// }

// #[test]
// fn test_inheritance_2() {
//     check_output_default(
//         r#"
// class A {
//     f() {
//         return "cat";
//     }
// }
// class B < A {}
// class C < B {}
// var c = C();
// print c.f();
// "#,
//         &vec_of_strings!["cat"],
//     );
// }

// #[test]
// fn test_inheritance_3() {
//     check_output_default(
//         r#"
// class A {
//     f() {
//         return this.attr;
//     }
// }
// class B < A {
//     init(attr) {
//         this.attr = attr;
//     }
// }

// var b = B(42);
// print b.f();
// "#,
//         &vec_of_strings!["42"],
//     );
// }

// #[test]
// fn test_inheritance_4() {
//     check_output_default(
//         r#"
// class A {
//     f() {
//         return this.attr;
//     }
// }
// class B < A {
// }
// var b = B();
// b.attr = 42;
// print b.f();
// "#,
//         &vec_of_strings!["42"],
//     );
// }

// #[test]
// fn test_inheriting_non_class() {
//     check_error_default(
//         r#"
// var NotClass = "So not a class";
// class OhNo < NotClass {}
// "#,
//         &|err: &str| assert!(err.starts_with("Superclass must be a class, found String at lineno=")),
//     )
// }

// #[test]
// fn test_super_1() {
//     check_output_default(
//         r#"
// class A {
//     method() {
//         print "A method";
//     }
// }

// class B < A {
//     method() {
//         print "B method";
//     }

//     test() {
//         super.method();
//     }
// }

// class C < B {}

// C().test();
// "#,
//         &vec_of_strings!["A method"],
//     )
// }

// #[test]
// fn test_super_2() {
//     check_output_default(
//         r#"
// class A {
//     method() {
//         print "A method";
//     }
// }

// class B < A {
//     method() {
//         print "B method";
//     }

//     test() {
//         var func = super.method;
//         func();
//     }
// }

// class C < B {}

// C().test();
// "#,
//         &vec_of_strings!["A method"],
//     )
// }

// #[test]
// fn test_super_3() {
//     check_output_default(
//         r#"
// class Doughnut {
//     cook() {
//         print "Dunk in the fryer.";
//         this.finish("sprinkles");
//     }

//     finish(ingredient) {
//         print "Finish with " + ingredient;
//     }
// }

// class Cruller < Doughnut {
//     finish(ingredient) {
//         // No sprinkles.
//         super.finish("icing");
//     }
// }

// Doughnut().cook();
// Cruller().cook();
// "#,
//         &vec_of_strings![
//             "Dunk in the fryer.",
//             "Finish with sprinkles",
//             "Dunk in the fryer.",
//             "Finish with icing"
//         ],
//     )
// }

// #[test]
// fn test_late_binding() {
//     check_output_default(
//         r#"
// fun a() { b(); }
// fun b() { print "hello world"; }

// a();
// "#,
//         &vec_of_strings!["hello world"],
//     )
// }

// #[test]
// fn test_list_building() {
//     check_output_lists("print([1,2,3]);", &vec_of_strings!["[1, 2, 3]"])
// }

// #[test]
// fn test_empty_list_building() {
//     check_output_lists("print([]);", &vec_of_strings!["[]"])
// }

// #[test]
// fn test_list_concat() {
//     check_output_lists(
//         "print([1,2,3] + [4,5,6]);",
//         &vec_of_strings!["[1, 2, 3, 4, 5, 6]"],
//     )
// }

// #[test]
// fn test_len() {
//     check_output_lists(
//         r#"
// print(len(""));
// print(len("cat"));
// print(len([]));
// print(len([1,2,3,4]));
// "#,
//         &vec_of_strings!["0", "3", "0", "4"],
//     )
// }

// #[test]
// fn test_for_each() {
//     check_output_lists(
//         r#"
// fun f(arg) { print arg; }
// forEach([1,2,3,4], f);
// "#,
//         &vec_of_strings!["1", "2", "3", "4"],
//     )
// }

// #[test]
// fn test_map() {
//     check_output_lists(
//         r#"
// fun f(arg) { return arg + 1; }
// print(map(f, [1,2,3,4]));
// "#,
//         &vec_of_strings!["[2, 3, 4, 5]"],
//     )
// }

// #[test]
// fn test_list_subscript() {
//     check_output_lists(
//         r#"
// var xs = [0,1];
// print(xs[0]);
// print(xs[1]);
// print(xs[-1]);
// print(xs[-2]);
// "#,
//         &vec_of_strings!["0", "1", "1", "0"],
//     )
// }

// #[test]
// fn test_list_setitem_1() {
//     check_output_lists(
//         r#"
// var xs = [0,1];
// xs[-1] = 42;
// print(xs);
// "#,
//         &vec_of_strings!["[0, 42]"],
//     )
// }

// #[test]
// fn test_list_setitem_2() {
//     check_output_lists(
//         r#"
// var xs = [[0,1]];
// xs[0][1] = 42;
// print(xs);
// "#,
//         &vec_of_strings!["[[0, 42]]"],
//     )
// }

// #[test]
// fn test_list_setitem_3() {
//     check_output_lists(
//         r#"
// class Foo {}
// var foo = Foo();
// foo.attr = [0];
// foo.attr[0] = 1337;
// print foo.attr;
// "#,
//         &vec_of_strings!["[1337]"],
//     )
// }
