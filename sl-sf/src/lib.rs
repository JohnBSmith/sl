
extern crate num_complex;

mod gamma;
pub use gamma::*;

pub mod pc;

#[allow(non_camel_case_types)]
pub type c64 = num_complex::Complex<f64>;


