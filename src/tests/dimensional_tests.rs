use crate::dimensional::*;
use num::Rational32;
use std::boxed::Box;

/// 测试维度的字符串表示
/// 包括基本维度和复合维度的显示格式
/// 测试所有可能的维度形式及其标准形式转换
#[test]
fn test_dimension_to_string() {
    let d = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    assert_eq!(format!("{}", d), "LM^2T^3I^4Θ^5N^6J^7");

    let d = Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 0);
    assert_eq!(format!("{}", d), "-");

    let d = Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0);
    assert_eq!(format!("{}", d), "L");
    
    let d = Dimension::PowerLawR(Rational32::new(1, 2), 
    Rational32::new(1, 2), 
    Rational32::new(1, 2), 
    Rational32::new(1, 2), 
    Rational32::new(1, 2), 
    Rational32::new(1, 2), 
    Rational32::new(1, 2));
    assert_eq!(format!("{}", d), "L^1/2M^1/2T^1/2I^1/2Θ^1/2N^1/2J^1/2");

    assert_eq!(format!("{}", LENGTH), "L");
    assert_eq!(format!("{}", MASS), "M");
    assert_eq!(format!("{}", TIME), "T");
    assert_eq!(format!("{}", ELECTRIC_CURRENT), "I");
    assert_eq!(format!("{}", TEMPERATURE), "Θ");
    assert_eq!(format!("{}", AMOUNT_OF_SUBSTANCE), "N");
    assert_eq!(format!("{}", LUMINOUS_INTENSITY), "J");

    assert_eq!(format!("{}", AREA), "L^2");
    assert_eq!(format!("{}", VOLUME), "L^3");
    assert_eq!(format!("{}", VELOCITY), "LT^-1");
    assert_eq!(format!("{}", ACCELERATION), "LT^-2");
    assert_eq!(format!("{}", FORCE), "LMT^-2");
    assert_eq!(format!("{}", ENERGY), "L^2MT^-2");
    assert_eq!(format!("{}", POWER), "L^2MT^-3");
    assert_eq!(format!("{}", DENSITY), "L^-3M");
    assert_eq!(format!("{}", PRESSURE), "L^-1MT^-2");
    assert_eq!(format!("{}", ELECTRIC_CHARGE), "TI");
    assert_eq!(format!("{}", ELECTRIC_POTENTIAL), "L^2MT^-2I^-1");
    assert_eq!(format!("{}", RESISTANCE), "L^2MT^-2I^-2");
    assert_eq!(format!("{}", CAPACITANCE), "L^-2M^-1T^3I^2");
    assert_eq!(format!("{}", INDUCTANCE), "L^2MT^-2I^-2");
    assert_eq!(format!("{}", FLUX), "L^2MT^-1I^-1");
    assert_eq!(format!("{}", FLUX_DENSITY), "MT^-1I^-1");
    assert_eq!(format!("{}", MAGNETIC_FIELD), "MT^-1I^-1");
    assert_eq!(format!("{}", MAGNETIC_FLUX), "L^2MT^-1I^-1");
    assert_eq!(format!("{}", MAGNETIC_FLUX_DENSITY), "LMT^-1I^-1");
}

/// 测试 PowerLaw 形式的维度构造和模式匹配
/// 验证维度值的正确性
#[test]
fn test_dimension_power_law_int() {
    let d = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    match d {
        Dimension::PowerLaw(a, b, c, d, e, f, g) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            assert_eq!(c, 3);
            assert_eq!(d, 4);
            assert_eq!(e, 5);
            assert_eq!(f, 6);
            assert_eq!(g, 7);
        }
        _ => panic!("Invalid dimension"),
    }
}

/// 测试 PowerLawIA 形式的维度构造和模式匹配
/// 验证数组形式的维度值
#[test]
fn test_dimension_power_law_int_array() {
    let d = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    match d {
        Dimension::PowerLawIA(a) => {
            assert_eq!(a, [1, 2, 3, 4, 5, 6, 7]);
        }
        _ => panic!("Invalid dimension"),
    }
}

/// 测试 PowerLawR 形式的维度构造和模式匹配
/// 验证有理数形式的维度值
#[test]
fn test_dimension_power_law_rational() {
    let d = Dimension::PowerLawR(
        Rational32::from_integer(1),
        Rational32::from_integer(2),
        Rational32::from_integer(3),
        Rational32::from_integer(4),
        Rational32::from_integer(5),
        Rational32::from_integer(6),
        Rational32::from_integer(7)
    );
    match d {
        Dimension::PowerLawR(a, _b, _c, _d, _e, _f, _g) => {
            assert_eq!(a, Rational32::from_integer(1));
        }
        _ => panic!("Invalid dimension"),
    }
}

/// 测试 PowerLawRA 形式的维度构造和模式匹配
/// 验证有理数数组形式的维度值
#[test]
fn test_dimension_power_law_rational_array() {
    let d = Dimension::PowerLawRA([
        Rational32::from_integer(1),
        Rational32::from_integer(2),
        Rational32::from_integer(3),
        Rational32::from_integer(4),
        Rational32::from_integer(5),
        Rational32::from_integer(6),
        Rational32::from_integer(7)
    ]);
    match d {
        Dimension::PowerLawRA(a) => {
            assert_eq!(a, [
                Rational32::from_integer(1),
                Rational32::from_integer(2),
                Rational32::from_integer(3),
                Rational32::from_integer(4),
                Rational32::from_integer(5),
                Rational32::from_integer(6),
                Rational32::from_integer(7)
            ]);
        }
        _ => panic!("Invalid dimension"),
    }
}

/// 测试 Product 形式的维度构造和模式匹配
/// 验证维度乘法的正确性
#[test]
fn test_dimension_product() {
    let d = Dimension::Product {
        left: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        right: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7))
    };
    match d {
        Dimension::Product { left, right } => {
            assert_eq!(*left, Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7));
            assert_eq!(*right, Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7));
        }
        _ => panic!("Invalid dimension"),
    }
}

/// 测试维度的显示和标准形式转换
/// 包括所有维度形式的显示格式和标准形式转换
#[test]
fn test_dimension_display() {
    // 1. 基本维度测试
    // 1.1 测试 PowerLaw 形式
    let d = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    assert_eq!(format!("{}", d), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", d), "PowerLaw(1, 2, 3, 4, 5, 6, 7)");
    
    // 1.2 测试 PowerLaw 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", standard), "PowerLaw(1, 2, 3, 4, 5, 6, 7)");

    // 1.3 测试 PowerLawIA 形式
    let d = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(format!("{}", d), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", d), "PowerLawIA([1, 2, 3, 4, 5, 6, 7])");
    
    // 1.4 测试 PowerLawIA 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", standard), "PowerLaw(1, 2, 3, 4, 5, 6, 7)");

    // 1.5 测试 PowerLawR 形式（整数有理数）
    let d = Dimension::PowerLawR(
        Rational32::from_integer(1),
        Rational32::from_integer(2),
        Rational32::from_integer(3),
        Rational32::from_integer(4),
        Rational32::from_integer(5),
        Rational32::from_integer(6),
        Rational32::from_integer(7)
    );
    assert_eq!(format!("{}", d), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", d), "PowerLawR(Ratio { numer: 1, denom: 1 }, Ratio { numer: 2, denom: 1 }, Ratio { numer: 3, denom: 1 }, Ratio { numer: 4, denom: 1 }, Ratio { numer: 5, denom: 1 }, Ratio { numer: 6, denom: 1 }, Ratio { numer: 7, denom: 1 })");
    
    // 1.6 测试 PowerLawR 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "LM^2T^3I^4Θ^5N^6J^7");
    assert_eq!(format!("{:?}", standard), "PowerLaw(1, 2, 3, 4, 5, 6, 7)");

    // 2. 复合维度测试
    // 2.1 测试 Product 形式
    let d = Dimension::Product {
        left: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        right: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7))
    };
    assert_eq!(format!("{}", d), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", d), "Product { left: PowerLaw(1, 2, 3, 4, 5, 6, 7), right: PowerLaw(1, 2, 3, 4, 5, 6, 7) }");
    
    // 2.2 测试 Product 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", standard), "PowerLaw(2, 4, 6, 8, 10, 12, 14)");

    // 2.3 测试 Division 形式
    let d = Dimension::Division {
        left: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        right: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7))
    };
    assert_eq!(format!("{}", d), "-");
    assert_eq!(format!("{:?}", d), "Division { left: PowerLaw(1, 2, 3, 4, 5, 6, 7), right: PowerLaw(1, 2, 3, 4, 5, 6, 7) }");
    
    // 2.4 测试 Division 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "-");
    assert_eq!(format!("{:?}", standard), "PowerLaw(0, 0, 0, 0, 0, 0, 0)");

    // 2.5 测试 PowerI 形式（整数幂）
    let d = Dimension::PowerI {
        original: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        power: 2
    };
    assert_eq!(format!("{}", d), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", d), "PowerI { original: PowerLaw(1, 2, 3, 4, 5, 6, 7), power: 2 }");
    
    // 2.6 测试 PowerI 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", standard), "PowerLaw(2, 4, 6, 8, 10, 12, 14)");

    // 2.7 测试 PowerR 形式（有理数幂）
    let d = Dimension::PowerR {
        original: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        power: Rational32::from_integer(2)
    };
    assert_eq!(format!("{}", d), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", d), "PowerR { original: PowerLaw(1, 2, 3, 4, 5, 6, 7), power: Ratio { numer: 2, denom: 1 } }");
    
    // 2.8 测试 PowerR 的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "L^2M^4T^6I^8Θ^10N^12J^14");
    assert_eq!(format!("{:?}", standard), "PowerLaw(2, 4, 6, 8, 10, 12, 14)");
}

/// 测试维度的相等性比较
/// 包括使用 == 运算符和 equals 方法的比较
#[test]
fn test_equality_comparison() {
    let dim1 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    let dim2 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    let dim3 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 8);

    // Using == operator (PartialEq)
    assert!(dim1 == dim2);
    assert!(dim1 != dim3);

    // Using equals method
    assert!(dim1.eq(&dim2));
    assert!(!dim1.eq(&dim3));

    // Test with PowerLawIA
    let dim4 = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    let dim5 = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    let dim6 = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 8]);

    // Using == operator (PartialEq)
    assert!(dim4 == dim5);
    assert!(dim4 != dim6);

    // Using equals method
    assert!(dim4.eq(&dim5));
    assert!(!dim4.eq(&dim6));

    
}

/// 测试等价维度的构造和比较
/// 包括：
/// 1. 相同维度的不同表示方式
/// 2. 通过乘法构造的等价维度
/// 3. 通过除法构造的等价维度
/// 4. 通过幂运算构造的等价维度
/// 5. 复杂组合的等价维度
/// 6. 有理数指数的等价维度
/// 7. 测试有理数指数的其他情况
#[test]
fn test_equivalent_dimensions() {
    // 1. 相同维度的不同表示方式
    let dim1 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    let dim2 = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(dim1, dim2);
    assert_eq!(dim2, dim1);

    // 2. 通过乘法构造的等价维度
    let dim3 = Dimension::Product {
        left: Box::new(Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0)),  // L
        right: Box::new(Dimension::PowerLaw(0, 1, 0, 0, 0, 0, 0))  // M
    };
    let dim4 = Dimension::PowerLaw(1, 1, 0, 0, 0, 0, 0);  // LM
    assert_eq!(dim3, dim4);
    assert_eq!(dim4, dim3);

    // 3. 通过除法构造的等价维度
    let dim5 = Dimension::Division {
        left: Box::new(Dimension::PowerLaw(2, 1, 0, 0, 0, 0, 0)),  // L^2M
        right: Box::new(Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0))  // L
    };
    let dim6 = Dimension::PowerLaw(1, 1, 0, 0, 0, 0, 0);  // LM
    assert_eq!(dim5, dim6);
    assert_eq!(dim6, dim5);

    // 4. 通过幂运算构造的等价维度
    let dim7 = Dimension::PowerI {
        original: Box::new(Dimension::PowerLaw(1, 1, 0, 0, 0, 0, 0)),  // LM
        power: 2
    };
    let dim8 = Dimension::PowerLaw(2, 2, 0, 0, 0, 0, 0);  // L^2M^2
    assert_eq!(dim7, dim8);
    assert_eq!(dim8, dim7);

    // 5. 复杂组合的等价维度
    let dim9 = Dimension::Product {
        left: Box::new(Dimension::Division {
            left: Box::new(Dimension::PowerLaw(2, 1, 0, 0, 0, 0, 0)),  // L^2M
            right: Box::new(Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0))  // L
        }),
        right: Box::new(Dimension::PowerI {
            original: Box::new(Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0)),  // L
            power: 2
        })
    };
    let dim10 = Dimension::PowerLaw(3, 1, 0, 0, 0, 0, 0);  // L^3M
    assert_eq!(dim9, dim10);
    assert_eq!(dim10, dim9);

    // 6. 有理数指数的等价维度
    let dim11 = Dimension::PowerR {
        original: Box::new(Dimension::PowerLaw(2, 2, 0, 0, 0, 0, 0)),  // L^2M^2
        power: Rational32::new(1, 2)  // 1/2
    };
    let dim12 = Dimension::PowerLaw(1, 1, 0, 0, 0, 0, 0);  // LM
    assert_eq!(dim11, dim12);
    assert_eq!(dim12, dim11);

    // 7. 测试有理数指数的其他情况
    let dim13 = Dimension::PowerR {
        original: Box::new(Dimension::PowerLaw(4, 4, 0, 0, 0, 0, 0)),  // L^4M^4
        power: Rational32::new(1, 2)  // 1/2
    };
    let dim14 = Dimension::PowerLaw(2, 2, 0, 0, 0, 0, 0);  // L^2M^2
    assert_eq!(dim13, dim14);
    assert_eq!(dim14, dim13);
}

/// 测试标准形式转换时的 panic 情况
/// 当维度包含非整数指数时应该 panic
#[test]
#[should_panic(expected = "Dimension is not in standard form")]
fn test_standard_form_panic() {
    let d = Dimension::PowerR {
        original: Box::new(LENGTH),
        power: Rational32::new(1, 2)
    };
    
    assert_eq!(format!("{}", d), "L^1/2");
    assert_eq!(format!("{:?}", d), "PowerR { original: PowerLaw(1, 0, 0, 0, 0, 0, 0), power: Ratio { numer: 1, denom: 2 } }");

    // This should panic because the dimension has rational exponents
    d.standard_form();
}

/// 测试零维度的特殊情况
/// 包括：
/// 1. 所有指数为零的维度
/// 2. 通过除法得到的零维度
/// 3. 零维度的显示格式
#[test]
fn test_zero_dimension() {
    // 1. 测试所有指数为零的维度
    let d = Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 0);
    assert_eq!(format!("{}", d), "-");
    assert_eq!(format!("{:?}", d), "PowerLaw(0, 0, 0, 0, 0, 0, 0)");

    // 2. 测试通过除法得到的零维度
    let d = Dimension::Division {
        left: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7)),
        right: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7))
    };
    assert_eq!(format!("{}", d), "-");
    assert_eq!(format!("{:?}", d), "Division { left: PowerLaw(1, 2, 3, 4, 5, 6, 7), right: PowerLaw(1, 2, 3, 4, 5, 6, 7) }");

    // 3. 测试零维度的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "-");
    assert_eq!(format!("{:?}", standard), "PowerLaw(0, 0, 0, 0, 0, 0, 0)");
}

/// 测试负指数的维度
/// 包括：
/// 1. 基本维度的负指数
/// 2. 通过除法得到的负指数
/// 3. 负指数的显示格式
#[test]
fn test_negative_exponents() {
    // 1. 测试基本维度的负指数
    let d = Dimension::PowerLaw(-1, -2, -3, -4, -5, -6, -7);
    assert_eq!(format!("{}", d), "L^-1M^-2T^-3I^-4Θ^-5N^-6J^-7");
    assert_eq!(format!("{:?}", d), "PowerLaw(-1, -2, -3, -4, -5, -6, -7)");

    // 2. 测试通过除法得到的负指数
    let d = Dimension::Division {
        left: Box::new(Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 0)),
        right: Box::new(Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7))
    };
    assert_eq!(format!("{}", d), "L^-1M^-2T^-3I^-4Θ^-5N^-6J^-7");
    assert_eq!(format!("{:?}", d), "Division { left: PowerLaw(0, 0, 0, 0, 0, 0, 0), right: PowerLaw(1, 2, 3, 4, 5, 6, 7) }");

    // 3. 测试负指数的标准形式
    let standard = d.standard_form();
    assert_eq!(format!("{}", standard), "L^-1M^-2T^-3I^-4Θ^-5N^-6J^-7");
    assert_eq!(format!("{:?}", standard), "PowerLaw(-1, -2, -3, -4, -5, -6, -7)");
}

/// 测试有理数指数的维度
/// 包括：
/// 1. 基本维度的有理数指数
/// 2. 通过幂运算得到的有理数指数
/// 3. 有理数指数的显示格式
#[test]
fn test_rational_exponents() {
    // 1. 测试基本维度的有理数指数
    let d = Dimension::PowerLawR(
        Rational32::new(1, 2),
        Rational32::new(1, 3),
        Rational32::new(1, 4),
        Rational32::new(1, 5),
        Rational32::new(1, 6),
        Rational32::new(1, 7),
        Rational32::new(1, 8)
    );
    assert_eq!(format!("{}", d), "L^1/2M^1/3T^1/4I^1/5Θ^1/6N^1/7J^1/8");
    assert_eq!(format!("{:?}", d), "PowerLawR(Ratio { numer: 1, denom: 2 }, Ratio { numer: 1, denom: 3 }, Ratio { numer: 1, denom: 4 }, Ratio { numer: 1, denom: 5 }, Ratio { numer: 1, denom: 6 }, Ratio { numer: 1, denom: 7 }, Ratio { numer: 1, denom: 8 })");

    // 2. 测试通过幂运算得到的有理数指数
    let d = Dimension::PowerR {
        original: Box::new(Dimension::PowerLaw(1, 1, 1, 1, 1, 1, 1)),
        power: Rational32::new(1, 2)
    };
    assert_eq!(format!("{}", d), "L^1/2M^1/2T^1/2I^1/2Θ^1/2N^1/2J^1/2");
    assert_eq!(format!("{:?}", d), "PowerR { original: PowerLaw(1, 1, 1, 1, 1, 1, 1), power: Ratio { numer: 1, denom: 2 } }");
} 