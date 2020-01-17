
use interpolation::pli_eq;

// m: number of backward steps
// n: number of forward steps
pub fn euler(g: Box<dyn Fn(f64,f64)->f64>, x0: f64, y0: f64, h: f64, m: usize, n: usize)
  -> Box<dyn Fn(f64)->f64>
{
    let mut v: Vec<f64> = Vec::new();
    let mut x=x0;
    let mut y=y0;
    for k in 0..m {
        y = y-h*g(x,y);
        x = x0-(k as f64)*h;
        v.push(y);
    }
    let px = if m==0 {x0} else {x-h};
    v.reverse();
    x=x0;
    y=y0;
    v.push(y);
    for k in 0..n {
        y = y+h*g(x,y);
        x = x0+(k as f64)*h;
        v.push(y);
    }
    return pli_eq(px,h,v.into_boxed_slice());
}
