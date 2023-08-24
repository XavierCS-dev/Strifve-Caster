use num_traits::{Float, NumCast};

/// Struct for operations on a column-major 4x4 matrix
pub struct Matrix4<T> {
    matrix: [[T; 4]; 4],
}

impl<T> Matrix4<T>
where
    T: NumCast + Copy + Float,
{
    pub fn new(matrix: [[T; 4]; 4]) -> Self {
        Self { matrix }
    }

    pub fn get(&self, column: usize, row: usize) -> T {
        self.matrix[column][row]
    }

    pub fn set(&mut self, column: usize, row: usize, value: T) {
        self.matrix[column][row] = value;
    }

    pub fn set_column(&mut self, column: usize, value: [T; 4]) {
        self.matrix[column] = value;
    }
}

impl<T> Matrix4<T>
where
    T: NumCast + Copy + Float,
    [[f32; 4]; 4]: From<[[T; 4]; 4]>,
{
    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        self.matrix.clone().into()
    }
}

impl<T> std::ops::Mul<&Matrix4<T>> for &Matrix4<T>
where
    T: NumCast + Copy + Float,
{
    type Output = Matrix4<T>;

    fn mul(self, rhs: &Matrix4<T>) -> Self::Output {
        // this code can be made cleaner at a later date, it is not a concern for now
        let column_one = [
            rhs[0][0] * self[0][0]
                + rhs[0][1] * self[1][0]
                + rhs[0][2] * self[2][0]
                + rhs[0][3] * self[3][0],
            rhs[0][0] * self[0][1]
                + rhs[0][1] * self[1][1]
                + rhs[0][2] * self[2][1]
                + rhs[0][3] * self[3][1],
            rhs[0][0] * self[0][2]
                + rhs[0][1] * self[1][2]
                + rhs[0][2] * self[2][2]
                + rhs[0][3] * self[3][2],
            rhs[0][0] * self[0][3]
                + rhs[0][1] * self[1][3]
                + rhs[0][2] * self[2][3]
                + rhs[0][3] * self[3][3],
        ];

        let column_two = [
            rhs[1][0] * self[0][0]
                + rhs[1][1] * self[1][0]
                + rhs[1][2] * self[2][0]
                + rhs[1][3] * self[3][0],
            rhs[1][0] * self[0][1]
                + rhs[1][1] * self[1][1]
                + rhs[1][2] * self[2][1]
                + rhs[1][3] * self[3][1],
            rhs[1][0] * self[0][2]
                + rhs[1][1] * self[1][2]
                + rhs[1][2] * self[2][2]
                + rhs[1][3] * self[3][2],
            rhs[1][0] * self[0][3]
                + rhs[1][1] * self[1][3]
                + rhs[1][2] * self[2][3]
                + rhs[1][3] * self[3][3],
        ];

        let column_three = [
            rhs[2][0] * self[0][0]
                + rhs[2][1] * self[1][0]
                + rhs[2][2] * self[2][0]
                + rhs[2][3] * self[3][0],
            rhs[2][0] * self[0][1]
                + rhs[2][1] * self[1][1]
                + rhs[2][2] * self[2][1]
                + rhs[2][3] * self[3][1],
            rhs[2][0] * self[0][2]
                + rhs[2][1] * self[1][2]
                + rhs[2][2] * self[2][2]
                + rhs[2][3] * self[3][2],
            rhs[2][0] * self[0][3]
                + rhs[2][1] * self[1][3]
                + rhs[2][2] * self[2][3]
                + rhs[2][3] * self[3][3],
        ];

        let column_four = [
            rhs[3][0] * self[0][0]
                + rhs[3][1] * self[1][0]
                + rhs[3][2] * self[2][0]
                + rhs[3][3] * self[3][0],
            rhs[3][0] * self[0][1]
                + rhs[3][1] * self[1][1]
                + rhs[3][2] * self[2][1]
                + rhs[3][3] * self[3][1],
            rhs[3][0] * self[0][2]
                + rhs[3][1] * self[1][2]
                + rhs[3][2] * self[2][2]
                + rhs[3][3] * self[3][2],
            rhs[3][0] * self[0][3]
                + rhs[3][1] * self[1][3]
                + rhs[3][2] * self[2][3]
                + rhs[3][3] * self[3][3],
        ];
        let mat = [column_one, column_two, column_three, column_four];
        Matrix4::new(mat)
    }
}

impl<T> std::ops::Index<usize> for Matrix4<T> {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}
