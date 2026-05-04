use std::fmt;

#[derive(Debug)]
pub struct Matrix {
    data: Vec<f64>,
    cols: usize,
    rows: usize,
}

impl Matrix {
    // new matrix
    pub fn new(data: &[f64], rows: usize, cols: usize) -> Result<Self, String> {
        if rows * cols != data.iter().count() {
            return Err("Size of the array does not match".to_string());
        }

        Ok(Matrix {
            data: data.to_vec(),
            rows,
            cols,
        })
    }

    // scale row
    pub fn scale(&mut self, scalar: f64, row: usize) {
        let start = row * self.cols;
        let end = start + self.cols;
        for element in &mut self.data[start..end] {
            *element *= scalar;
        }
    }

    pub fn add_scaled_row(&mut self, target: usize, source: usize, scalar: f64) {
        let cols = self.cols;
        let t_start = target * cols;
        let s_start = source * cols;

        for c in 0..cols {
            self.data[t_start + c] += scalar * self.data[s_start + c];
        }
    }

    // swap two rows
    pub fn permute_rows(&mut self, row_a: usize, row_b: usize) {
        let cols = self.cols;
        let a_start = row_a * cols;
        let b_start = row_b * cols;

        for c in 0..cols {
            self.data.swap(a_start + c, b_start + c);
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            let start = r * self.cols;
            let end = start + self.cols;
            let row = &self.data[start..end];

            // Format each row like [1.0, 2.0, 3.0]
            write!(f, "[")?;
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", val)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
