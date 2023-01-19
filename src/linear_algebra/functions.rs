use super::matrix::length;

/// Returns (mxn) zero matrix.
pub fn zero_matrix(m: usize, n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![];

    for _row_len in 0..m {
        let mut row: Vec<f64> = vec![];
        for _col_len in 0..n {
            row.push(0.0);
        }
        matrix.push(row);
    }

    matrix
}

/// Multiplies two given matrices.
pub fn matrix_product(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m1_length = length(&m1);
    let m2_length = length(&m2);

    if m1_length.1 != m2_length.0 {
        panic!("These matrices cannot be matrix producted.");
    }

    let mut new_matrix = zero_matrix(m1_length.0, m2_length.1);

    let new_matrix_len = length(&new_matrix);

    for row_index in 0..new_matrix_len.0 {
        for col_index in 0..new_matrix_len.1 {
            let mut sum = 0.0;
            for index in 0..m1_length.1 {
                sum += m1[row_index][index] * m2[index][col_index];
            }
            new_matrix[row_index][col_index] = sum;
        }
    }

    new_matrix
}

/// Transposes a matrix.
pub fn transpose(m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m_length = length(&m);
    let mut new_matrix = zero_matrix(m_length.1, m_length.0);

    let new_matrix_len = length(&new_matrix);

    for row_index in 0..new_matrix_len.0 {
        for col_index in 0..new_matrix_len.1 {
            new_matrix[row_index][col_index] = m[col_index][row_index];
        }
    }

    new_matrix
}

/// Multiplies two vectors.
pub fn dot_product(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> f64 {
    let m1_length = length(&m1);
    let m2_length = length(&m2);

    if m1_length.0 != 1 || m2_length.0 != 1 {
        panic!("These matrices are not row matrix.");
    } else if m1_length.1 != m2_length.1 {
        panic!("These matrices do not have same length.");
    }

    let result = matrix_product(m1, transpose(m2))[0][0];

    result
}
