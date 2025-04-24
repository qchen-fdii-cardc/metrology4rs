use crate::dimensional::*;
use crate::dimensional_analysis::DimensionalAnalysisSolution;
use crate::dimensional_analysis::solve_dimensional_analysis;
use num::Rational32;

#[test]
fn test_solve_unique_solution() {
    match solve_dimensional_analysis(FREQUENCY, vec![LENGTH, MASS, ACCELERATION]) {
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
    println!("Testing multiple solutions case:");
    println!("Target: LENGTH");
    println!("Dimensions: [MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY]");
    match solve_dimensional_analysis(
        LENGTH,
        vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY],
    ) {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n } => {
            assert_eq!(rank, 2);
            assert_eq!(n, 4);
        }
        _ => panic!("Expected multiple solution"),
    }

    match solve_dimensional_analysis(
        DIMENSIONLESS,
        vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY]
    ) {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n} => {
            assert_eq!(rank, 2);
            assert_eq!(n, 4);
        }
        _ => panic!("Explected multiple solution"),
    }
}
