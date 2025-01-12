mod bayer_matrix {
    type BayerPrecision = u16;

    pub struct BayerMatrix {
        pub matrix: Vec<Vec<BayerPrecision>>,
    }

    impl BayerMatrix {
        pub fn new(order: u8) -> Self {
            if order == 0 {
                return BayerMatrix {
                    matrix: vec![vec![0]],
                };
            }

            let matrix = BayerMatrix::new(order - 1).matrix;
            let size = 2_usize.pow(order as u32);
            let mut new_matrix = vec![vec![0; size]; size];

            for y in 0..size {
                for x in 0..size {
                    let (new_x, new_y) = (x % matrix.len(), y % matrix.len());
                    new_matrix[y][x] = match (x >= size / 2, y >= size / 2) {
                        (false, false) => matrix[new_y][new_x] * 4,
                        (true, false) => matrix[new_y][new_x] * 4 + 2,
                        (false, true) => matrix[new_y][new_x] * 4 + 3,
                        (true, true) => matrix[new_y][new_x] * 4 + 1,
                    };
                }
            }

            BayerMatrix { matrix: new_matrix }
        }

        pub fn is_valid(&self) -> bool {
            let mut elements: Vec<BayerPrecision> = self
                .matrix
                .iter()
                .flat_map(|row| row.iter())
                .cloned()
                .collect();

            elements.sort_unstable();

            for (i, &num) in elements.iter().enumerate() {
                if num != i as BayerPrecision {
                    return false;
                }
            }

            return true;
        }
    }
}
