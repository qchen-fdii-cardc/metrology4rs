use num::traits::{NumAssign, One, Signed, Zero};
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Index, IndexMut};

/// A matrix stored in column-major order
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>, // column-major storage
    rows: usize,
    cols: usize,
}

impl<T> Display for Matrix<T>
where
    T: Clone + Zero + One + Signed + NumAssign + Debug + PartialOrd + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Print the matrix
        write!(f, "[")?;
        for i in 0..self.rows {
            if i == 0 {
                write!(f, "[")?;
            } else {
                write!(f, "\n [")?;
            }

            for j in 0..self.cols {
                let element = format!("{}", self.data[j][i]);
                write!(f, "{}", element)?;
                if j < self.cols - 1 {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
            if i < self.rows - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Matrix<T>
where
    T: Clone + Zero + One + Signed + NumAssign + Debug + PartialOrd,
{
    /// Create a new matrix with given dimensions, initialized to zero
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![T::zero(); rows]; cols];
        Matrix { data, rows, cols }
    }

    /// Create a matrix from a 1D vector column
    pub fn from_col(col: Vec<T>) -> Self {
        if col.is_empty() {
            return Matrix::new(0, 0);
        }
        let rows = col.len();
        let cols = 1;
        let data = vec![col];
        Matrix { data, rows, cols }
    }

    /// Create a matrix from a 1D vector row
    pub fn from_row(row: Vec<T>) -> Self {
        if row.is_empty() {
            return Matrix::new(0, 0);
        }
        let rows = 1;
        let cols = row.len();
        let mut data = vec![vec![T::zero(); rows]; cols];
        for (j, value) in row.into_iter().enumerate() {
            data[j][0] = value;
        }
        Matrix { data, rows, cols }
    }

    /// Create a matrix from a 2D vector (col-major input)
    pub fn from_cols(cols: Vec<Vec<T>>) -> Self {
        if cols.is_empty() {
            return Matrix::new(0, 0);
        }
        let cols_count = cols.len();
        let rows_count = if cols_count > 0 { cols[0].len() } else { 0 };
        Matrix {
            data: cols,
            rows: rows_count,
            cols: cols_count,
        }
    }

    /// Create a matrix from a 2D vector (row-major input)
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        if rows.is_empty() {
            return Matrix::new(0, 0);
        }
        let rows_count = rows.len();
        let cols_count = rows[0].len();

        // Convert from row-major to column-major
        let mut data = vec![vec![T::zero(); rows_count]; cols_count];
        for i in 0..rows_count {
            for j in 0..cols_count {
                data[j][i] = rows[i][j].clone();
            }
        }

        Matrix {
            data,
            rows: rows_count,
            cols: cols_count,
        }
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get a row as a vector
    pub fn row(&self, i: usize) -> Vec<T> {
        if self.rows == 0 || self.cols == 0 {
            return Vec::new();
        }
        let mut row = Vec::with_capacity(self.cols);
        for j in 0..self.cols {
            row.push(self.data[j][i].clone());
        }
        row
    }

    /// Get a column as a vector
    pub fn col(&self, j: usize) -> Vec<T> {
        if self.rows == 0 || self.cols == 0 {
            return Vec::new();
        }
        self.data[j].clone()
    }

    /// Swap two rows
    pub fn swap_rows(&mut self, i1: usize, i2: usize) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }
        for j in 0..self.cols {
            self.data[j].swap(i1, i2);
        }
    }

    /// Swap two columns
    pub fn swap_cols(&mut self, j1: usize, j2: usize) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }
        self.data.swap(j1, j2);
    }

    /// Multiply a row by a scalar
    pub fn mul_row(&mut self, i: usize, scalar: T) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }
        for j in 0..self.cols {
            self.data[j][i] *= scalar.clone();
        }
    }

    /// Add a multiple of one row to another: row_i1 += scalar * row_i2
    pub fn add_mul_row(&mut self, i1: usize, i2: usize, scalar: T) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }
        for j in 0..self.cols {
            let temp = self.data[j][i2].clone() * scalar.clone();
            self.data[j][i1] += temp;
        }
    }

    /// Find the maximum absolute value in a row
    pub fn row_max_abs(&self, i: usize) -> (usize, T) {
        if self.rows == 0 || self.cols == 0 {
            return (0, T::zero());
        }
        let mut max_col = 0;
        let mut max_val = self.data[0][i].abs();

        for j in 1..self.cols {
            let val = self.data[j][i].abs();
            if val > max_val {
                max_col = j;
                max_val = val;
            }
        }

        (max_col, max_val)
    }

    /// Find the maximum absolute value in a column
    pub fn col_max_abs(&self, j: usize) -> (usize, T) {
        if self.rows == 0 || self.cols == 0 {
            return (0, T::zero());
        }
        let mut max_row = 0;
        let mut max_val = self.data[j][0].abs();

        for i in 1..self.rows {
            let val = self.data[j][i].abs();
            if val > max_val {
                max_row = i;
                max_val = val;
            }
        }

        (max_row, max_val)
    }

    /// Reduce the matrix to reduced row echelon form using Gaussian-Jordan elimination
    /// Returns the rank of the matrix
    pub fn to_row_echelon_form(&mut self) -> usize {
        if self.rows == 0 || self.cols == 0 {
            return 0;
        }

        let mut rank = 0;
        let mut pivot_col = 0;

        // Forward elimination (convert to row echelon form)
        for i in 0..self.rows {
            // Find the pivot row (row with largest absolute value in current column)
            let mut max_row = i;
            let mut max_val = self.data[pivot_col][i].abs();

            for k in i + 1..self.rows {
                let val = self.data[pivot_col][k].abs();
                if val > max_val {
                    max_row = k;
                    max_val = val;
                }
            }

            // If the pivot is zero, try next column
            if max_val == T::zero() {
                pivot_col += 1;
                if pivot_col >= self.cols {
                    break;
                }
                continue;
            }

            // Swap rows if necessary
            if max_row != i {
                self.swap_rows(i, max_row);
            }

            // Make the pivot 1
            let pivot = self.data[pivot_col][i].clone();
            if pivot != T::one() {
                self.mul_row(i, T::one() / pivot.clone());
            }

            // Eliminate entries below the pivot
            for k in i + 1..self.rows {
                let factor = self.data[pivot_col][k].clone();
                if factor != T::zero() {
                    self.add_mul_row(k, i, -factor);
                }
            }

            rank += 1;
            pivot_col += 1;
            if pivot_col >= self.cols {
                break;
            }
        }

        // Backward elimination (convert to reduced row echelon form)
        let mut pivot_col = 0;
        for i in 0..rank {
            // Find the pivot column for this row
            while pivot_col < self.cols && self.data[pivot_col][i] == T::zero() {
                pivot_col += 1;
            }
            if pivot_col >= self.cols {
                break;
            }

            // Eliminate entries above the pivot
            for k in 0..i {
                let factor = self.data[pivot_col][k].clone();
                if factor != T::zero() {
                    self.add_mul_row(k, i, -factor);
                }
            }
            pivot_col += 1;
        }

        rank
    }
}

// Implement indexing operations
impl<T> Index<(usize, usize)> for Matrix<T>
where
    T: Clone + Zero + One + Signed + NumAssign + Debug + PartialOrd,
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[j][i] // Note: column-major order
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T>
where
    T: Clone + Zero + One + Signed + NumAssign + Debug + PartialOrd,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[j][i] // Note: column-major order
    }
}
