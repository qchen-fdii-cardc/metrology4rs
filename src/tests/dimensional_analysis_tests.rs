use crate::dimensional::*;
use crate::dimensional_analysis::DimensionalAnalysisSolution;
use crate::dimensional_analysis::solve_dimensional_analysis;
use num::Rational32;

#[test]
fn test_solve_unique_solution() {
    println!("Testing unique solution case:");
    println!("Target: FREQUENCY");
    println!("Dimensions: [LENGTH, MASS, ACCELERATION]");
    let result = solve_dimensional_analysis(FREQUENCY, vec![LENGTH, MASS, ACCELERATION]);
    println!("Result: {}", result);
    match result {
        DimensionalAnalysisSolution::UniqueSolution(coefs) => {
            assert_eq!(
                coefs,
                [
                    Rational32::new(-1, 2),
                    Rational32::from_integer(0),
                    Rational32::new(1, 2)
                ]
            );
        }
        _ => panic!("Expected unique solution"),
    }
}

#[test]
fn test_solve_multiple_solution() {
    println!("\nTesting multiple solutions case 1:");
    println!("Target: LENGTH");
    println!("Dimensions: [MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY]");
    let result = solve_dimensional_analysis(
        LENGTH,
        vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY],
    );
    println!("Result: {}", result);
    match result {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n } => {
            assert_eq!(rank, 2);
            assert_eq!(n, 4);
        }
        _ => panic!("Expected multiple solution"),
    }

    println!("\nTesting multiple solutions case 2:");
    println!("Target: DIMENSIONLESS");
    println!("Dimensions: [MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY]");
    let result = solve_dimensional_analysis(
        DIMENSIONLESS,
        vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY],
    );
    println!("Result: {}", result);
    match result {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n } => {
            assert_eq!(rank, 2);
            assert_eq!(n, 4);
        }
        _ => panic!("Expected multiple solution"),
    }
}
