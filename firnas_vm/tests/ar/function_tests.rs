use crate::common::check_error_default;
use crate::common::check_output_default;
use crate::common::evaluate;
use arabic_utils::arabic_number::ArabicNumber;

#[test]
fn test_functions_1() {
    check_output_default(
        r#"
دالة هل_سنحصل_عليها() {
    اطبع_سطر("اجل")؛
}

اطبع_سطر(هل_سنحصل_عليها)؛
"#,
        &vec_of_strings!["<fn 'هل_سنحصل_عليها'>"],
    )
}

#[test]
fn test_functions_2() {
    check_output_default(
        r#"
دالة د(س، ص) {
    اطبع_سطر(س + ص)؛
}

اطبع_سطر(د)؛
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

اطبع_سطر(د)؛
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
دع س = د(١،٢)؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["٣"],
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

دع س = د(١،٢)؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["٤"],
    );
}

#[test]
fn test_functions_8() {
    check_output_default(
        r#"
دع س = ٢؛
دالة د(س) {
    اطبع_سطر(٢ * س)؛

}

د(س)؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["٤", "٢"],
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
    اذا_كان (س<= ١) { رد ١؛ }
    رد س * مضروب(س − ١)؛
}
دع س = مضروب(١٠)؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings![format!(
            "{}",
            (fact(10) as f64).to_arabic_decimal().unwrap()
        )],
    );
}

#[test]
fn test_functions_10() {
    check_output_default(
        r#"
دالة هل_الرقم_زوجي(س) {
    اذا_كان (س ==  ٠) { رد صح؛ }
    رد هل_الرقم_فردي(س − ١)؛
}

دالة هل_الرقم_فردي(س) {
    اذا_كان (س ==  ١) { رد صح؛ }
    رد هل_الرقم_زوجي(س − ١)؛
}

اطبع_سطر(هل_الرقم_زوجي(١٠))؛
"#,
        &vec_of_strings!["صح"],
    );
}

#[test]
fn test_native_functions() {
    let res = evaluate(
        r#"
دالة فيبوناتشي(س) {
    اذا_كان(س < ٢) رد س؛
    رد فيبوناتشي(س − ٢) + فيبوناتشي(س − ١)؛
}

دع البداية = ساعة()؛
اطبع_سطر(فيبوناتشي(٥))؛
اطبع_سطر(ساعة() − البداية)؛
اطبع_سطر(٤٢)؛
"#,
        firnas_ext::Extensions::default(),
    );

    match res {
        Ok(output) => {
            assert_eq!(output.len(), 3);
            assert_eq!(output[0], "٥");
            assert_eq!(output[2], "٤٢");
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
        اطبع_سطر(س)؛
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
    اطبع_سطر(س)؛
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
        اطبع_سطر(س)؛
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
        اطبع_سطر(س)؛
    }

    مغلق = داخلي؛
}

مغلق()؛
"#,
        &vec_of_strings!["خارج"],
    );
}
