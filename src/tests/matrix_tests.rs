use crate::matrix::Matrix;
use num::rational::Rational32;
use num::BigInt;
use num::traits::FromPrimitive;

#[test]
fn test_matrix_creation() {
    let mat: Matrix<Rational32> = Matrix::new(2, 3);
    assert_eq!(mat.dimensions(), (2, 3));
    assert_eq!(mat[(0, 0)], Rational32::from_integer(0));
    
    // Test empty matrix
    let empty_mat: Matrix<Rational32> = Matrix::new(0, 0);
    assert_eq!(empty_mat.dimensions(), (0, 0));
}

#[test]
fn test_matrix_from_rows() {
    let rows = vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ];
    let mat = Matrix::from_rows(rows);
    assert_eq!(mat.dimensions(), (2, 2));
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(2));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(3));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(4));

    // Test empty matrix
    let empty_mat: Matrix<Rational32> = Matrix::from_rows(Vec::new());
    assert_eq!(empty_mat.dimensions(), (0, 0));
}

#[test]
fn test_row_operations() {
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ]);

    // Test row swap
    mat.swap_rows(0, 1);
    assert_eq!(mat[(0, 0)], Rational32::from_integer(3));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(4));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(2));

    // Test row multiplication
    mat.mul_row(0, Rational32::from_integer(2));
    assert_eq!(mat[(0, 0)], Rational32::from_integer(6));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(8));

    // Test row addition
    mat.add_mul_row(1, 0, Rational32::from_integer(3));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(19));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(26));

    // Test row operations with zero scalar
    mat.mul_row(0, Rational32::from_integer(0));
    assert_eq!(mat[(0, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(0));
}

#[test]
fn test_matrix_with_f64() {
    let mut mat: Matrix<f64> = Matrix::new(2, 2);
    mat[(0, 0)] = 1.0;
    mat[(0, 1)] = 2.0;
    mat[(1, 0)] = 3.0;
    mat[(1, 1)] = 4.0;

    assert_eq!(mat.dimensions(), (2, 2));
    assert_eq!(mat[(0, 0)], 1.0);
    assert_eq!(mat[(0, 1)], 2.0);
    assert_eq!(mat[(1, 0)], 3.0);
    assert_eq!(mat[(1, 1)], 4.0);

    mat.mul_row(0, 2.0);
    assert_eq!(mat[(0, 0)], 2.0);
    assert_eq!(mat[(0, 1)], 4.0);

    // Test with negative values
    mat[(0, 0)] = -1.0;
    mat[(0, 1)] = -2.0;
    assert_eq!(mat[(0, 0)], -1.0);
    assert_eq!(mat[(0, 1)], -2.0);
}

#[test]
fn test_matrix_with_bigint() {
    let mut mat: Matrix<BigInt> = Matrix::new(2, 2);
    mat[(0, 0)] = BigInt::from_i32(1).unwrap();
    mat[(0, 1)] = BigInt::from_i32(2).unwrap();
    mat[(1, 0)] = BigInt::from_i32(3).unwrap();
    mat[(1, 1)] = BigInt::from_i32(4).unwrap();

    assert_eq!(mat.dimensions(), (2, 2));
    assert_eq!(mat[(0, 0)], BigInt::from_i32(1).unwrap());
    assert_eq!(mat[(0, 1)], BigInt::from_i32(2).unwrap());
    assert_eq!(mat[(1, 0)], BigInt::from_i32(3).unwrap());
    assert_eq!(mat[(1, 1)], BigInt::from_i32(4).unwrap());

    mat.mul_row(0, BigInt::from_i32(2).unwrap());
    assert_eq!(mat[(0, 0)], BigInt::from_i32(2).unwrap());
    assert_eq!(mat[(0, 1)], BigInt::from_i32(4).unwrap());

    // Test with large numbers
    mat[(0, 0)] = BigInt::from_i32(1000000).unwrap();
    assert_eq!(mat[(0, 0)], BigInt::from_i32(1000000).unwrap());
}

#[test]
fn test_row_and_col_access() {
    let mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
        vec![Rational32::from_integer(4), Rational32::from_integer(5), Rational32::from_integer(6)],
    ]);

    // Test row access
    let row0 = mat.row(0);
    assert_eq!(row0, vec![
        Rational32::from_integer(1),
        Rational32::from_integer(2),
        Rational32::from_integer(3)
    ]);

    // Test column access
    let col1 = mat.col(1);
    assert_eq!(col1, vec![
        Rational32::from_integer(2),
        Rational32::from_integer(5)
    ]);

    // Test with empty matrix
    let empty_mat: Matrix<Rational32> = Matrix::new(0, 0);
    assert!(empty_mat.row(0).is_empty());
    assert!(empty_mat.col(0).is_empty());
}

#[test]
fn test_row_max_abs() {
    let mat = Matrix::from_rows(vec![
        vec![Rational32::new(-1, 2), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::new(-4, 1)],
    ]);

    let (col, val) = mat.row_max_abs(0);
    assert_eq!(col, 1);
    assert_eq!(val, Rational32::from_integer(2));

    let (col, val) = mat.row_max_abs(1);
    assert_eq!(col, 1);
    assert_eq!(val, Rational32::from_integer(4));

    // Test with all zeros
    let zero_mat: Matrix<Rational32> = Matrix::new(2, 2);
    let (col, val) = zero_mat.row_max_abs(0);
    assert_eq!(col, 0);
    assert_eq!(val, Rational32::from_integer(0));
}

#[test]
fn test_column_swap() {
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ]);

    mat.swap_cols(0, 1);
    assert_eq!(mat[(0, 0)], Rational32::from_integer(2));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(1));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(4));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(3));

    // Test swapping same column
    mat.swap_cols(0, 0);
    assert_eq!(mat[(0, 0)], Rational32::from_integer(2));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(4));
}

#[test]
fn test_zero_matrix() {
    let mat: Matrix<Rational32> = Matrix::new(3, 3);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(mat[(i, j)], Rational32::from_integer(0));
        }
    }

    // Test with different dimensions
    let mat: Matrix<Rational32> = Matrix::new(2, 4);
    for i in 0..2 {
        for j in 0..4 {
            assert_eq!(mat[(i, j)], Rational32::from_integer(0));
        }
    }
}

#[test]
fn test_col_max_abs() {
    // Test with positive and negative values
    let mat = Matrix::from_rows(vec![
        vec![Rational32::new(-1, 2), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::new(-4, 1)],
    ]);

    let (row, val) = mat.col_max_abs(0);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(3));

    let (row, val) = mat.col_max_abs(1);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(4));

    // Test with all positive values
    let mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ]);

    let (row, val) = mat.col_max_abs(0);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(3));

    let (row, val) = mat.col_max_abs(1);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(4));

    // Test with a larger matrix
    let mat = Matrix::from_rows(vec![
        vec![Rational32::new(-1, 2), Rational32::from_integer(2), Rational32::from_integer(0)],
        vec![Rational32::from_integer(3), Rational32::new(-4, 1), Rational32::from_integer(5)],
        vec![Rational32::from_integer(1), Rational32::from_integer(1), Rational32::from_integer(6)],
    ]);

    let (row, val) = mat.col_max_abs(0);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(3));

    let (row, val) = mat.col_max_abs(1);
    assert_eq!(row, 1);
    assert_eq!(val, Rational32::from_integer(4));

    let (row, val) = mat.col_max_abs(2);
    assert_eq!(row, 2);
    assert_eq!(val, Rational32::from_integer(6));

    // Test with all zeros
    let zero_mat: Matrix<Rational32> = Matrix::new(2, 2);
    let (row, val) = zero_mat.col_max_abs(0);
    assert_eq!(row, 0);
    assert_eq!(val, Rational32::from_integer(0));
}

#[test]
fn test_index_operations() {
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ]);

    // Test read access
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(2));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(3));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(4));

    // Test write access
    mat[(0, 0)] = Rational32::from_integer(5);
    assert_eq!(mat[(0, 0)], Rational32::from_integer(5));

    // Test with different types
    let mut mat_f64: Matrix<f64> = Matrix::new(2, 2);
    mat_f64[(0, 0)] = 1.5;
    assert_eq!(mat_f64[(0, 0)], 1.5);
}

#[test]
fn test_row_echelon_form() {
    // Test case 1: Full rank matrix
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
        vec![Rational32::from_integer(4), Rational32::from_integer(5), Rational32::from_integer(6)],
        vec![Rational32::from_integer(7), Rational32::from_integer(8), Rational32::from_integer(9)],
    ]);
    println!("Original matrix:");
    println!("{}", mat);
    let rank = mat.to_row_echelon_form();
    println!("Reduced row echelon form:");
    println!("{}", mat);
    assert_eq!(rank, 2);  // This matrix has rank 2
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(2, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(0));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(1));
    assert_eq!(mat[(2, 1)], Rational32::from_integer(0));

    // Test case 2: Matrix with zero rows
    let mut empty_mat: Matrix<Rational32> = Matrix::new(0, 0);
    let rank = empty_mat.to_row_echelon_form();
    assert_eq!(rank, 0);

    // Test case 3: Matrix with all zeros
    let mut zero_mat: Matrix<Rational32> = Matrix::new(3, 3);
    let rank = zero_mat.to_row_echelon_form();
    assert_eq!(rank, 0);

    // Test case 4: Matrix with linearly dependent rows
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(2), Rational32::from_integer(4)],
    ]);
    println!("Original matrix:");
    println!("{}", mat);
    let rank = mat.to_row_echelon_form();
    println!("Reduced row echelon form:");
    println!("{}", mat);
    assert_eq!(rank, 1);  // Rows are linearly dependent
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(2));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(0));

    // Test case 5: Matrix with negative values
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::new(-1, 1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::new(-4, 1)],
    ]);
    println!("Original matrix:");
    println!("{}", mat);
    let rank = mat.to_row_echelon_form();
    println!("Reduced row echelon form:");
    println!("{}", mat);
    assert_eq!(rank, 2);  // Full rank
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(0));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(1));

    // Test case 6: Matrix that requires both forward and backward elimination
    let mut mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
        vec![Rational32::from_integer(2), Rational32::from_integer(4), Rational32::from_integer(6)],
        vec![Rational32::from_integer(1), Rational32::from_integer(1), Rational32::from_integer(1)],
    ]);
    println!("Original matrix:");
    println!("{}", mat);
    let rank = mat.to_row_echelon_form();
    println!("Reduced row echelon form:");
    println!("{}", mat);
    assert_eq!(rank, 2);  // Rank is 2
    assert_eq!(mat[(0, 0)], Rational32::from_integer(1));
    assert_eq!(mat[(1, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(2, 0)], Rational32::from_integer(0));
    assert_eq!(mat[(0, 1)], Rational32::from_integer(0));
    assert_eq!(mat[(1, 1)], Rational32::from_integer(1));
    assert_eq!(mat[(2, 1)], Rational32::from_integer(0));
}

#[test]
fn test_matrix_display() {
    // Test case 1: Regular matrix
    let mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2)],
        vec![Rational32::from_integer(3), Rational32::from_integer(4)],
    ]);
    let expected = "[[1,2],
 [3,4]]";
    assert_eq!(format!("{}", mat), expected);

    // Test case 2: Matrix with different width numbers
    let mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(200)],
        vec![Rational32::from_integer(30), Rational32::from_integer(4)],
    ]);
    let expected = "[[1,200],
 [30,4]]";
    assert_eq!(format!("{}", mat), expected);

    // Test case 3: Matrix with fractions
    let mat = Matrix::from_rows(vec![
        vec![Rational32::new(1, 2), Rational32::new(3, 4)],
        vec![Rational32::new(5, 6), Rational32::new(7, 8)],
    ]);
    let expected = "[[1/2,3/4],
 [5/6,7/8]]";
    assert_eq!(format!("{}", mat), expected);

    // Test case 4: Empty matrix
    let empty_mat: Matrix<Rational32> = Matrix::new(0, 0);
    let expected = "[]";
    assert_eq!(format!("{}", empty_mat), expected);

    // Test case 5: Row vector
    let row_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
    ]);
    let expected = "[[1,2,3]]";
    assert_eq!(format!("{}", row_vec), expected);

    // Test case 6: Column vector
    let col_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1)],
        vec![Rational32::from_integer(2)],
        vec![Rational32::from_integer(3)],
    ]);
    let expected = "[[1],
 [2],
 [3]]";
    assert_eq!(format!("{}", col_vec), expected);

    // Test case 7: Larger matrix
    let mat = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
        vec![Rational32::from_integer(4), Rational32::from_integer(5), Rational32::from_integer(6)],
        vec![Rational32::from_integer(7), Rational32::from_integer(8), Rational32::from_integer(9)],
    ]);
    let expected = "[[1,2,3],
 [4,5,6],
 [7,8,9]]";
    assert_eq!(format!("{}", mat), expected);
}

#[test]
fn test_row_and_column_vectors() {
    // Test row vector (1x3)
    let row_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(2), Rational32::from_integer(3)],
    ]);
    assert_eq!(row_vec.dimensions(), (1, 3));
    let expected = "[[1,2,3]]";
    assert_eq!(format!("{}", row_vec), expected);

    // Test column vector (3x1)
    let col_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1)],
        vec![Rational32::from_integer(2)],
        vec![Rational32::from_integer(3)],
    ]);
    assert_eq!(col_vec.dimensions(), (3, 1));
    let expected = "[[1],
 [2],
 [3]]";
    assert_eq!(format!("{}", col_vec), expected);

    // Test row vector with fractions
    let row_vec = Matrix::from_rows(vec![
        vec![Rational32::new(1, 2), Rational32::new(3, 4), Rational32::new(5, 6)],
    ]);
    let expected = "[[1/2,3/4,5/6]]";
    assert_eq!(format!("{}", row_vec), expected);

    // Test column vector with fractions
    let col_vec = Matrix::from_rows(vec![
        vec![Rational32::new(1, 2)],
        vec![Rational32::new(3, 4)],
        vec![Rational32::new(5, 6)],
    ]);
    let expected = "[[1/2],
 [3/4],
 [5/6]]";
    assert_eq!(format!("{}", col_vec), expected);

    // Test row vector with different width numbers
    let row_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1), Rational32::from_integer(200), Rational32::from_integer(30)],
    ]);
    let expected = "[[1,200,30]]";
    assert_eq!(format!("{}", row_vec), expected);

    // Test column vector with different width numbers
    let col_vec = Matrix::from_rows(vec![
        vec![Rational32::from_integer(1)],
        vec![Rational32::from_integer(200)],
        vec![Rational32::from_integer(30)],
    ]);
    let expected = "[[1],
 [200],
 [30]]";
    assert_eq!(format!("{}", col_vec), expected);
} 