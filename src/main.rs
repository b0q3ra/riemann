mod matrix;
use matrix::Matrix;

fn main() {
    let mut mat = Matrix::new(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 3).unwrap();

    println!(
        "Original:
{}",
        mat
    );

    mat.scale(2.0, 0);
    println!(
        "After scaling row 0 by 2:
{}",
        mat
    );

    // reset for next test
    let mut mat = Matrix::new(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 3).unwrap();

    mat.add_scaled_row(0, 2, -3.0);
    println!(
        "Row 0 = Row 0 + (-3) * Row 2:
{}",
        mat
    );

    mat.permute_rows(0, 2);
    println!(
        "After swapping rows 0 and 2:
{}",
        mat
    );
}
