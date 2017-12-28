
extern crate sl_analysis;
extern crate sl_sf;
extern crate sl_plot;

pub mod quad {
    pub use sl_analysis::quad::*;
}
pub mod diff {
    pub use sl_analysis::diff::*;
}
pub mod interpolation {
    pub use sl_analysis::interpolation::*;
}
pub mod ode {
    pub use sl_analysis::ode::*;
}

pub mod sf {
    pub use sl_sf::*;
}

pub mod plot {
    pub use sl_plot::*;
}
