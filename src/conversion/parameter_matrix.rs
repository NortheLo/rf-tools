use ndarray::Array2;
use num_complex::Complex;
use num_traits::Float;

#[derive(Debug)]
pub enum ParameterMatrixError {
    NotSquare { rows: usize, cols: usize },

    DimensionMismatch,

    SingularMatrix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterMatrix<T: Float> {
    data: Array2<Complex<T>>,
}
impl<T: Float> ParameterMatrix<T> {
    pub fn new(data: Array2<Complex<T>>) -> Result<Self, ParameterMatrixError> {
        if data.nrows() != data.ncols() {
            return Err(ParameterMatrixError::NotSquare {
                rows: data.nrows(),
                cols: data.ncols(),
            });
        }

        Ok(Self { data })
    }

    pub fn nrows(&self) -> usize {
        self.data.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.data.ncols()
    }

    pub fn as_array(&self) -> &Array2<Complex<T>> {
        &self.data
    }

    pub fn into_array(self) -> Array2<Complex<T>> {
        self.data
    }
}
