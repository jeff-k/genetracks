#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(unused_variables)]

pub use core::fmt;
pub mod components;
pub mod elements;
mod render;

pub use elements::{ElemStyle, ElementData, TrackData};
pub use render::{Layout, LayoutWrapper};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
