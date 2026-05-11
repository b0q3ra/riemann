use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct QRResult {
    pub q: Matrix,
    pub r: Matrix
}

pub trait QRDecomposable {
    type Error;
    fn qr_decompose(&self) -> Result<QRResult, Self::Error>;
}


impl QRDecomposable for Matrix {
    type Error = String;

    fn qr_decompose(&self) -> Result<QRResult, Self::Error> {

        let q = Matrix::identity(10, 10);
        let r = Matrix::identity(10, 10);
  
        return Ok(
            QRResult {
                 q: q, r: r 
                });
    }
}
