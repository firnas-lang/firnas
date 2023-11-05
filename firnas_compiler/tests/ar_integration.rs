#![cfg(feature = "ar")]

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
        String::from("اطبع ٤٢ * ١٢؛"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_compiles_2() {
    Compiler::compile(
        String::from(r"اطبع −٢ * ٣ + (−٤ \ ٢)؛"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_var_decl_compiles_1() {
    Compiler::compile(String::from("دع س = ٢؛"), firnas_ext::Extensions::default()).unwrap();
}

#[test]
fn test_var_decl_implicit_nil() {
    Compiler::compile(String::from("دع س؛"), firnas_ext::Extensions::default()).unwrap();
}

#[test]
fn test_var_reading_2() {
    Compiler::compile(
        String::from("دع س؛ اطبع س؛"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_var_reading_3() {
    Compiler::compile(
        String::from("دع س؛ اطبع س * ٢ + س؛"),
        firnas_ext::Extensions::default(),
    )
    .unwrap();
}

#[test]
fn test_this_outside_method_1() {
    check_semantic_error("اطبع هذا؛", &|err: &str| {
        assert!(err.starts_with("Cannot use 'this' outside of class"))
    })
}

#[test]
fn test_this_outside_method_2() {
    let code = "دالة فوو() { اطبع هذا؛ }";
    check_semantic_error(code, &|err: &str| {
        assert!(err.starts_with("Cannot use 'this' outside of class"))
    })
}

#[test]
fn test_self_ineritance_is_error() {
    let code = "صنف ط < ط { }؛";
    check_semantic_error(code, &|err: &str| {
        assert!(err.starts_with("A class cannot inherit from itself."))
    })
}

#[test]
fn test_cant_use_super_outside_class() {
    let code = "دالة د() { اساس.بار()؛ }؛";
    check_semantic_error(code, &|err: &str| {
        assert!(err.starts_with("Can't use 'super' outside of a class"))
    })
}

#[test]
fn test_cant_use_super_in_class_with_no_superclass() {
    let code = "صنف فوو { بار() { اساس.بار() } }؛";
    check_semantic_error(code, &|err: &str| {
        assert!(err.starts_with("Can't use 'super' in a class with no superclass"))
    })
}

#[test]
fn test_setitem_illegal_target_globals() {
    let func_or_err = Compiler::compile(
        String::from(
            r#"
دع س = ٢؛
دع ص = ٣؛
س * ص = ٥؛
"#,
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
            r#"
{
    دع س = ٢؛
    دع ص = ٣؛
    س * ص = ٥؛
}
"#,
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
            r#"
{
    دع س = ٢؛
    دع س = ٣؛
}
"#,
        ),
        firnas_ext::Extensions::default(),
    );

    match func_or_err {
        Err(Error::Semantic(err)) => assert!(err.what.starts_with("Redeclaration of variable")),
        _ => panic!("expected semantic error"),
    }
}
