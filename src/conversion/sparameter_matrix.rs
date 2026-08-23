use crate::conversion::parameter_matrix::{ParameterMatrix, ParameterMatrixError};
use crate::conversion::zparameter_matrix::ZParameterMatrix;
use ndarray::Array2;
use num_complex::Complex;
use num_traits::Float;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct SParameterMatrix<T: Float> {
    matrix: ParameterMatrix<T>,
}

/// S-Parameter Matrix describes the scattering of a N-port
/// network and how the incident signal is reflected and scattered
/// across the ports.
impl<T: Float> Deref for SParameterMatrix<T> {
    type Target = ParameterMatrix<T>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<T: Float> SParameterMatrix<T> {
    pub fn new(data: Array2<Complex<T>>) -> Result<Self, ParameterMatrixError> {
        Ok(Self {
            matrix: ParameterMatrix::new(data)?,
        })
    }
    pub fn to_z_parameters(&self) -> Result<ZParameterMatrix<T>, ParameterMatrixError> {
        let eye: Array2<Complex<T>> = Array2::eye(2);
        ZParameterMatrix::new(eye)
    }
}
