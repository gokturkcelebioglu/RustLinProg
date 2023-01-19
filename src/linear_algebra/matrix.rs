/// Returns matrix' row and column lengths as (row, col)
pub fn length(matrix: &Vec<Vec<f64>>) -> (usize, usize) {
    let row = matrix.len();
    let col = matrix[0].len();
    (row, col)
}
