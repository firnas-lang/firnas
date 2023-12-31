use crate::common::check_output_default;
use arabic_utils::arabic_number::ArabicNumber;

#[test]
fn test_if_stmt() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛

اذا_كان(س)
{
    اطبع_سطر(س)؛
}

اذا_كان(ص)
{
    اطبع_سطر(ص)؛
}
"#,
        &vec_of_strings!["١"],
    );
}

#[test]
fn test_if_then_else_1() {
    check_output_default(
        r#"
دع س = ٠؛
اذا_كان(س) {
    اطبع_سطر("مرحباً")؛
} غير_ذلك {
    اطبع_سطر("ودعاً")؛
}
"#,
        &vec_of_strings!["ودعاً"],
    );
}

#[test]
fn test_if_then_else_2() {
    check_output_default(
        r#"
دع س = ١؛
اذا_كان(س) {
    اطبع_سطر("مرحباً")؛
} غير_ذلك {
    اطبع_سطر("ودعاً")؛
}
"#,
        &vec_of_strings!["مرحباً"],
    );
}

#[test]
fn test_and_1() {
    check_output_default(
        r#"
دع س = صح؛
دع ص = خطا؛
اذا_كان(ص و س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_2() {
    check_output_default(
        r#"
دع س = صح؛
دع ص = خطا؛
اذا_كان(ص و س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_3() {
    check_output_default(
        r#"
دع س = صح؛
دع ص = صح؛
اذا_كان(ص و س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_1() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = صح؛
اذا_كان(ص او س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_2() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = صح؛
اذا_كان(ص او س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_or_3() {
    check_output_default(
        r#"
دع س = خطا؛
دع ص = خطا؛
اذا_كان(ص او س) {
    اطبع_سطر("قطة")؛
} غير_ذلك {
    اطبع_سطر("كلب")؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_while() {
    check_output_default(
        r#"
{
    دع س = ٠؛
    دع مجموع = ٠؛
    طالما (س < ١٠٠) {
        س = س + ١؛
        مجموع = مجموع + س؛
    }
    اطبع_سطر(مجموع)؛
}
"#,
        &vec_of_strings!["٥٠٥٠"],
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
    دع عاملي = ١؛
    من (دع س = ١؛ س <= ١٠؛ س = س + ١) {
        عاملي = عاملي * س؛
    }
    اطبع_سطر(عاملي)؛
}
"#,
        &vec_of_strings![format!(
            "{}",
            (fact(10) as f64).to_arabic_decimal().unwrap()
        )],
    );
}
