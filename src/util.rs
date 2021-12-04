pub const WIDTH: usize = 4;
pub const CELL_WIDTH: usize = 9;

pub fn dot_product(lhs: &[[u32; WIDTH]; WIDTH], rhs: &[[f64; WIDTH]; WIDTH]) -> f64 {
    let mut dp = 0.0;

    for i in 0..WIDTH {
        for j in 0..WIDTH {
            dp += lhs[i][j] as f64 * rhs[i][j];
        }
    }

    dp
}

pub fn reverse(matrix: &mut [[u32; WIDTH]; WIDTH]) {
    for i in 0..WIDTH {
        matrix[i].reverse();
    }
}

pub fn transpose(matrix: &mut [[u32; WIDTH]; WIDTH]) {
    for i in 0..WIDTH {
        for j in i..WIDTH {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}
