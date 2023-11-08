#![cfg(feature = "ar")]

use firnas_compiler::compiler::*;
use firnas_ext;
use firnas_vm::virtual_machine::*;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn evaluate(code: &str, extensions: firnas_ext::Extensions) -> Result<Vec<String>, String> {
    let func_or_err = Compiler::compile(String::from(code), extensions);

    match func_or_err {
        Ok(func) => {
            let mut vm = VirtualMachine::default();
            let res = vm.interpret(func);
            match res {
                Ok(()) => Ok(vm.get_output()),
                Err(VmError::Runtime(err)) => Err(err),
            }
        }
        Err(Error::Lexical(err)) => Err(err.what),
        Err(Error::Parse(err)) => Err(err.what),
        Err(Error::Semantic(err)) => Err(err.what),
        Err(Error::Internal(err)) => Err(err),
    }
}

fn check_output(code: &str, extensions: firnas_ext::Extensions, expected_output: &[String]) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => assert_eq!(output, expected_output),
        Err(err) => panic!("{}", err),
    }
}

fn check_output_default(code: &str, expected_output: &[String]) {
    check_output(code, firnas_ext::Extensions::default(), expected_output);
}

fn check_output_lists(code: &str, expected_output: &[String]) {
    check_output(
        code,
        firnas_ext::Extensions {
            lists: true,
            ..Default::default()
        },
        expected_output,
    );
}

fn check_error(code: &str, extensions: firnas_ext::Extensions, f: &dyn Fn(&str) -> ()) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => panic!("{:?}", output),
        Err(err) => f(&err),
    }
}

fn check_error_default(code: &str, f: &dyn Fn(&str) -> ()) {
    check_error(code, firnas_ext::Extensions::default(), f);
}

#[test]
fn it_should_print_var_value() {
    let code = "دع س = ٢؛ اطبع س؛";
    check_output_default(code, &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_in_scope() {
    let code = "{ دع س = ٢؛ اطبع س؛ }";
    check_output_default(code, &vec_of_strings!["2"]);
}

#[test]
fn it_should_print_var_value_after_mutation() {
    check_output_default(
        r#"
دع س = ٢؛
دع ص = ٣؛
اطبع س * ص + ٤؛
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_print_var_value_after_mutation_in_scope() {
    check_output_default(
        r#"
{
    دع س = ٢؛
    دع ص = ٣؛
    اطبع س * ص + ٤؛
}
"#,
        &vec_of_strings!["10"],
    );
}

#[test]
fn it_should_return_inf_when_dividing_by_zero() {
    let code = r"اطبع ٢ \ ٠؛";
    check_output_default(code, &vec_of_strings!["inf"]);
}

#[test]
fn it_should_set_items_global() {
    check_output_default(
        r#"
دع فطور = "تمر"؛
دع مشروب = "لبن"؛
فطور = فطور + " مع " + مشروب؛
اطبع فطور؛
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
    اطبع فطور؛
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
fn test_print_locals() {
    check_output_default(
        r#"
{
    دع س = ٠؛
    دع ص = ١؛
    اطبع س؛
    اطبع ص؛
}
"#,
        &vec_of_strings!["0", "1"],
    );
}

#[test]
fn test_print_globals() {
    check_output_default(
        r#"
دع س = ٠؛
دع ص = ١؛
اطبع س؛
اطبع ص؛
"#,
        &vec_of_strings!["0", "1"],
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

اطبع د()؛
"#,
        &vec_of_strings!["nil"],
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

#[test]
fn test_classes_1() {
    check_output_default(
        r#"
صنف ص_بشر {}
اطبع ص_بشر؛
"#,
        &vec_of_strings!["<class 'ص_بشر'>"],
    );
}

#[test]
fn test_classes_instances_1() {
    check_output_default(
        r#"
صنف ص_بشر {}
دع مثال = ص_بشر()؛
اطبع مثال؛
"#,
        &vec_of_strings!["<ص_بشر instance>"],
    );
}

#[test]
fn test_setattr_1() {
    check_output_default(
        r#"
صنف ص_فوو {}
دع فوو = ص_فوو()؛
فوو.صفة = ٤٢؛
اطبع فوو.صفة؛
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_setattr_2() {
    check_output_default(
        r#"
صنف ص_خبز_محمص {}
دع خبز_محمص = ص_خبز_محمص()؛
خبز_محمص.مربى = "عنب"؛
اطبع خبز_محمص.مربى؛
"#,
        &vec_of_strings!["عنب"],
    );
}

#[test]
fn test_setattr_3() {
    check_output_default(
        r#"
صنف ص_زوج {}
دع زوج = ص_زوج()؛
زوج.اول = ١؛
زوج.ثاني = ٢؛
اطبع زوج.اول + زوج.ثاني؛
"#,
        &vec_of_strings!["3"],
    );
}

#[test]
fn test_bound_methods_1() {
    check_output_default(
        r#"
صنف ص_فوو {
    بار() {
        رد ٤٢؛
    }
}

دع فوو = ص_فوو()؛
اطبع فوو.بار؛
"#,
        &vec_of_strings!["<bound method of ص_فوو instance>"],
    );
}

#[test]
fn test_calling_bound_methods_no_this() {
    check_output_default(
        r#"
صنف كعك {
    إضافات(اول، ثاني) {
        اطبع "كعكة مع " + اول + " و " + ثاني؛
    }
}

دع كعكة = كعك()؛
كعكة.إضافات("توت"، "كريمة")؛
"#,
        &vec_of_strings!["كعكة مع توت و كريمة"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_1() {
    check_output_default(
        r#"
صنف متداخلة {
    وظيفة() {
        اطبع هذا؛
    }
}

متداخلة().وظيفة()؛
"#,
        &vec_of_strings!["<متداخلة instance>"],
    );
}

#[test]
fn test_calling_bound_methods_with_this_2() {
    check_output_default(
        r#"
صنف متداخلة {
    وظيفة() {
        دالة د() {
            اطبع هذا؛
        }

        د()؛
    }
}

متداخلة().وظيفة()؛
"#,
        &vec_of_strings!["<متداخلة instance>"],
    );
}

#[test]
fn test_multiple_method_definitions() {
    check_output_default(
        r#"
صنف فطور {
    بيض() {}
    جبنة() {}
}

اطبع فطور().جبنة()؛
"#,
        &vec_of_strings!["nil"],
    );
}

#[test]
fn test_init_1() {
    check_output_default(
        r#"
صنف فطور {
    تهيئة(س) { هذا.س = س؛ }
    بيض(ص) { رد هذا.س + ص؛ }
}

اطبع فطور(٢).بيض(٣)؛
"#,
        &vec_of_strings!["5"],
    );
}

#[test]
fn test_invoking_fields() {
    check_output_default(
        r#"
صنف ص_أووبس {
    تهيئة() {
        دالة د() {
            اطبع "ليست وظيفة"؛
        }

        هذا.حقل = د؛
    }
}

دع أووبس = ص_أووبس()؛
أووبس.حقل()؛
"#,
        &vec_of_strings!["ليست وظيفة"],
    );
}

#[test]
fn test_inheritance_1() {
    check_output_default(
        r#"
صنف ص_أ {
    د() {
        رد "قطة"؛
    }
}
صنف ص_ب < ص_أ { }

دع ب = ص_ب()؛
اطبع ب.د()؛
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_inheritance_2() {
    check_output_default(
        r#"
صنف ص_أ {
    د() {
        رد "قطة"؛
    }
}
صنف ص_ب < ص_أ { }
صنف ص_ت < ص_ب { }
دع ت = ص_ت()؛
اطبع ت.د()؛
"#,
        &vec_of_strings!["قطة"],
    );
}

#[test]
fn test_inheritance_3() {
    check_output_default(
        r#"
صنف ص_أ {
    د() {
        رد هذا.حقل؛
    }
}
صنف ص_ب < ص_أ {
    تهيئة(حقل) {
        هذا.حقل = حقل؛
    }
}

دع ب = ص_ب(٤٢)؛
اطبع ب.د()؛
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheritance_4() {
    check_output_default(
        r#"
صنف ص_أ {
    د() {
        رد هذا.حقل؛
    }
}
صنف ص_ب < ص_أ { }

دع ب = ص_ب()؛
ب.حقل = ٤٢؛
اطبع ب.د()؛
"#,
        &vec_of_strings!["42"],
    );
}

#[test]
fn test_inheriting_non_class() {
    check_error_default(
        r#"
دع ليس_بصنف = ""؛
صنف كلا < ليس_بصنف { }
"#,
        &|err: &str| assert!(err.starts_with("Superclass must be a class, found String at lineno=")),
    )
}

#[test]
fn test_super_1() {
    check_output_default(
        r#"
صنف ص_أ {
    وظيفة() {
        اطبع "أ وظيفة"؛
    }
}

صنف ص_ب < ص_أ {
    وظيفة() {
        اطبع "ب وظيفة"؛
    }

    اختبار() {
        اساس.وظيفة()؛
    }
}

صنف ص_ت < ص_ب { }

ص_ت().اختبار()؛
"#,
        &vec_of_strings!["أ وظيفة"],
    )
}

#[test]
fn test_super_2() {
    check_output_default(
        r#"
صنف ص_أ {
    وظيفة() {
        اطبع "أ وظيفة"؛
    }
}

صنف ص_ب < ص_أ {
    وظيفة() {
        اطبع "ب وظيفة"؛
    }

    اختبار() {
        دع د = اساس.وظيفة؛
        د()؛
    }
}

صنف ص_ت < ص_ب { }

ص_ت().اختبار()؛
"#,
        &vec_of_strings!["أ وظيفة"],
    )
}

#[test]
fn test_super_3() {
    check_output_default(
        r#"
صنف كعكة_محلاة {
    اطبخ() {
        اطبع "ضعها في المقلاة"؛
        هذا.انهي("الرشات")؛
    }

    انهي(المكونات) {
        اطبع "انهي ب" + المكونات؛
    }
}

صنف صنف_كرولر < كعكة_محلاة {
    انهي(المكونات) {
        اساس.انهي("تثليج")؛
    }
}

كعكة_محلاة().اطبخ()؛
صنف_كرولر().اطبخ()؛
"#,
        &vec_of_strings![
            "ضعها في المقلاة",
            "انهي بالرشات",
            "ضعها في المقلاة",
            "انهي بتثليج"
        ],
    )
}

#[test]
fn test_late_binding() {
    check_output_default(
        r#"
دالة أ() {
    ب()؛
}

دالة ب() {
    اطبع "مرحباً يا عالم"؛
}

أ()؛
"#,
        &vec_of_strings!["مرحباً يا عالم"],
    )
}
