use crate::dimensional::*;
use num::Rational32;
use std::boxed::Box;

// =============================================
// Basic Dimension Construction Tests
// =============================================

#[test]
fn test_basic_dimension_construction() {
    // Test PowerLaw construction
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

    // Test PowerLawIA construction
    let d = Dimension::PowerLawIA([1, 2, 3, 4, 5, 6, 7]);
    match d {
        Dimension::PowerLawIA(a) => {
            assert_eq!(a, [1, 2, 3, 4, 5, 6, 7]);
        }
        _ => panic!("Invalid dimension"),
    }

    // Test PowerLawR construction
    let d = Dimension::PowerLawR(
        Rational32::from_integer(1),
        Rational32::from_integer(2),
        Rational32::from_integer(3),
        Rational32::from_integer(4),
        Rational32::from_integer(5),
        Rational32::from_integer(6),
        Rational32::from_integer(7),
    );
    match d {
        Dimension::PowerLawR(a, _b, _c, _d, _e, _f, _g) => {
            assert_eq!(a, Rational32::from_integer(1));
        }
        _ => panic!("Invalid dimension"),
    }
}

// =============================================
// Dimension Display and Formatting Tests
// =============================================

#[test]
fn test_dimension_display() {
    // Test basic dimensions
    assert_eq!(format!("{}", LENGTH), "L");
    assert_eq!(format!("{}", MASS), "M");
    assert_eq!(format!("{}", TIME), "T");
    assert_eq!(format!("{}", ELECTRIC_CURRENT), "I");
    assert_eq!(format!("{}", TEMPERATURE), "Θ");
    assert_eq!(format!("{}", AMOUNT_OF_SUBSTANCE), "N");
    assert_eq!(format!("{}", LUMINOUS_INTENSITY), "J");

    // Test derived dimensions
    assert_eq!(format!("{}", AREA), "L^2");
    assert_eq!(format!("{}", VOLUME), "L^3");
    assert_eq!(format!("{}", VELOCITY), "LT^-1");
    assert_eq!(format!("{}", ACCELERATION), "LT^-2");
    assert_eq!(format!("{}", FORCE), "LMT^-2");
    assert_eq!(format!("{}", ENERGY), "L^2MT^-2");
    assert_eq!(format!("{}", POWER), "L^2MT^-3");
}

// =============================================
// Dimension Operations Tests
// =============================================

#[test]
fn test_dimension_operations() {
    // Test multiplication
    let force = MASS * ACCELERATION;
    assert_eq!(force, FORCE);

    // Test division
    let pressure = FORCE / AREA;
    assert_eq!(pressure, PRESSURE);

    // Test power operations
    let length_squared = LENGTH.powi(2);
    assert_eq!(length_squared[0], Rational32::from_integer(2));

    let length_sqrt = LENGTH.sqrt();
    assert_eq!(length_sqrt[0], Rational32::new(1, 2));

    // Test reciprocal
    let force_reciprocal = FORCE.reciprocal();
    assert_eq!(force_reciprocal[0], Rational32::from_integer(-1));
    assert_eq!(force_reciprocal[1], Rational32::from_integer(-1));
    assert_eq!(force_reciprocal[2], Rational32::from_integer(2));
}

// =============================================
// Dimension Comparison Tests
// =============================================

#[test]
fn test_dimension_comparison() {
    let dim1 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    let dim2 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 7);
    let dim3 = Dimension::PowerLaw(1, 2, 3, 4, 5, 6, 8);

    // Test equality
    assert!(dim1[0] == dim2[0]);
    assert!(dim1[1] == dim2[1]);
    assert!(dim1[2] == dim2[2]);
    assert!(dim1[3] == dim2[3]);
    assert!(dim1[4] == dim2[4]);
    assert!(dim1[5] == dim2[5]);
    assert!(dim1[6] == dim2[6]);

    // Test inequality
    assert!(
        dim1[0] != dim3[0]
            || dim1[1] != dim3[1]
            || dim1[2] != dim3[2]
            || dim1[3] != dim3[3]
            || dim1[4] != dim3[4]
            || dim1[5] != dim3[5]
            || dim1[6] != dim3[6]
    );
}

// =============================================
// Special Cases Tests
// =============================================

#[test]
fn test_special_cases() {
    // Test dimensionless
    assert!(DIMENSIONLESS.is_dimensionless());
    assert!(DIMENSIONLESS.is_zero());

    // Test zero dimension
    let d = Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 0);
    assert_eq!(format!("{}", d), "-");
    assert_eq!(d[0], Rational32::from_integer(0));
    assert_eq!(d[1], Rational32::from_integer(0));
    assert_eq!(d[2], Rational32::from_integer(0));

    // Test negative exponents
    let d = Dimension::PowerLaw(-1, -2, -3, -4, -5, -6, -7);
    assert_eq!(format!("{}", d), "L^-1M^-2T^-3I^-4Θ^-5N^-6J^-7");
    assert_eq!(d[0], Rational32::from_integer(-1));
    assert_eq!(d[1], Rational32::from_integer(-2));
    assert_eq!(d[2], Rational32::from_integer(-3));
}

// =============================================
// Error Cases Tests
// =============================================

#[test]
#[should_panic(expected = "no add operation allowed")]
fn test_add_different_dimensions() {
    let _ = LENGTH + MASS;
}

#[test]
#[should_panic(expected = "no sub operation allowed")]
fn test_sub_different_dimensions() {
    let _ = LENGTH - MASS;
}

#[test]
#[should_panic(expected = "Dimension is not in standard form")]
fn test_standard_form_panic() {
    let d = Dimension::PowerR {
        original: Box::new(LENGTH),
        power: Rational32::new(1, 2),
    };
    d.standard_form();
}

// =============================================
// Index Access Tests
// =============================================

#[test]
fn test_dimension_index_access() {
    // Test basic dimension indices
    assert_eq!(LENGTH[0], Rational32::from_integer(1));
    assert_eq!(LENGTH[1], Rational32::from_integer(0));
    assert_eq!(LENGTH[2], Rational32::from_integer(0));

    assert_eq!(MASS[0], Rational32::from_integer(0));
    assert_eq!(MASS[1], Rational32::from_integer(1));
    assert_eq!(MASS[2], Rational32::from_integer(0));

    assert_eq!(TIME[0], Rational32::from_integer(0));
    assert_eq!(TIME[1], Rational32::from_integer(0));
    assert_eq!(TIME[2], Rational32::from_integer(1));
}

#[test]
#[should_panic(expected = "Index out of bounds:")]
fn test_index_out_of_scope() {
    _ = LENGTH[7];
}

// =============================================
// Exponent Manipulation Tests
// =============================================

#[test]
fn test_exponent_manipulation() {
    // Test setting exponents
    let length = LENGTH.with_exponent(0, Rational32::from_integer(2));
    assert_eq!(length[0], Rational32::from_integer(2));
    assert_eq!(length[1], Rational32::from_integer(0));

    // Test rational exponents
    let length = LENGTH.with_exponent(0, Rational32::new(1, 2));
    assert_eq!(length[0], Rational32::new(1, 2));
}
