
// Arithmetic-geometric mean agm(x,y).
// Complete elliptic integrals K(m), E(m).
// Carlson symmetric form RF(x,y,z), RC(x,y).

#![allow(non_snake_case)]

use std::f64::consts::PI;

pub fn agm(mut x: f64, mut y: f64) -> f64 {
    for _ in 0..20 {
        let xh = (x+y)/2.0;
        let yh = (x*y).sqrt();
        x=xh; y=yh;
        if (x-y).abs()<1E-15 {break;}
    }
    return x;
}

// Modified arithmetic-geometric mean, see
// Semjon Adlaj: "An eloquent formula for the perimeter
// of an ellipse", Notices of the AMS 59(8) (2012), p. 1094-1099.
fn magm(mut x: f64, mut y: f64) -> f64 {
    let mut z=0.0;
    for _ in 0..20 {
        let xh = 0.5*(x+y);
        let r = ((x-z)*(y-z)).sqrt();
        let yh = z+r;
        let zh = z-r;
        x=xh; y=yh; z=zh;
        if (x-y).abs()<2E-15 {break;}
    }
    return x;
}

// m=k*k
pub fn cK(m: f64) -> f64 {
    return 0.5*PI/agm(1.0,(1.0-m).sqrt());
}

// m=k*k
pub fn cE(m: f64) -> f64 {
    let M = agm(1.0,(1.0-m).sqrt());
    let N = magm(1.0,1.0-m);
    return 0.5*PI*N/M;
}

pub fn RF(mut x: f64, mut y: f64, mut z: f64) -> f64 {
    for _ in 0..26 {
        let a = (x*y).sqrt()+(x*z).sqrt()+(y*z).sqrt();
        x=(x+a)/4.0; y=(y+a)/4.0; z=(z+a)/4.0;
    }
    return 1.0/(x).sqrt();
}

pub fn RC(x: f64, y: f64) -> f64 {
    return RF(x,y,y);
}

pub fn RJ(mut x: f64, mut y: f64, mut z: f64, mut p: f64) -> f64 {
    let delta = (p-x)*(p-y)*(p-z);
    let mut s = 0.0;
    let n: i32 = 12;
    for k in 0..n {
        let rx = x.sqrt();
        let ry = y.sqrt();
        let rz = z.sqrt();
        let rp = p.sqrt();

        let a = rx*ry+rx*rz+ry*rz;
        let d = (rp+rx)*(rp+ry)*(rp+rz);
        let e = (4f64).powi(-3*k)/(d*d)*delta;

        x = (x+a)/4.0;
        y = (y+a)/4.0;
        z = (z+a)/4.0;
        p = (p+a)/4.0;
        s += (4f64).powi(-k)/d*RC(1.0,1.0+e);
    }
    return (x).powf(-3.0/2.0)*(4f64).powi(-n)+6.0*s;
}

pub fn RD(x: f64, y: f64, z: f64) -> f64 {
    return RJ(x,y,z,z);
}

pub fn eiF(phi: f64, m: f64) -> f64 {
    let s = (phi).sin();
    let c = (phi).cos();
    return s*RF(c*c,1.0-m*s*s,1.0);
}

pub fn eiE(phi: f64, m: f64) -> f64 {
    let s = (phi).sin();
    let c = (phi).cos();
    let mss = m*s*s;
    return s*RF(c*c,1.0-mss,1.0)-1.0/3.0*mss*s*RJ(c*c,1.0-mss,1.0,1.0);
}

pub fn eiPi(phi: f64,n: f64,m: f64) -> f64 {
    let s = (phi).sin();
    let c = (phi).cos();
    let mss = m*s*s;
    let nss = n*s*s;
    return s*RF(c*c,1.0-mss,1.0)+1.0/3.0*nss*s*RJ(c*c,1.0-mss,1.0,1.0-nss);
}

