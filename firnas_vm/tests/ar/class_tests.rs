use crate::common::check_error_default;
use crate::common::check_output_default;

#[test]
fn test_classes_1() {
    check_output_default(
        r#"
صنف ص_بشر {}
اطبع_سطر(ص_بشر)؛
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
اطبع_سطر(مثال)؛
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
اطبع_سطر(فوو.صفة)؛
"#,
        &vec_of_strings!["٤٢"],
    );
}

#[test]
fn test_setattr_2() {
    check_output_default(
        r#"
صنف ص_خبز_محمص {}
دع خبز_محمص = ص_خبز_محمص()؛
خبز_محمص.مربى = "عنب"؛
اطبع_سطر(خبز_محمص.مربى)؛
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
اطبع_سطر(زوج.اول + زوج.ثاني)؛
"#,
        &vec_of_strings!["٣"],
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
اطبع_سطر(فوو.بار)؛
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
        اطبع_سطر("كعكة مع " + اول + " و " + ثاني)؛
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
        اطبع_سطر(هذا)؛
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
            اطبع_سطر(هذا)؛
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
دع س = فطور().جبنة()؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["عدم"],
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

اطبع_سطر(فطور(٢).بيض(٣))؛
"#,
        &vec_of_strings!["٥"],
    );
}

#[test]
fn test_invoking_fields() {
    check_output_default(
        r#"
صنف ص_أووبس {
    تهيئة() {
        دالة د() {
            اطبع_سطر("ليست وظيفة")؛
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
دع س = ب.د()؛
اطبع_سطر(س)؛
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
دع س = ت.د()؛
اطبع_سطر(س)؛
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
دع س = ب.د()؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["٤٢"],
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
دع س = ب.د()؛
اطبع_سطر(س)؛
"#,
        &vec_of_strings!["٤٢"],
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
        اطبع_سطر("أ وظيفة")؛
    }
}

صنف ص_ب < ص_أ {
    وظيفة() {
        اطبع_سطر("ب وظيفة")؛
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
        اطبع_سطر("أ وظيفة")؛
    }
}

صنف ص_ب < ص_أ {
    وظيفة() {
        اطبع_سطر("ب وظيفة")؛
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
        اطبع_سطر("ضعها في المقلاة")؛
        هذا.انهي("الرشات")؛
    }

    انهي(المكونات) {
        اطبع_سطر("انهي ب" + المكونات)؛
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
    اطبع_سطر("مرحباً يا عالم")؛
}

أ()؛
"#,
        &vec_of_strings!["مرحباً يا عالم"],
    )
}
