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
            let mut interp = VirtualMachine::default();
            let res = interp.interpret(func);
            match res {
                Ok(()) => Ok(interp.get_output()),
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
fn test_var_reading_1() {
    check_output_default("var x = 2; print x;", &vec_of_strings!["2"]);
}

#[test]
fn test_var_reading_locals_1() {
    check_output_default("{var x = 2; print x;}", &vec_of_strings!["2"]);
}

#[test]
fn test_var_reading_4() {
    check_output_default(
        "var x = 2;\n\
         var y = 3;\n\
         print x * y + 4;",
        &vec_of_strings!["10"],
    );
}

#[test]
fn test_var_reading_locals_2() {
    check_output_default(
        "{\n\
           var x = 2;\n\
           var y = 3;\n\
           print x * y + 4;\n\
         }\n",
        &vec_of_strings!["10"],
    );
}

#[test]
fn test_div_by_zero() {
    check_output_default("print 1 / 0;", &vec_of_strings!["inf"]);
}

#[test]
fn test_setitem_globals() {
    check_output_default(
        "var breakfast = \"beignets\";\n\
         var beverage = \"cafe au lait\";\n\
         breakfast = \"beignets with \" + beverage;\n\
         print breakfast;",
        &vec_of_strings!["beignets with cafe au lait"],
    );
}

#[test]
fn test_setitem_locals() {
    check_output_default(
        "{\n\
           var breakfast = \"beignets\";\n\
           var beverage = \"cafe au lait\";\n\
           breakfast = \"beignets with \" + beverage;\n\
           print breakfast;\n\
         }\n",
        &vec_of_strings!["beignets with cafe au lait"],
    );
}

#[test]
fn test_read_in_own_initializer() {
    check_error_default(
        "{\n\
           var a = \"outer\";\n\
           {\n\
             var a = a;\n\
           }\n\
         }\n",
        &|err: &str| assert!(err.starts_with("Cannot read local variable in its own initializer.")),
    )
}

#[test]
fn test_if_stmt() {
    check_output_default(
        "var x = 0;\n\
         var y = 1;\n\
         if (x) {\n\
           print x;\n\
         }\n\
         if (y) {\n\
           print y;\n\
         }",
        &vec_of_strings!["1"],
    );
}

#[test]
fn test_if_then_else_1() {
    check_output_default(
        "var x = 0;\n\
         if (x) {\n\
           print \"hello\";\n\
         } else {\n\
           print \"goodbye\";\n\
         }",
        &vec_of_strings!["goodbye"],
    );
}

#[test]
fn test_if_then_else_2() {
    check_output_default(
        "var x = 1;\n\
         if (x) {\n\
           print \"hello\";\n\
         } else {\n\
           print \"goodbye\";\n\
         }",
        &vec_of_strings!["hello"],
    );
}

#[test]
fn test_print_locals() {
    check_output_default(
        "{\n\
           var x = 0;\n\
           var y = 1;\n\
           print x;\n\
           print y;\n\
         }",
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_print_globals() {
    check_output_default(
        "var x = 0;\n\
         var y = 1;\n\
         print x;\n\
         print y;\n",
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_and_1() {
    check_output_default(
        "var x = false;\n\
         var y = true;\n\
         if (y and x) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_and_2() {
    check_output_default(
        "var x = false;\n\
         var y = true;\n\
         if (x and y) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_and_3() {
    check_output_default(
        "var x = true;\n\
         var y = true;\n\
         if (y and x) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_1() {
    check_output_default(
        "var x = false;\n\
         var y = true;\n\
         if (y or x) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_2() {
    check_output_default(
        "var x = false;\n\
         var y = true;\n\
         if (x or y) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_3() {
    check_output_default(
        "var x = false;\n\
         var y = false;\n\
         if (y or x) {\n\
           print \"cat\";\n\
         } else {\n\
           print \"dog\";\n\
         }\n",
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_while() {
    check_output_default(
        "{var x = 0;\n\
         var sum = 0;\n\
         while (x < 100) {\n\
           x = x + 1;\n\
           sum = sum + x;\n\
         }\n\
         print sum;}",
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
        "{\n\
           var fact = 1;\n\
           for (var i = 1; i <= 10; i = i + 1) {\n\
             fact = fact * i;\n\
           }\n\
           print fact;\n\
         }",
        &vec_of_strings![format!("{}", fact(10))],
    );
}

#[test]
fn test_functions_1() {
    check_output_default(
        "fun areWeHavingItYet() {\n\
           print \"Yes we are!\";\n\
         }\n\
         \n\
         print areWeHavingItYet;\n",
        &vec_of_strings!["<fn 'areWeHavingItYet'>"],
    )
}

#[test]
fn test_functions_2() {
    check_output_default(
        "fun f(x, y) {\n\
           print x + y;\n\
         }\n\
         \n\
         print f;\n",
        &vec_of_strings!["<fn 'f'>"],
    )
}

#[test]
fn test_functions_3() {
    check_output_default(
        "fun f(x, y) {\n\
           return x + y;\n\
         }\n\
         \n\
         print f;\n",
        &vec_of_strings!["<fn 'f'>"],
    )
}

#[test]
fn test_functions_4() {
    check_output_default(
        "fun f() {\n\
           return;\n\
         }\n\
         \n\
         print f();\n",
        &vec_of_strings!["nil"],
    )
}

#[test]
fn test_functions_5() {
    check_error_default("return 42;", &|err: &str| {
        assert_eq!(err, "Cannot return from top-level code.")
    })
}

#[test]
fn test_functions_6() {
    check_output_default(
        "fun f(x, y) {\n\
           return x + y;\n\
         }\n\
         \n\
         print f(1,2);\n",
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_functions_7() {
    check_output_default(
        "fun g(x) {\n\
           return 2 * x;\n\
         }\n\
         \n\
         fun f(x, y) {\n\
           return g(x) + y;\n\
         }\n\
         \n\
         print f(1,2);\n",
        &vec_of_strings!["4"],
    );
}

#[test]
fn test_functions_8() {
    check_output_default(
        "var x = 2;\n\
         fun f(x) {\n\
           print 2 * x;\n\
         }\n\
         \n\
         f(x);\n\
         print x;\n",
        &vec_of_strings!["4", "2"],
    );
}

#[test]
fn test_functions_9() {
    fn fact(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        return n * fact(n - 1);
    }

    check_output_default(
        "fun fact(n) {\n\
           if (n <= 1) { return 1; }\n\
           return n * fact(n - 1);\n\
         }\n\
         \n\
         print fact(10);\n",
        &vec_of_strings![format!("{}", fact(10))],
    );
}

#[test]
fn test_functions_10() {
    check_output_default(
        "fun isEven(n) {\n\
           if (n = 0) { return true; }\n\
           return isOdd(n - 1);\n\
         }\n\
         fun isOdd(n) {\n\
           if (n = 1) { return true; }\n\
           return isEven(n - 1);\n\
         }\n\
         \n\
         print isEven(10);\n",
        &vec_of_strings!["true"],
    );
}

#[test]
fn test_native_functions() {
    let res = evaluate(
        "fun fib(n) {\n\
           if (n < 2) return n;\n\
           return fib(n - 2) + fib(n - 1);\n\
         }\n\
         \n\
         var start = clock();\n\
         print fib(5);\n\
         print clock() - start;\n\
         print 42;",
        firnas_ext::Extensions::default(),
    );

    match res {
        Ok(output) => {
            assert_eq!(output.len(), 3);
            assert_eq!(output[0], "5");
            assert_eq!(output[2], "42");
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}

#[test]
fn test_get_upval_on_stack() {
    check_output_default(
        "fun outer() {\n\
           var x = \"outside\";\n\
           fun inner() {\n\
             print x;\n\
           }\n\
           inner();\n\
         }\n\
         outer();",
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_set_upval_on_stack() {
    check_output_default(
        "fun outer() {\n\
           var x = \"before\";\n\
           fun inner() {\n\
             x = \"assigned\";\n\
           }\n\
           inner();\n\
           print x;\n\
         }\n\
         outer();",
        &vec_of_strings!["assigned"],
    );
}

#[test]
fn test_closing_upvals_after_return() {
    check_output_default(
        "fun outer() {\n\
           var x = \"outside\";\n\
           fun inner() {\n\
             print x;\n\
           }\n\
           \n\
           return inner;\n\
        }\n\
        \n\
        var closure = outer();\n\
        closure();",
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_closing_upvals_after_scope() {
    check_output_default(
        "var closure;\n\
         {\n\
           var x = \"outside\";\n\
           fun inner() {\n\
             print x;\n\
           }\n\
           \n\
           closure = inner;\n\
        }\n\
        \n\
        closure();",
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_classes_1() {
    check_output_default(
        "class Brioche {}\n\
         print Brioche;\n",
        &vec_of_strings!["<class 'Brioche'>"],
    );
}

#[test]
fn test_classes_instances_1() {
    check_output_default(
        "class Brioche {}\n\
         var instance = Brioche();\n\
         print instance;\n",
        &vec_of_strings!["<Brioche instance>"],
    );
}

#[test]
fn test_setattr_1() {
    check_output_default(
        "class Foo {}\n\
         var foo = Foo();\n\
         foo.attr = 42;\n\
         print foo.attr;\n",
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_setattr_2() {
    check_output_default(
        "class Toast {}\n\
         var toast = Toast();\n\
         print toast.jam = \"grape\";",
        &vec_of_strings!["grape"],
    );
}

#[test]
fn test_setattr_3() {
    check_output_default(
        "class Pair {}\n\
         var pair = Pair();\n\
         pair.first = 1;\n\
         pair.second = 2;\n\
         print pair.first + pair.second;",
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_bound_methods_1() {
    check_output_default(
        "class Foo {\n\
           bar() {\n\
             return 42;
           }\n\
         }\n\
         var foo = Foo();\n\
         print foo.bar;",
        &vec_of_strings!["<bound method of Foo instance>"],
    );
}

#[test]
fn test_calling_bound_methods_no_this() {
    check_output_default(
        "class Scone {\n\
           topping(first, second) {\n\
             print \"scone with \" + first + \" and \" + second;\n\
           }\n\
         }\n\
         \n\
         var scone = Scone();\n\
         scone.topping(\"berries\", \"cream\");",
        &vec_of_strings!["scone with berries and cream"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_1() {
    check_output_default(
        "class Nested {\n\
           method() {\n\
             print this;\n\
           }\n\
         }\n\
         \n\
         Nested().method();",
        &vec_of_strings!["<Nested instance>"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_2() {
    check_output_default(
        "class Nested {\n\
           method() {\n\
             fun function() {\n\
               print this;\n\
             }\n\
             \n\
             function();\n\
           }\n\
         }\n\
         \n\
         Nested().method();",
        &vec_of_strings!["<Nested instance>"],
    );
}

#[test]
fn test_multiple_method_definitions() {
    check_output_default(
        "class Brunch {\n\
           bacon() {}\n\
           eggs() {}\n\
         }\n\
         print Brunch().bacon();",
        &vec_of_strings!["nil"],
    );
}

#[test]
fn test_init_1() {
    check_output_default(
        "class Brunch {\n\
           init(x) {this.x = x;}\n\
           eggs(y) {return this.x + y;}\n\
         }\n\
         print Brunch(2).eggs(3);",
        &vec_of_strings!["5"],
    );
}

#[test]
fn test_invoking_fields() {
    check_output_default(
        "class Oops {\n\
           init() {\n\
             fun f() {\n\
               print \"not a method\";\n\
             }\n\
             \n\
             this.field = f;\n\
           }\n\
         }\n\
         \n\
         var oops = Oops();\n\
         oops.field();\n",
        &vec_of_strings!["not a method"],
    );
}

#[test]
fn test_inheritance_1() {
    check_output_default(
        "class A {\n\
           f() {\n\
             return \"cat\";\n\
           }\n\
         }\n\
         class B < A {}\n\
         var b = B();\n\
         print b.f();",
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_inheritance_2() {
    check_output_default(
        "class A {\n\
           f() {\n\
             return \"cat\";\n\
           }\n\
         }\n\
         class B < A {}\n\
         class C < B {}\n\
         var c = C();\n\
         print c.f();",
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_inheritance_3() {
    check_output_default(
        "class A {\n\
           f() {\n\
             return this.attr;
           }\n\
         }\n\
         class B < A {\n\
           init(attr) {\n\
             this.attr = attr;\n\
           }\n\
         }\n\
         var b = B(42);\n\
         print b.f();",
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheritance_4() {
    check_output_default(
        "class A {\n\
           f() {\n\
             return this.attr;
           }\n\
         }\n\
         class B < A {\n\
         }\n\
         var b = B();\n\
         b.attr = 42;
         print b.f();",
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheriting_non_class() {
    check_error_default(
        "var NotClass = \"So not a class\";\n\
         class OhNo < NotClass {}\n",
        &|err: &str| assert!(err.starts_with("Superclass must be a class, found String at lineno=")),
    )
}

#[test]
fn test_super_1() {
    check_output_default(
        "class A {\n\
           method() {\n\
             print \"A method\";\n\
           }\n\
         }\n\
         \n\
         class B < A {\n\
           method() {\n\
             print \"B method\";\n\
           }\n\
           \n\
           test() {\n\
             super.method();\n\
           }\n\
         }\n\
         \n\
         class C < B {}\n\
         \n\
         C().test();\n",
        &vec_of_strings!["A method"],
    )
}

#[test]
fn test_super_2() {
    check_output_default(
        "class A {\n\
           method() {\n\
             print \"A method\";\n\
           }\n\
         }\n\
         \n\
         class B < A {\n\
           method() {\n\
             print \"B method\";\n\
           }\n\
           \n\
           test() {\n\
             var func = super.method;\n\
             func();\n\
           }\n\
         }\n\
         \n\
         class C < B {}\n\
         \n\
         C().test();\n",
        &vec_of_strings!["A method"],
    )
}

#[test]
fn test_super_3() {
    check_output_default(
        "class Doughnut {\n\
           cook() {\n\
             print \"Dunk in the fryer.\";\n\
             this.finish(\"sprinkles\");\n\
           }\n\
           \n\
           finish(ingredient) {\n\
             print \"Finish with \" + ingredient;\n\
           }\n\
         }\n\
         \n\
         class Cruller < Doughnut {\n\
           finish(ingredient) {\n\
             // No sprinkles.\n\
             super.finish(\"icing\");\n\
           }\n\
         }\n\
         \n\
         Doughnut().cook();\n\
         Cruller().cook();\n",
        &vec_of_strings![
            "Dunk in the fryer.",
            "Finish with sprinkles",
            "Dunk in the fryer.",
            "Finish with icing"
        ],
    )
}

#[test]
fn test_late_binding() {
    check_output_default(
        "fun a() { b(); }\n\
         fun b() { print \"hello world\"; }\n\
         \n\
         a();\n",
        &vec_of_strings!["hello world"],
    )
}

#[test]
fn test_list_building() {
    check_output_lists("print([1,2,3]);", &vec_of_strings!["[1, 2, 3]"])
}

#[test]
fn test_empty_list_building() {
    check_output_lists("print([]);", &vec_of_strings!["[]"])
}

#[test]
fn test_list_concat() {
    check_output_lists(
        "print([1,2,3] + [4,5,6]);",
        &vec_of_strings!["[1, 2, 3, 4, 5, 6]"],
    )
}

#[test]
fn test_len() {
    check_output_lists(
        "print(len(\"\")); \n\
         print(len(\"cat\")); \n\
         print(len([])); \n\
         print(len([1,2,3,4]));",
        &vec_of_strings!["0", "3", "0", "4"],
    )
}

#[test]
fn test_for_each() {
    check_output_lists(
        "fun f(arg) { print arg; } \n\
         forEach([1,2,3,4], f);",
        &vec_of_strings!["1", "2", "3", "4"],
    )
}

#[test]
fn test_map() {
    check_output_lists(
        "fun f(arg) { return arg + 1; } \n\
         print(map(f, [1,2,3,4]));",
        &vec_of_strings!["[2, 3, 4, 5]"],
    )
}

#[test]
fn test_list_subscript() {
    check_output_lists(
        "var xs = [0,1]; \n\
         print(xs[0]); \n\
         print(xs[1]); \n\
         print(xs[-1]); \n\
         print(xs[-2]); \n\
         ",
        &vec_of_strings!["0", "1", "1", "0"],
    )
}

#[test]
fn test_list_setitem_1() {
    check_output_lists(
        "var xs = [0,1]; \n\
         xs[-1] = 42; \n\
         print(xs);",
        &vec_of_strings!["[0, 42]"],
    )
}

#[test]
fn test_list_setitem_2() {
    check_output_lists(
        "var xs = [[0,1]]; \n\
         xs[0][1] = 42; \n\
         print(xs);",
        &vec_of_strings!["[[0, 42]]"],
    )
}

#[test]
fn test_list_setitem_3() {
    check_output_lists(
        "class Foo {}\n\
         var foo = Foo();\n\
         foo.attr = [0];\n\
         foo.attr[0] = 1337;\n\
         print foo.attr;",
        &vec_of_strings!["[1337]"],
    )
}
