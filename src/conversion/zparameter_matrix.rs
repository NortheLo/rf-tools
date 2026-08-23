use crate::conversion::parameter_matrix::{ParameterMatrix, ParameterMatrixError};
use crate::conversion::sparameter_matrix::SParameterMatrix;
use ndarray::Array2;
use num_complex::Complex;
use num_traits::Float;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct ZParameterMatrix<T: Float> {
    matrix: ParameterMatrix<T>,
}

impl<T: Float> Deref for ZParameterMatrix<T> {
    type Target = ParameterMatrix<T>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<T: Float> ZParameterMatrix<T> {
    pub fn new(data: Array2<Complex<T>>) -> Result<Self, ParameterMatrixError> {
        Ok(Self {
            matrix: ParameterMatrix::new(data)?,
        })
    }

    pub fn to_s_parameters(&self) -> Result<SParameterMatrix<T>, ParameterMatrixError> {
        let eye: Array2<Complex<T>> = Array2::eye(2);
        SParameterMatrix::new(eye)
    }
}
