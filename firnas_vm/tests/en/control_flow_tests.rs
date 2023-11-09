use crate::common::check_output_default;

#[test]
fn test_if_stmt() {
    check_output_default(
        r#"
var x = 0;
var y = 1;
if (x) {
    print x;
}
if (y) {
    print y;
}
"#,
        &vec_of_strings!["1"],
    );
}

#[test]
fn test_if_then_else_1() {
    check_output_default(
        r#"
var x = 0;
if (x) {
    print "hello";
} else {
    print "goodbye";
}
"#,
        &vec_of_strings!["goodbye"],
    );
}

#[test]
fn test_if_then_else_2() {
    check_output_default(
        r#"
var x = 1;
if (x) {
    print "hello";
} else {
    print "goodbye";
}
"#,
        &vec_of_strings!["hello"],
    );
}

#[test]
fn test_and_1() {
    check_output_default(
        r#"
var x = false;
var y = true;
if (y and x) {
    print "cat";
} else {
    print "dog";
}
"#,
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_and_2() {
    check_output_default(
        r#"
var x = false;
var y = true;
if (x and y) {
    print "cat";
} else {
print "dog";
}
"#,
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_and_3() {
    check_output_default(
        r#"
var x = true;
var y = true;
if (y and x) {
    print "cat";
} else {
    print "dog";
}
"#,
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_1() {
    check_output_default(
        r#"
var x = false;
var y = true;
if (y or x) {
    print "cat";
} else {
    print "dog";
}
"#,
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_2() {
    check_output_default(
        r#"
var x = false;
var y = true;
if (x or y) {
    print "cat";
} else {
    print "dog";
}
"#,
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_or_3() {
    check_output_default(
        r#"
var x = false;
var y = false;
if (y or x) {
    print "cat";
} else {
    print "dog";
}
"#,
        &vec_of_strings!["dog"],
    );
}

#[test]
fn test_while() {
    check_output_default(
        r#"
{
    var x = 0;
    var sum = 0;
    while (x < 100) {
        x = x + 1;
        sum = sum + x;
    }
    print sum;
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
    var fact = 1;
    for (var i = 1; i <= 10; i = i + 1) {
        fact = fact * i;
    }
    print fact;
}
"#,
        &vec_of_strings![format!("{}", fact(10))],
    );
}
