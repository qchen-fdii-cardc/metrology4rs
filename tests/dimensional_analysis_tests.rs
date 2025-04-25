use metrology4rs::dimensional::*;
use metrology4rs::dimensional_analysis::*;
use metrology4rs::matrix::Matrix;
use num::rational::Rational32;

#[test]
fn test_solve_dimensional_analysis_case_1() {
    // Test case 1: Frequency = Length^(-1/2) * Mass^1 * Acceleration^(1/2)
    let sys = DimensionalAnalysis {
        target: FREQUENCY,
        dependencies: vec![LENGTH, MASS, ACCELERATION],
    };
    let result = sys.solve();

    match result {
        DimensionalAnalysisSolution::UniqueSolution(solution) => {
            let expected = Matrix::from_col(vec![
                Rational32::new(-1, 2),
                Rational32::from_integer(0),
                Rational32::new(1, 2),
            ]);
            assert_eq!(solution, expected, "Expected unique solution");
        }
        _ => panic!("Expected unique solution"),
    }
}

#[test]
fn test_solve_dimensional_analysis_case_2() {
    // Test case 2: No solution
    let sys = DimensionalAnalysis {
        target: FORCE,
        dependencies: vec![LENGTH, TIME],
    };
    let result = sys.solve();

    match result {
        DimensionalAnalysisSolution::NoSolution => (),
        _ => panic!("Expected no solution"),
    }
}
#[test]
fn test_solve_dimensional_analysis_case_3() {
    // Test case 3: Multiple solutions
    let sys = DimensionalAnalysis {
        target: DIMENSIONLESS,
        dependencies: vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY],
    };
    println!("{}", sys);
    let result = sys.solve();

    match result {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n, reduced_a, reduced_b } => {
            assert_eq!(rank, 3, "Expected rank 0");
            assert_eq!(n, 4, "Expected 3 variables");
            println!("A=\n{}", reduced_a);
            println!("b=\n{}", reduced_b);
        }
        _ => panic!("Expected multiple solutions"),
    }
}


#[test]
fn test_solve_dimensional_analysis_case_4() {
    // Test case 3: Multiple solutions
    let sys = DimensionalAnalysis {
        target: LENGTH,
        dependencies: vec![MASS, ACCELERATION, FORCE * VELOCITY.powi(-2), VELOCITY],
    };
    println!("{}", sys);

    let result = sys.solve();


    match result {
        DimensionalAnalysisSolution::MultipleSolutions { rank, n, reduced_a, reduced_b } => {
            assert_eq!(rank, 3, "Expected rank 0");
            assert_eq!(n, 4, "Expected 3 variables");
            println!("A=\n{}", reduced_a);
            println!("b=\n{}", reduced_b);
        }
        _ => panic!("Expected multiple solutions"),
    }
}