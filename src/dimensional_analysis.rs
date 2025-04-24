use crate::dimensional::*;
use num::rational::Rational32;
use num::traits::Signed;

#[derive(Debug)]
pub enum DimensionalAnalysisSolution {
    NoSolution,
    UniqueSolution(Vec<Rational32>),
    MultipleSolutions { rank: usize, n: usize },
}

/// Performs Gaussian elimination on a system of equations Ax = b
/// A is represented as a vector of rows, each row is a [Rational32; 7]
/// b is a column vector of size 7
pub fn gaussian_elimination(
    a: &mut Vec<[Rational32; 7]>,
    b: &mut [Rational32; 7],
) -> DimensionalAnalysisSolution {
    let n = a.len(); // number of variables
    let m = 7; // number of equations

    // println!("Initial matrix:");
    // for i in 0..n {
    //     print!("a({}) = [", i);
    //     for (j, &val) in a[i].iter().enumerate() {
    //         if j > 0 {
    //             print!(", ");
    //         }
    //         print!("{}", val);
    //     }
    //     println!("]");
    // }
    // print!("b = [");
    // for (i, &val) in b.iter().enumerate() {
    //     if i > 0 {
    //         print!(", ");
    //     }
    //     print!("{}", val);
    // }
    // println!("]");

    // Forward elimination
    for col in 0..n {
        // Find the row with the largest absolute value in the current column
        let mut max_row = col;
        for row in col..m {
            if a[col][row].abs() > a[col][max_row].abs() {
                max_row = row;
            }
        }

        // If the maximum element is zero, skip this column
        if a[col][max_row] == Rational32::from_integer(0) {
            continue;
        }

        // Swap columns if necessary
        if max_row != col {
            for i in 0..n {
                let temp = a[i][max_row];
                a[i][col] = a[i][max_row];
                a[i][max_row] = temp;
            }
            let temp = b[max_row];
            b[max_row] = b[col];
            b[col] = temp;
        }

        // Eliminate the current column in all rows below
        for row in (col + 1)..m {
            if a[col][row] != Rational32::from_integer(0) {
                let factor = a[col][row] / a[col][col];
                for i in 0..n {
                    a[i][row] = a[i][row] - factor * a[i][col];
                }
                b[row] = b[row] - factor * b[col];
            }
        }
    }

    // println!("After forward elimination:");
    // for i in 0..n {
    //     print!("a({}) = [", i);
    //     for (j, &val) in a[i].iter().enumerate() {
    //         if j > 0 {
    //             print!(", ");
    //         }
    //         print!("{}", val);
    //     }
    //     println!("]");
    // }
    // print!("b = [");
    // for (i, &val) in b.iter().enumerate() {
    //     if i > 0 {
    //         print!(", ");
    //     }
    //     print!("{}", val);
    // }
    // println!("]");

    // Check for multiple solutions
    let mut rank = 0;
    for i in 0..n {
        if a[i][i] != Rational32::from_integer(0) {
            rank += 1;
        }
    }

    if rank < n {
        // println!("Multiple solutions detected: rank = {}, n = {}", rank, n);
        return DimensionalAnalysisSolution::MultipleSolutions { rank: rank, n: n };
    }

    // Back substitution for unique solution
    let mut x = vec![Rational32::from_integer(0); n];
    for row in (0..n).rev() {
        let mut sum = Rational32::from_integer(0);
        for col in (row + 1)..n {
            sum = sum + a[col][row] * x[col];
        }
        if a[row][row] == Rational32::from_integer(0) {
            if b[row] - sum != Rational32::from_integer(0) {
                return DimensionalAnalysisSolution::NoSolution;
            }
            continue;
        }
        x[row] = (b[row] - sum) / a[row][row];
    }

    // print!("Solution: [");
    // for (i, &val) in x.iter().enumerate() {
    //     if i > 0 {
    //         print!(", ");
    //     }
    //     print!("{}", val);
    // }
    // println!("]");
    DimensionalAnalysisSolution::UniqueSolution(x)
}

pub fn solve_dimensional_analysis(
    target: Dimension,
    dimensions: Vec<Dimension>,
) -> DimensionalAnalysisSolution {
    let n = dimensions.len();
    let mut a = vec![[Rational32::from_integer(0); 7]; n];
    let mut b = [Rational32::from_integer(0); 7];

    // Build the coefficient matrix and right-hand side vector
    for i in 0..7 {
        b[i] = target[i]; // Move target to right side with negative sign
        for j in 0..n {
            a[j][i] = dimensions[j][i]; // Transpose the matrix to match dimensions
        }
    }

    gaussian_elimination(&mut a, &mut b)
}
