pub fn qr_string(matrix: Vec<Vec<bool>>) -> String {
    let width = matrix.len();
    let mut out = String::with_capacity((width / 2) * (width + 1));

    for row in 0..width / 2 {
        let row = row * 2;
        for col in 0..width {
            match (matrix[row][col], matrix[row + 1][col]) {
                (false, false) => out.push(' '),
                (true, true) => out.push('█'),
                (false, true) => out.push('▄'),
                (true, false) => out.push('▀'),
            };
        }
        out.push('\n');
    }

    if width % 2 == 1 {
        for i in 0..width {
            match matrix[width - 1][i] {
                false => out.push(' '),
                true => out.push('▀'),
            }
        }
        out.push('\n');
    }
    
    out
}
