

pub fn diff(f: &dyn Fn(f64)->f64, x: f64, h: f64) -> f64 {
    return (f(x+h)-f(x-h))/(2.0*h);
}

pub fn diffn(n: u32, f: &dyn Fn(f64)->f64, x: f64, h: f64) -> f64 {
    if n==0 {
        return f(x);
    }else if n==1 {
        return diff(f,x,h);
    }else{
        return (diffn(n-1,f,x+h,h)-diffn(n-1,f,x-h,h))/(2.0*h);
    }
}
