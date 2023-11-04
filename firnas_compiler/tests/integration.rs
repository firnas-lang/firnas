use firnas_compiler::compiler::Compiler;
use firnas_compiler::compiler::Error;

fn check_semantic_error(code: &str, f: &dyn Fn(&str) -> ()) {
    let func_or_err = Compiler::compile(String::from(code), firnas_ext::Extensions::default());

    match func_or_err {
        Err(Error::Semantic(err)) => f(&err.what),
        _ => panic!("expected semantic error"),
    }
}

#[test]
fn test_compiles_1() {
    Compiler::compile(
        String::from("print 42 * 12;"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_compiles_2() {
    Compiler::compile(
        String::from("print -2 * 3 + (-4 / 2);"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_var_decl_compiles_1() {
    Compiler::compile(
        String::from("var x = 2;"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_var_decl_implicit_nil() {
    Compiler::compile(String::from("var x;"), firnas_ext::Extensions::default()).unwrap();
}

#[test]
fn test_var_reading_2() {
    Compiler::compile(
        String::from("var x; print x;"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_var_reading_3() {
    Compiler::compile(
        String::from("var x; print x * 2 + x;"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_this_outside_method_1() {
    check_semantic_error("print this;", &|err: &str| {
        assert!(err.starts_with("Cannot use 'this' outside of class"))
    })
}

#[test]
fn test_this_outside_method_2() {
    check_semantic_error("fun foo() {print this;}", &|err: &str| {
        assert!(err.starts_with("Cannot use 'this' outside of class"))
    })
}

#[test]
fn test_self_ineritance_is_error() {
    check_semantic_error("class A < A {}", &|err: &str| {
        assert!(err.starts_with("A class cannot inherit from itself."))
    })
}

#[test]
fn test_cant_use_super_outside_class() {
    check_semantic_error("fun f() { super.bar(); }", &|err: &str| {
        assert!(err.starts_with("Can't use 'super' outside of a class"))
    })
}

#[test]
fn test_cant_use_super_in_class_with_no_superclass() {
    check_semantic_error("class Foo { bar() { super.bar(); } }", &|err: &str| {
        assert!(err.starts_with("Can't use 'super' in a class with no superclass"))
    })
}

#[test]
fn test_setitem_illegal_target_globals() {
    let func_or_err = Compiler::compile(
        String::from(
            "var x = 2;\n\
             var y = 3;\n\
             x * y = 5;",
        ),
        firnas_ext::Extensions::default(),
    );

    match func_or_err {
        Err(Error::Semantic(err)) => assert!(err.what.starts_with("Invalid assignment target")),
        _ => panic!("expected semantic error"),
    }
}

#[test]
fn test_setitem_illegal_target_locals() {
    let func_or_err = Compiler::compile(
        String::from(
            "{\n\
               var x = 2;\n\
               var y = 3;\n\
               x * y = 5;\n\
             }\n",
        ),
        firnas_ext::Extensions::default(),
    );

    match func_or_err {
        Err(Error::Semantic(err)) => assert!(err.what.starts_with("Invalid assignment target")),
        _ => panic!("expected semantic error"),
    }
}

#[test]
fn test_redeclaration_of_locals_is_error() {
    let func_or_err = Compiler::compile(
        String::from(
            "{\n\
               var x = 2;\n\
               var x = 3;\n\
             }",
        ),
        firnas_ext::Extensions::default(),
    );

    match func_or_err {
        Err(Error::Semantic(err)) => assert!(err.what.starts_with("Redeclaration of variable")),
        _ => panic!("expected semantic error"),
    }
}
