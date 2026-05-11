use crate::matrix::Matrix;

/// Result of an LU decomposition with partial pivoting: P × A = L × U
#[derive(Debug, Clone)]
pub struct LUPResult {
    /// Permutation matrix (product of row swaps)
    pub p: Matrix,
    /// Lower triangular matrix (unit diagonal)
    pub l: Matrix,
    /// Upper triangular matrix
    pub u: Matrix,
}

/// Trait for types that can be decomposed via LU with partial pivoting.
pub trait LUPDecomposable {
    type Error;
    fn lu_decompose(&self) -> Result<LUPResult, Self::Error>;
}

impl LUPDecomposable for Matrix {
    type Error = String;

    /// Computes the LUP decomposition using Doolittle's method with partial pivoting.
    ///
    /// Returns `(P, L, U)` such that `P × A = L × U`, where:
    /// - `P` is a permutation matrix
    /// - `L` is unit lower triangular
    /// - `U` is upper triangular
    fn lu_decompose(&self) -> Result<LUPResult, String> {
        let n = self.rows();

        if n != self.cols() {
            return Err("Matrix must be square for LU decomposition".to_string());
        }

        // Initialize U as a copy of A, L = I, P = I
        let mut u = self.clone();
        let mut l = Matrix::identity(n, n);
        let mut p = Matrix::identity(n, n);

        for k in 0..n {
            // --- Partial pivoting: find the pivot row ---
            let mut max_val = u.get_at(k, k).unwrap().abs();
            let mut pivot_row = k;

            for i in (k + 1)..n {
                let val = u.get_at(i, k).unwrap().abs();
                if val > max_val {
                    max_val = val;
                    pivot_row = i;
                }
            }

            if max_val < 1e-15 {
                return Err(format!("Matrix is singular (pivot near zero at column {})", k));
            }

            // Swap rows in U, P, and in L (but only columns 0..k for L)
            if pivot_row != k {
                u.permute_rows(k, pivot_row);
                p.permute_rows(k, pivot_row);
                // Swap rows in L for columns before k
                for j in 0..k {
                    let tmp = l.get_at(k, j).unwrap();
                    l.set_at(k, j, l.get_at(pivot_row, j).unwrap())?;
                    l.set_at(pivot_row, j, tmp)?;
                }
            }

            // --- Elimination ---
            let pivot = u.get_at(k, k).unwrap();
            for i in (k + 1)..n {
                let factor = u.get_at(i, k).unwrap() / pivot;
                l.set_at(i, k, factor)?;

                // Subtract factor * row k from row i in U
                for j in k..n {
                    let u_ij = u.get_at(i, j).unwrap() - factor * u.get_at(k, j).unwrap();
                    u.set_at(i, j, u_ij)?;
                }
            }
        }

        Ok(LUPResult { p, l, u })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lu_decomposition_2x2() {
        // A = [[4, 3], [6, 3]]
        let a = Matrix::new(&[4.0, 3.0, 6.0, 3.0], 2, 2).unwrap();
        let result = a.lu_decompose().unwrap();

        // Verify P * A ≈ L * U
        let pa = multiply(&result.p, &a);
        let lu = multiply(&result.l, &result.u);

        for i in 0..2 {
            for j in 0..2 {
                let diff = (pa.get_at(i, j).unwrap() - lu.get_at(i, j).unwrap()).abs();
                assert!(diff < 1e-12, "Mismatch at ({},{}): {}", i, j, diff);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_3x3() {
        let a = Matrix::new(&[2.0, -1.0, -2.0, -4.0, 6.0, 3.0, -4.0, -2.0, 8.0], 3, 3).unwrap();
        let result = a.lu_decompose().unwrap();

        let pa = multiply(&result.p, &a);
        let lu = multiply(&result.l, &result.u);

        for i in 0..3 {
            for j in 0..3 {
                let diff = (pa.get_at(i, j).unwrap() - lu.get_at(i, j).unwrap()).abs();
                assert!(diff < 1e-12, "Mismatch at ({},{}): {}", i, j, diff);
            }
        }
    }

    #[test]
    fn test_non_square_rejected() {
        let a = Matrix::new(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
        assert!(a.lu_decompose().is_err());
    }

    /// Helper: naive matrix multiplication (used only in tests)
    fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
        let m = a.rows();
        let n = a.cols();
        let p = b.cols();
        let mut c = Matrix::new(&vec![0.0; m * p], m, p).unwrap();
        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a.get_at(i, k).unwrap() * b.get_at(k, j).unwrap();
                }
                c.set_at(i, j, sum).unwrap();
            }
        }
        c
    }
}


