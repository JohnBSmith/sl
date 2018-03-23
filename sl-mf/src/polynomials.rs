
use gamma::gamma;
const SQRT_PI: f64 = 1.7724538509055159;

fn legendre_rec(n: i32, m: i32,x: f64) -> f64{
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

