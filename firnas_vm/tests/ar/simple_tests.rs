use crate::common::check_error_default;
use crate::common::check_output_default;

#[test]
fn it_should_print_var_value() {
    let code = r#"
دع س = ٢؛
اطبع_سطر(س)؛
    "#;
    check_output_default(code, &vec_of_strings!["٢"]);
}

#[test]
fn it_should_print_var_value_in_scope() {
    let code = r#"
{
    دع س = ٢؛
    اطبع_سطر(س)؛
}
        "#;
    check_output_default(code, &vec_of_strings!["٢"]);
}

#[test]
fn it_should_print_var_value_after_mutation() {
    check_output_default(
        r#"
دع س = ٢؛
دع ص = ٣؛
اطبع_سطر(س * ص + ٤)؛
"#,
        &vec_of_strings!["١٠"],
    );
}

#[test]
fn it_should_print_var_value_after_mutation_in_scope() {
    check_output_default(
        r#"
{
    دع س = ٢؛
    دع ص = ٣؛
    اطبع_سطر(س * ص + ٤)؛
}
"#,
        &vec_of_strings!["١٠"],
    );
}

#[test]
fn it_should_return_inf_when_dividing_by_zero() {
    let code = r"اطبع_سطر(٢ \ ٠)؛";
    check_output_default(code, &vec_of_strings!["لانهاية"]);
}

#[test]
fn it_should_set_items_global() {
    check_output_default(
        r#"
دع فطور = "تمر"؛
دع مشروب = "لبن"؛
فطور = فطور + " مع " + مشروب؛
اطبع_سطر(فطور)؛
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
    اطبع_سطر(فطور)؛
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
fn test_print_locals() {
    check_output_default(
        r#"
{
    دع س = ٠؛
    دع ص = ١؛
    اطبع_سطر(س)؛
    اطبع_سطر(ص)؛
}
"#,
        &vec_of_strings!["٠", "١"],
    );
}

#[test]
fn test_print_globals() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛
اطبع_سطر(س)؛
اطبع_سطر(ص)؛
"#,
        &vec_of_strings!["٠", "١"],
    );
}
