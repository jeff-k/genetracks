#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub use core::fmt;
pub mod components;
pub mod elements;
mod render;

pub use elements::{ElemStyle, ElementData, TrackData};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
