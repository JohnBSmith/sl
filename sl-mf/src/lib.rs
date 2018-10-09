
extern crate num_complex;

mod gamma;
mod polynomials;
mod ei;
mod antiderivatives;

pub mod sf{
  pub use gamma::*;
  pub use polynomials::*;
  pub use ei::*;
  pub use antiderivatives::*;
}

pub mod nt;
pub mod pc;

#[allow(non_camel_case_types)]
pub type c64 = num_complex::Complex<f64>;

pub const TAU: f64 = 6.283185307179586;
pub const GAMMA: f64 = 0.57721566490153286061;


