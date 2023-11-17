use crate::common::check_error_default;
use crate::common::check_output_default;

#[test]
fn test_classes_1() {
    check_output_default(
        r#"
class Brioche {}
printLine(Brioche);
"#,
        &vec_of_strings!["<class 'Brioche'>"],
    );
}

#[test]
fn test_classes_instances_1() {
    check_output_default(
        r#"
class Brioche {}
var instance = Brioche();
printLine(instance);
"#,
        &vec_of_strings!["<Brioche instance>"],
    );
}

#[test]
fn test_setattr_1() {
    check_output_default(
        r#"
class Foo {}
var foo = Foo();
foo.attr = 42;
printLine(foo.attr);
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_setattr_2() {
    check_output_default(
        r#"
class Toast {}
var toast = Toast();
toast.jam = "grape";
printLine(toast.jam);
"#,
        &vec_of_strings!["grape"],
    );
}

#[test]
fn test_setattr_3() {
    check_output_default(
        r#"
class Pair {}
var pair = Pair();
pair.first = 1;
pair.second = 2;
printLine(pair.first + pair.second);
"#,
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_bound_methods_1() {
    check_output_default(
        r#"
class Foo {
    bar() {
        return 42;
    }
}

var foo = Foo();
printLine(foo.bar);
"#,
        &vec_of_strings!["<bound method of Foo instance>"],
    );
}

#[test]
fn test_calling_bound_methods_no_this() {
    check_output_default(
        r#"
class Scone {
    topping(first, second) {
        printLine("scone with " + first + " and " + second);
    }
}

var scone = Scone();
scone.topping("berries", "cream");
"#,
        &vec_of_strings!["scone with berries and cream"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_1() {
    check_output_default(
        r#"
class Nested {
    method() {
        printLine(this);
    }
}

Nested().method();
"#,
        &vec_of_strings!["<Nested instance>"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_2() {
    check_output_default(
        r#"
class Nested {
    method() {
        fun function() {
            printLine(this);
        }

        function();
    }
}

Nested().method();
"#,
        &vec_of_strings!["<Nested instance>"],
    );
}

#[test]
fn test_multiple_method_definitions() {
    check_output_default(
        r#"
class Brunch {
    bacon() {}
    eggs() {}
}
var x = Brunch().bacon();
printLine(x);
"#,
        &vec_of_strings!["nil"],
    );
}

#[test]
fn test_init_1() {
    check_output_default(
        r#"
class Brunch {
    init(x) { this.x = x; }
    eggs(y) { return this.x + y; }
}
printLine(Brunch(2).eggs(3));
"#,
        &vec_of_strings!["5"],
    );
}

#[test]
fn test_invoking_fields() {
    check_output_default(
        r#"
class Oops {
    init() {
        fun f() {
            printLine("not a method");
        }

        this.field = f;
    }
}

var oops = Oops();
oops.field();
"#,
        &vec_of_strings!["not a method"],
    );
}

#[test]
fn test_inheritance_1() {
    check_output_default(
        r#"
class A {
    f() {
        return "cat";
    }
}
class B < A {}
var b = B();
var x = b.f();
printLine(x);
"#,
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_inheritance_2() {
    check_output_default(
        r#"
class A {
    f() {
        return "cat";
    }
}
class B < A {}
class C < B {}
var c = C();
var x = c.f();
printLine(x);
"#,
        &vec_of_strings!["cat"],
    );
}

#[test]
fn test_inheritance_3() {
    check_output_default(
        r#"
class A {
    f() {
        return this.attr;
    }
}
class B < A {
    init(attr) {
        this.attr = attr;
    }
}

var b = B(42);
var x = b.f();
printLine(x);
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheritance_4() {
    check_output_default(
        r#"
class A {
    f() {
        return this.attr;
    }
}
class B < A {
}
var b = B();
b.attr = 42;
var x = b.f();
printLine(x);
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheriting_non_class() {
    check_error_default(
        r#"
var NotClass = "So not a class";
class OhNo < NotClass {}
"#,
        &|err: &str| assert!(err.starts_with("Superclass must be a class, found String at lineno=")),
    )
}

#[test]
fn test_super_1() {
    check_output_default(
        r#"
class A {
    method() {
        printLine("A method");
    }
}

class B < A {
    method() {
        printLine("B method");
    }

    test() {
        super.method();
    }
}

class C < B {}

C().test();
"#,
        &vec_of_strings!["A method"],
    )
}

#[test]
fn test_super_2() {
    check_output_default(
        r#"
class A {
    method() {
        printLine("A method");
    }
}

class B < A {
    method() {
        printLine("B method");
    }

    test() {
        var func = super.method;
        func();
    }
}

class C < B {}

C().test();
"#,
        &vec_of_strings!["A method"],
    )
}

#[test]
fn test_super_3() {
    check_output_default(
        r#"
class Doughnut {
    cook() {
        printLine("Dunk in the fryer.");
        this.finish("sprinkles");
    }

    finish(ingredient) {
        printLine("Finish with " + ingredient);
    }
}

class Cruller < Doughnut {
    finish(ingredient) {
        // No sprinkles.
        super.finish("icing");
    }
}

Doughnut().cook();
Cruller().cook();
"#,
        &vec_of_strings![
            "Dunk in the fryer.",
            "Finish with sprinkles",
            "Dunk in the fryer.",
            "Finish with icing"
        ],
    )
}
