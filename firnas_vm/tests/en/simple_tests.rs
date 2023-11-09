use crate::common::{check_error_default, check_output_default};

#[test]
fn it_should_print_var_value() {
    check_output_default("var x = 2; print x;", &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_in_scope() {
    check_output_default("{var x = 2; print x;}", &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_after_mutation() {
    check_output_default(
        r#"
var x = 2;
var y = 3;
print x * y + 4;
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_print_var_value_after_mutation_in_scope() {
    check_output_default(
        r#"
{
    var x = 2;
    var y = 3;
    print x * y + 4;
}
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_return_inf_when_dividing_by_zero() {
    check_output_default("print 1 / 0;", &vec_of_strings!["inf"]);
}

#[test]
fn it_should_set_items_global() {
    check_output_default(
        r#"
var breakfast = "beignets";
var beverage = "cafe au lait";
breakfast = "beignets with " + beverage;
print breakfast;
"#,
        &vec_of_strings!["beignets with cafe au lait"],
    );
}

#[test]
fn it_should_set_items_in_scope() {
    check_output_default(
        r#"
{
    var breakfast = "beignets";
    var beverage = "cafe au lait";
    breakfast = "beignets with " + beverage;
    print breakfast;
}
"#,
        &vec_of_strings!["beignets with cafe au lait"],
    );
}

#[test]
fn it_should_fail_on_read_from_own_initializer() {
    check_error_default(
        r#"
{
    var a = "outer";
    {
        var a = a;
    }
}
"#,
        &|err: &str| assert!(err.starts_with("Cannot read local variable in its own initializer.")),
    )
}

#[test]
fn test_print_locals() {
    check_output_default(
        r#"
{
    var x = 0;
    var y = 1;
    print x;
    print y;
}
"#,
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_print_globals() {
    check_output_default(
        r#"
var x = 0;
var y = 1;
print x;
print y;
"#,
        &vec_of_strings!["0", "1"],
    );
}
