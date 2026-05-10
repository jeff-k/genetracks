#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
//#![allow(unused_variables)]
//#![allow(clippy::needless_pass_by_value)]

pub use core::fmt;
pub mod components;
pub mod elements;
pub(crate) mod path;
mod render;
pub mod slider;

pub use elements::{ElemStyle, ElementData, TrackData};
pub use render::{Layout, LayoutWrapper};

/// Basepair coordinate
pub type Bp = u32;

pub struct Ivl {
    start: Bp,
    end: Bp,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn to_coord(self) -> Bp {
        self.x.max(0.0).round() as u32
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1},{:.1}", self.x, self.y)
    }
}
