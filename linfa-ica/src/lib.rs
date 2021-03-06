#[macro_use]
extern crate ndarray;

pub mod error;
pub mod fast_ica;

use ndarray::NdFloat;
use ndarray_linalg::Lapack;
use num_traits::FromPrimitive;

pub trait Float: NdFloat + Lapack + Default + Clone + FromPrimitive {}

impl Float for f32 {}
impl Float for f64 {}
