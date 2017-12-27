
extern crate lm_analysis;
extern crate lm_sf;
extern crate lm_plot;

pub mod quad {
    pub use lm_analysis::quad::*;
}
pub mod diff {
    pub use lm_analysis::diff::*;
}
pub mod interpolation {
    pub use lm_analysis::interpolation::*;
}
pub mod ode {
    pub use lm_analysis::ode::*;
}

pub mod sf {
    pub use lm_sf::*;
}

pub mod plot {
    pub use lm_plot::*;
}
