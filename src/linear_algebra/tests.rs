// #[cfg(test)]
// mod tests {
    
//     use crate::linear_algebra::{functions::*, matrix::Matrix};

//     #[test]
//     fn test_zero_matrix() {
//         assert_eq!(zero_matrix(2,3).get(), vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
//     }

//     #[test]
//     fn test_matrix_product() {
//         let m1 = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec(), [4.0, 5.0, 6.0].to_vec()]);
//         let m2= Matrix::new(vec![[7.0, 8.0].to_vec(), [9.0, 10.0].to_vec(), [11.0, 12.0].to_vec()]);
//         assert_eq!(matrix_product(m1, m2).get(), vec![[58.0, 64.0], [139.0, 154.0]]);
//     }

//     #[test]
//     #[should_panic(expected = "These matrices cannot be matrix producted.")]
//     fn test_matrix_product_panic() {
//         let m1 = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec(), [4.0, 5.0, 6.0].to_vec()]);
//         let m2= Matrix::new(vec![[7.0, 8.0].to_vec(), [9.0, 10.0].to_vec()]);
//         matrix_product(m1, m2);
//     }

//     #[test]
//     fn test_transpose() {
//         let m = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec(), [4.0, 5.0, 6.0].to_vec()]);
//         assert_eq!(transpose(m).get(), vec![[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]])
//     }

//     #[test]
//     fn test_dot_product() {
//         let m1 = Matrix::new(vec![[1.0, 3.0, -5.0].to_vec()]);
//         let m2= Matrix::new(vec![[4.0, -2.0, -1.0].to_vec()]);
//         assert_eq!(dot_product(m1, m2), 3.0);
//     }

//     #[test]
//     #[should_panic(expected = "These matrices are not row matrix.")]
//     fn test_dot_product_panic_row_matrix() {
//         let m1 = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec(), [4.0, 5.0, 6.0].to_vec()]);
//         let m2= Matrix::new(vec![[7.0, 8.0].to_vec(), [9.0, 10.0].to_vec(), [11.0, 12.0].to_vec()]);
//         dot_product(m1, m2);
//     }

//     #[test]
//     #[should_panic(expected = "These matrices do not have same length.")]
//     fn test_dot_product_panic_length() {
//         let m1 = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec()]);
//         let m2= Matrix::new(vec![[7.0, 8.0].to_vec()]);
//         dot_product(m1, m2);
//     }

//     #[test]
//     fn test_get_rows() {
//         let m = Matrix::new(vec![[1.0, 2.0, 3.0].to_vec(), [4.0, 5.0, 6.0].to_vec()]);
//         assert_eq!(m.get(), vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
//     }

// }