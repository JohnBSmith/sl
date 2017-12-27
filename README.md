
# LibMath, a library of mathematical algorithms

## Analysis package

* `diff` (numerical differentiation)
* `quad` (numerical integration)
* `ode` (ordinary differential equations)
* `interpolation` (obtain functions from data)

## Scientific functions package

* `sf` (scientific functions)
    * `gamma` (gamma and related functions)

## Plotting library

* `Canvas` (a canvas to draw)

## Example

```rust
extern crate lm;

use lm::quad::gauss;
use lm::ode::euler;

// Calculate ln(x) = integral(1 to x) 1/t dt
// by gauss quadrature.
fn ln(x: f64) -> f64 {
    return gauss(&|t| 1.0/t, 1.0, x, 10);
}

fn main() {
	// Calculate exp(x) from the ODE y'=y by the euler method.
    // We have y'=g(x,y), g(x,y)=y.
    let g = |_,y| y;
    let x0 = 0.0;
    let y0 = 1.0;
    let h = 0.001;
    let n = 10000;

    let exp = euler(Box::new(g),x0,y0,h,0,n);

    for i in 1..6 {
       let x = i as f64;
       println!("{:8.4} |{:8.4} |{:8.4} |{:8.4}",x,ln(x),exp(x),exp(ln(x)));
    }
}

// Output:
// 1.0000 |  0.0000 |  2.7169 |  1.0000
// 2.0000 |  0.6931 |  7.3817 |  1.9993
// 3.0000 |  1.0986 | 20.0555 |  2.9984
// 4.0000 |  1.3863 | 54.4891 |  3.9972
// 5.0000 |  1.6094 |148.0428 |  4.9960
```

