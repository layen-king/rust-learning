use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})\n", &self.0, &self.1)?;
        write!(f, "({},{})", &self.2, &self.3)
    }
}

#[allow(unused)]
fn transpose(t: Matrix) -> Matrix {
    let (f, s, t, k) = (t.0, t.1, t.2, t.3);
    Matrix(f, t, s, k)
}

#[test]
fn test() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("transpose:\n{}", transpose(matrix));
}
