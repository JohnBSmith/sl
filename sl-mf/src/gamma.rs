
use num_complex::Complex;

use std::f64::consts::PI;

#[allow(non_camel_case_types)]
type c64 = Complex<f64>;

fn lanczos_gamma(x: f64) -> f64 {
    let p=[
        0.99999999999980993, 676.5203681218851, -1259.1392167224028,
        771.32342877765313, -176.61502916214059, 12.507343278686905,
        -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7
    ];
    let x = x-1.0;
    let mut y = p[0];
    y+=p[1]/(x+1.0); y+=p[2]/(x+2.0);
    y+=p[3]/(x+3.0); y+=p[4]/(x+4.0);
    y+=p[5]/(x+5.0); y+=p[6]/(x+6.0);
    y+=p[7]/(x+7.0); y+=p[8]/(x+8.0);
    let t=x+7.5;
    return (2.0*PI).sqrt()*t.powf(x+0.5)*(-t).exp()*y;
}

pub fn gamma(x: f64) -> f64 {
    if x<0.5 {
        return PI/(x*PI).sin()/lanczos_gamma(1.0-x);
    }else{
        return lanczos_gamma(x);
    }
}

fn lanczos_cgamma(z: c64) -> c64 {
    let p=[
        0.99999999999980993, 676.5203681218851, -1259.1392167224028,
        771.32342877765313, -176.61502916214059, 12.507343278686905,
        -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7
    ];
    let z = z-1.0;
    let mut y = c64{re: p[0], im: 0.0};
    y=y+p[1]/(z+1.0); y=y+p[2]/(z+2.0);
    y=y+p[3]/(z+3.0); y=y+p[4]/(z+4.0);
    y=y+p[5]/(z+5.0); y=y+p[6]/(z+6.0);
    y=y+p[7]/(z+7.0); y=y+p[8]/(z+8.0);
    let t=z+7.5;
    return (2.0*PI).sqrt()*t.powc(z+0.5)*(-t).exp()*y;
}

pub fn cgamma(z: c64) -> c64 {
    if z.re<0.5 {
        return PI/(PI*z).sin()/lanczos_cgamma(1.0-z);
    }else{
        return lanczos_cgamma(z);
    }
}

fn upper_gamma_cf(s: f64, z: f64, n: u32) -> f64 {
  let mut x = 0.0;
  let mut k = n;
  while k != 0 {
      let kf = k as f64;
      x = kf*(s-kf)/(2.0*kf+1.0+z-s+x);
      k-=1;
  }
  return (-z).exp()/(1.0+z-s+x);
}

fn lower_gamma_series(s: f64, z: f64, n: u32) -> f64 {
  let mut y = 0.0;
  let mut p = 1.0/s;
  let mut k: u32 = 1;
  while k<=n {
      y+=p;
      p = p*z/(s+k as f64);
      k+=1;
  }
  return y*(-z).exp();
}

/// Upper incomplete gamma function.
pub fn upper_gamma(s: f64, x: f64) -> f64{
    if s+4.0<x {
        return x.powf(s)*upper_gamma_cf(s,x,40);
    }else{
        return gamma(s)-x.powf(s)*lower_gamma_series(s,x,60);
    }
}

/// Lower incomplete gamma function.
pub fn lower_gamma(s: f64, x: f64) -> f64 {
    if s+4.0<x {
        return gamma(s)-x.powf(s)*upper_gamma_cf(s,x,40);
    }else{
        return x.powf(s)*lower_gamma_series(s,x,60);
    }
}
