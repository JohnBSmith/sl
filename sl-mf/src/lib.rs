
extern crate num_complex;

mod gamma;
mod polynomials;
mod ei;

pub mod sf{
  pub use gamma::*;
  pub use polynomials::*;
  pub use ei::*;
}

pub mod pc;

#[allow(non_camel_case_types)]
pub type c64 = num_complex::Complex<f64>;


