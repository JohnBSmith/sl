
use std::f64::NAN;

pub fn pli_eq(x0: f64, dx: f64, y: Box<[f64]>) -> Box<Fn(f64)->f64> {
    return Box::new(move |x: f64| -> f64 {
        let k = ((x-x0)/dx) as usize;
        let y1 = match y.get(k) {Some(value)=>*value, None=>return NAN};
        let y2 = match y.get(k+1) {Some(value)=>*value, None=>return NAN};
        let m = (y2-y1)/dx;
        return m*(x-x0-(k as f64)*dx)+y1;
    });
}
