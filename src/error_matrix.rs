pub mod error_matrix {
    type MatrixSize = usize;
    type ErrorPrecision = f64;

    pub enum ErrorMatrixType {
        Basic,
        FloydSteinberg,
        JarvisJudiceNinke,
        Atkinson,
    }

    pub struct ErrorMatrix {
        pub x_origin: MatrixSize,
        pub matrix: Vec<Vec<ErrorPrecision>>,
    }

    impl ErrorMatrix {
        pub fn new(x_origin: MatrixSize, matrix: Vec<Vec<ErrorPrecision>>) -> Self {
            ErrorMatrix { x_origin, matrix }
        }

        pub fn get_value(&self, row: MatrixSize, col: MatrixSize) -> Option<ErrorPrecision> {
            if row < self.matrix.len() && col < self.matrix[row].len() {
                Some(self.matrix[row][col])
            } else {
                None
            }
        }

        pub fn get_type(matrix_type: ErrorMatrixType) -> Self {
            match matrix_type {
                ErrorMatrixType::Basic => ErrorMatrix::new(0, vec![vec![0.0, 0.5], vec![0.5, 0.0]]),
                ErrorMatrixType::FloydSteinberg => ErrorMatrix::new(
                    1,
                    vec![
                        vec![0.0, 0.0, 7.0 / 16.0],
                        vec![3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0],
                    ],
                ),
                ErrorMatrixType::JarvisJudiceNinke => ErrorMatrix::new(
                    2,
                    vec![
                        vec![0.0, 0.0, 0.0, 7.0 / 48.0, 5.0 / 48.0],
                        vec![3.0 / 48.0, 5.0 / 48.0, 7.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0],
                        vec![1.0 / 48.0, 3.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0, 1.0 / 48.0],
                    ],
                ),
                ErrorMatrixType::Atkinson => ErrorMatrix::new(
                    1,
                    vec![
                        vec![0.0, 0.0, 1.0 / 8.0, 1.0 / 8.0],
                        vec![1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 0.0 / 8.0],
                        vec![0.0, 1.0 / 8.0, 0.0, 0.0],
                    ],
                ),
            }
        }
    }
}
