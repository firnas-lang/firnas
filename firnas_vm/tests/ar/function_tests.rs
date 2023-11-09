use crate::common::check_error_default;
use crate::common::check_output_default;
use crate::common::evaluate;

#[test]
fn test_functions_1() {
    check_output_default(
        r#"
دالة هل_سنحصل_عليها() {
    اطبع "اجل"؛
}

اطبع هل_سنحصل_عليها؛
"#,
        &vec_of_strings!["<fn 'هل_سنحصل_عليها'>"],
    )
}

#[test]
fn test_functions_2() {
    check_output_default(
        r#"
دالة د(س، ص) {
    اطبع س + ص؛
}

اطبع د؛
"#,
        &vec_of_strings!["<fn 'د'>"],
    )
}

#[test]
fn test_functions_3() {
    check_output_default(
        r#"
دالة د(س، ص) {
    رد س + ص؛
}

اطبع د؛
"#,
        &vec_of_strings!["<fn 'د'>"],
    )
}

#[test]
fn test_functions_4() {
    check_output_default(
        r#"
دالة د() {
    رد؛
}
دع س = د()؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["عدم"],
    )
}

#[test]
fn test_functions_5() {
    check_error_default("رد ٤٢؛", &|err: &str| {
        assert_eq!(err, "Cannot return from top-level code.")
    })
}

#[test]
fn test_functions_6() {
    check_output_default(
        r#"
دالة د(س، ص) {
    رد س + ص؛
}

اطبع د(١،٢)؛
"#,
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_functions_7() {
    check_output_default(
        r#"
دالة ت(س) {
    رد ٢ * س؛
}

دالة د(س، ص) {
    رد ت(س) + ص؛
}

اطبع د(١، ٢)؛
"#,
        &vec_of_strings!["4"],
    );
}

#[test]
fn test_functions_8() {
    check_output_default(
        r#"
دع س = ٢؛
دالة د(س) {
    اطبع ٢ * س؛
}

د(س)؛
اطبع س؛
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
دالة مضروب(س) {
    لو (س<= ١) { رد ١؛ }
    رد س * مضروب(س − ١)؛
}

اطبع مضروب(١٠)؛
"#,
        &vec_of_strings![format!("{}", fact(10))],
    );
}

#[test]
fn test_functions_10() {
    check_output_default(
        r#"
دالة هل_الرقم_زوجي(س) {
    لو (س ==  ٠) { رد صحيح؛ }
    رد هل_الرقم_فردي(س − ١)؛
}

دالة هل_الرقم_فردي(س) {
    لو (س ==  ١) { رد صحيح؛ }
    رد هل_الرقم_زوجي(س − ١)؛
}

اطبع هل_الرقم_زوجي(١٠)؛
"#,
        &vec_of_strings!["true"],
    );
}

#[test]
fn test_native_functions() {
    let res = evaluate(
        r#"
دالة فيبوناتشي(س) {
    لو(س < ٢) رد س؛
    رد فيبوناتشي(س − ٢) + فيبوناتشي(س − ١)؛
}

دع البداية = ساعة()؛
اطبع فيبوناتشي(٥)؛
اطبع ساعة() − البداية؛
اطبع ٤٢؛
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
دالة خارجي() {
    دع س = "في خارجي"؛
    دالة داخلي() {
        اطبع س؛
    }
    داخلي()؛
}
خارجي()؛
"#,
        &vec_of_strings!["في خارجي"],
    );
}

#[test]
fn test_set_upval_on_stack() {
    check_output_default(
        r#"
دالة خارجي() {
    دع س = "قبل"؛
    دالة داخلي() {
        س = "مكلف"؛
    }
    داخلي()؛
    اطبع س؛
}
خارجي()؛
"#,
        &vec_of_strings!["مكلف"],
    );
}

#[test]
fn test_closing_upvals_after_return() {
    check_output_default(
        r#"
دالة خارجي() {
    دع س = "خارج"؛
    دالة دخلي() {
        اطبع س؛
    }
    رد دخلي؛
}

دع مقفل = خارجي()؛
مقفل()؛
"#,
        &vec_of_strings!["خارج"],
    );
}

#[test]
fn test_closing_upvals_after_scope() {
    check_output_default(
        r#"
دع مغلق؛
{
    دع س = "خارج"؛
    دالة داخلي() {
        اطبع س؛
    }

    مغلق = داخلي؛
}

مغلق()؛
"#,
        &vec_of_strings!["خارج"],
    );
}
