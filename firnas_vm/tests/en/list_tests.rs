use crate::common::check_output_lists;

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
        r#"
print(len(""));
print(len("cat"));
print(len([]));
print(len([1,2,3,4]));
"#,
        &vec_of_strings!["0", "3", "0", "4"],
    )
}

#[test]
fn test_for_each() {
    check_output_lists(
        r#"
fun f(arg) { print arg; }
forEach([1,2,3,4], f);
"#,
        &vec_of_strings!["1", "2", "3", "4"],
    )
}

#[test]
fn test_map() {
    check_output_lists(
        r#"
fun f(arg) { return arg + 1; }
print(map(f, [1,2,3,4]));
"#,
        &vec_of_strings!["[2, 3, 4, 5]"],
    )
}

#[test]
fn test_list_subscript() {
    check_output_lists(
        r#"
var xs = [0,1];
print(xs[0]);
print(xs[1]);
print(xs[-1]);
print(xs[-2]);
"#,
        &vec_of_strings!["0", "1", "1", "0"],
    )
}

#[test]
fn test_list_setitem_1() {
    check_output_lists(
        r#"
var xs = [0,1];
xs[-1] = 42;
print(xs);
"#,
        &vec_of_strings!["[0, 42]"],
    )
}

#[test]
fn test_list_setitem_2() {
    check_output_lists(
        r#"
var xs = [[0,1]];
xs[0][1] = 42;
print(xs);
"#,
        &vec_of_strings!["[[0, 42]]"],
    )
}

#[test]
fn test_list_setitem_3() {
    check_output_lists(
        r#"
class Foo {}
var foo = Foo();
foo.attr = [0];
foo.attr[0] = 1337;
print foo.attr;
"#,
        &vec_of_strings!["[1337]"],
    )
}
