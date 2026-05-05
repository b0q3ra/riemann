mod matrix;
mod decompositions;

use matrix::Matrix;
use decompositions::lu::LUPDecomposable;

fn main() {
    let mut mat = Matrix::new(&[1.0, 2.0, 3.0,
                                1.0, -2.0, 1.0,
                                0.0, 1.0, 1.0
                                ],
                            3, 3).unwrap();

    let lup = mat.lu_decompose().unwrap();

    println!("{}", lup.l);
    println!("{}", lup.u);
}
