use crate::common::check_output_default;

#[test]
fn test_if_stmt() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛

لو(س)
{
    اطبع س؛
}

لو(ص)
{
    اطبع ص؛
}
"#,
        &vec_of_strings!["1"],
    );
}

#[test]
fn test_if_then_else_1() {
    check_output_default(
        r#"
دع س = ٠؛
لو(س) {
    اطبع "مرحباً"؛
} اخر {
    اطبع "ودعاً"؛
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
لو(س) {
    اطبع "مرحباً"؛
} اخر {
    اطبع "ودعاً"؛
}
"#,
        &vec_of_strings!["مرحباً"],
    );
}

#[test]
fn test_and_1() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = خطا؛
لو(س و ص) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_2() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = خطا؛
لو(ص و س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
}
"#,
        &vec_of_strings!["كلب"],
    );
}

#[test]
fn test_and_3() {
    check_output_default(
        r#"
دع س = صحيح؛
دع ص = صحيح؛
لو(ص و س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
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
دع ص = صحيح؛
لو(س او ص) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
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
دع ص = صحيح؛
لو(ص او س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
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
لو(ص او س) {
    اطبع "قطة"؛
} اخر {
    اطبع "كلب"؛
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
    اطبع مجموع؛
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
    دع عاملي = ١؛
    من (دع س = ١؛ س <= ١٠؛ س = س + ١) {
        عاملي = عاملي * س؛
    }
    اطبع عاملي؛
}
"#,
        &vec_of_strings![format!("{}", fact(10))],
    );
}
