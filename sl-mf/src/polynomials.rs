
use gamma::gamma;
const SQRT_PI: f64 = 1.7724538509055159;

fn legendre_rec(n: i32, m: i32, x: f64) -> f64{
    if n==m {
        return SQRT_PI/gamma(0.5-n as f64)*(2.0*(1.0-x*x).sqrt()).powi(n);
    }else if n-1==m {
        return x*(2*n-1) as f64*legendre_rec(m,m,x);
    }else{
        let mut a = legendre_rec(m,m,x);
        let mut b = legendre_rec(m+1,m,x);
        let mf = m as f64;
        for k in m+2..n+1 {
            let k = k as f64;
            let h = ((2.0*k-1.0)*x*b-(k-1.0+mf)*a)/(k-mf);
            a=b; b=h;
        }
        return b;
    }
}

/// Associated legendre function P[n,m](x).
pub fn legendre(n: i32, m: i32, x: f64) -> f64 {
    let n = if n<0 {-n-1} else {n};
    if m.abs()>n {
        return 0.0;
    }else if m<0 {
        panic!("In legendre(n,m,x): case m<0 is not implemented.");
    }else{
        return legendre_rec(n,m,x);
    }
}

/// Hermite polynomial H[n](x).
pub fn hermite(n: u32, x: f64) -> f64 {
    if n==0 {
        return 1.0;
    }else if n==1 {
        return 2.0*x;
    }else{
        let mut a = 1.0;
        let mut b = 2.0*x;
        for k in 1..n {
            let h = 2.0*x*b-2.0*k as f64*a;
            a = b; b = h;
        }
        return b;
    }
}

/// Chebyshev polynomial of the first kind: T[n](x).
pub fn chebyshev1(n: u32, x: f64) -> f64 {
    if n==0 {
        return 1.0;
    }else if n==1 {
        return x;
    }else{
        let mut a = 1.0;
        let mut b = x;
        for _ in 1..n {
            let h = 2.0*x*b-a;
            a = b; b = h;
        }
        return b;
    }
}

/// Chebyshev polynomial of the second kind: U[n](x).
pub fn chebyshev2(n: u32, x: f64) -> f64 {
  if n==0 {
      return 1.0;
  }else if n==1 {
      return 2.0*x;
  }else{
      let mut a = 1.0;
      let mut b = 2.0*x;
      for _ in 1..n {
          let h = 2.0*x*b-a;
          a = b; b = h;
      }
      return b;
  }
}
