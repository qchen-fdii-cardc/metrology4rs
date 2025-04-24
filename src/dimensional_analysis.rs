use crate::dimensional::*;
use crate::matrix::Matrix;
use num::rational::Rational32;
use std::fmt;

#[derive(Clone, Debug)]
pub struct DimensionalAnalysis {
    pub target: Dimension,
    pub dependencies: Vec<Dimension>,
}

#[derive(Debug)]
pub enum DimensionalAnalysisSolution {
    NoSolution,
    UniqueSolution(Matrix<Rational32>),
    MultipleSolutions {
        rank: usize,
        n: usize,
        reduced_a: Matrix<Rational32>,
        reduced_b: Matrix<Rational32>,
    },
}

impl fmt::Display for DimensionalAnalysisSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DimensionalAnalysisSolution::NoSolution => write!(f, "No feasible solution."),
            DimensionalAnalysisSolution::UniqueSolution(v) => {
                write!(f, "Unique solution:\n")?;
                write!(f, "{}", v)
            }
            DimensionalAnalysisSolution::MultipleSolutions { rank, n, reduced_a, reduced_b } => {
                writeln!(f, "Multiple solutions (rank = {}, variables = {})", rank, n)?;
                writeln!(f, "Reduced row echelon form:")?;
                writeln!(f, "A:\n{}", reduced_a)?;
                write!(f, "b:\n{}", reduced_b)
            }
        }
    }
}

impl fmt::Display for DimensionalAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ab, _) = self.build_augmented_matrix();
        write!(f, "Ab=\n{}", ab)
    }
}

impl DimensionalAnalysis {
    /// Builds the augmented matrix [A|b] from the dimensional analysis problem
    /// Returns a tuple of (matrix, non_zero_rows) where:
    /// - matrix is the augmented matrix [A|b]
    /// - non_zero_rows is the list of dimension indices that have non-zero values
    fn build_augmented_matrix(&self) -> (Matrix<Rational32>, Vec<usize>) {
        let n = self.dependencies.len();
        
        // Find which rows (dimensions) have at least one non-zero value
        let mut non_zero_rows = Vec::new();
        for i in 0..7 {
            if self.target[i] != Rational32::from_integer(0)
                || self.dependencies.iter().any(|dj| dj[i] != Rational32::from_integer(0))
            {
                non_zero_rows.push(i);
            }
        }

        let m = non_zero_rows.len();
        if m == 0 {
            return (Matrix::new(0, n + 1), non_zero_rows);
        }

        // Build the augmented matrix [A|b]
        let mut ab = Matrix::new(m, n + 1);
        for (row_idx, &dim_idx) in non_zero_rows.iter().enumerate() {
            // Fill the right-hand side (b)
            ab[(row_idx, n)] = self.target[dim_idx];
            
            // Fill the coefficient matrix (A)
            for j in 0..n {
                ab[(row_idx, j)] = self.dependencies[j][dim_idx];
            }
        }

        (ab, non_zero_rows)
    }

    pub fn solve(&self) -> DimensionalAnalysisSolution {
        let n = self.dependencies.len();
        
        // Build the augmented matrix
        let (mut ab, non_zero_rows) = self.build_augmented_matrix();
        let m = non_zero_rows.len();
        
        if m == 0 {
            // Return a zero matrix for trivial solution
            return DimensionalAnalysisSolution::UniqueSolution(Matrix::new(1, n));
        }

        // println!("Augmented matrix [A|b]:");
        // println!("{}", ab);

        // Convert to reduced row echelon form
        let rank = ab.to_row_echelon_form();
        // println!("Reduced row echelon form:");
        // println!("{}", ab);

        // Check for no solution
        for i in 0..m {
            let mut all_zeros = true;
            for j in 0..n {
                if ab[(i, j)] != Rational32::from_integer(0) {
                    all_zeros = false;
                    break;
                }
            }
            if all_zeros && ab[(i, n)] != Rational32::from_integer(0) {
                return DimensionalAnalysisSolution::NoSolution;
            }
        }

        if rank < n {
            // Multiple solutions
            let mut reduced_a = Matrix::new(m, n);
            let mut reduced_b = Matrix::new(m, 1);
            
            // Extract the reduced matrix and vector
            for i in 0..m {
                for j in 0..n {
                    reduced_a[(i, j)] = ab[(i, j)];
                }
                reduced_b[(i, 0)] = ab[(i, n)];
            }

            return DimensionalAnalysisSolution::MultipleSolutions {
                rank,
                n,
                reduced_a,
                reduced_b,
            };
        }

        // Unique solution - create a column matrix
        let mut solution = Matrix::new(1, n);
        for i in 0..n {
            solution[(0, i)] = ab[(i, n)];
        }

        DimensionalAnalysisSolution::UniqueSolution(solution)
    }
}
