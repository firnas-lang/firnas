use crate::common::check_error_default;
use crate::common::check_output_default;
use crate::common::evaluate;

#[test]
fn test_functions_1() {
    check_output_default(
        r#"
fun areWeHavingItYet() {
    print "Yes we are!";
}

printLine(areWeHavingItYet);
"#,
        &vec_of_strings!["<fn 'areWeHavingItYet'>"],
    )
}

#[test]
fn test_functions_2() {
    check_output_default(
        r#"
fun f(x, y) {
    printLine(x + y);
}

printLine(f);
"#,
        &vec_of_strings!["<fn 'f'>"],
    )
}

#[test]
fn test_functions_3() {
    check_output_default(
        r#"
fun f(x, y) {
    return x + y;
}

printLine(f);
"#,
        &vec_of_strings!["<fn 'f'>"],
    )
}

#[test]
fn test_functions_4() {
    check_output_default(
        r#"
fun f() {
    return;
}

var x = f();
printLine(x);
"#,
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
        r#"
fun f(x, y) {
    return x + y;
}
var x = f(1,2);
printLine(x);
"#,
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_functions_7() {
    check_output_default(
        r#"
fun g(x) {
    return 2 * x;
}

fun f(x, y) {
    return g(x) + y;
}

var x = f(1, 2);
printLine(x);
"#,
        &vec_of_strings!["4"],
    );
}

#[test]
fn test_functions_8() {
    check_output_default(
        r#"
var x = 2;
fun f(x) {
    printLine(2 * x);
}

f(x);
printLine(x);
"#,
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
        r#"
fun fact(n) {
    if (n <= 1) { return 1; }
    return n * fact(n - 1);
}

printLine(fact(10));
"#,
        &vec_of_strings![format!("{}", fact(10))],
    );
}

#[test]
fn test_functions_10() {
    check_output_default(
        r#"
fun isEven(n) {
    if (n = 0) { return true; }
    return isOdd(n - 1);
}

fun isOdd(n) {
    if (n = 1) { return true; }
    return isEven(n - 1);
}

printLine(isEven(10));
"#,
        &vec_of_strings!["true"],
    );
}

#[test]
fn test_native_functions() {
    let res = evaluate(
        r#"
fun fib(n) {
    if (n < 2) return n;
    return fib(n - 2) + fib(n - 1);
}

var start = clock();
printLine(fib(5));
printLine(clock() - start);
printLine(42);
"#,
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
        r#"
fun outer() {
    var x = "outside";
    fun inner() {
        printLine(x);
    }
    inner();
}
outer();
"#,
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_set_upval_on_stack() {
    check_output_default(
        r#"
fun outer() {
    var x = "before";
    fun inner() {
        x = "assigned";
    }
    inner();
    printLine(x);
}
outer();
"#,
        &vec_of_strings!["assigned"],
    );
}

#[test]
fn test_closing_upvals_after_return() {
    check_output_default(
        r#"
fun outer() {
    var x = "outside";
    fun inner() {
        printLine(x);
    }

    return inner;
}

var closure = outer();
closure();
"#,
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_closing_upvals_after_scope() {
    check_output_default(
        r#"
var closure;
{
    var x = "outside";
    fun inner() {
        printLine(x);
    }

    closure = inner;
}

closure();
"#,
        &vec_of_strings!["outside"],
    );
}

#[test]
fn test_late_binding() {
    check_output_default(
        r#"
fun a() { b(); }
fun b() { printLine("hello world"); }

a();
"#,
        &vec_of_strings!["hello world"],
    )
}
