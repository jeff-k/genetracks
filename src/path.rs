use crate::{Bp, Point};
use core::fmt::Write;

pub struct PathBuilder {
    path: String,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            path: String::with_capacity(128),
        }
    }

    pub fn close(mut self) -> String {
        let _ = write!(self.path, "Z");
        self.path
    }

    pub fn mv(mut self, point: Point) -> Self {
        let _ = write!(self.path, "M {point} ");
        self
    }

    pub fn line(mut self, point: Point) -> Self {
        let _ = write!(self.path, "L {point} ");
        self
    }

    pub fn arc(mut self, radius: f64, point: Point, large: bool, sweep: bool) -> Self {
        let _ = match (large, sweep) {
            (false, false) => write!(self.path, "A {radius:.1} {radius:.1} 0 0 0 {point} "),
            (false, true) => write!(self.path, "A {radius:.1} {radius:.1} 0 0 1 {point} "),
            (true, false) => write!(self.path, "A {radius:.1} {radius:.1} 0 1 0 {point} "),
            (true, true) => write!(self.path, "A {radius:.1} {radius:.1} 0 1 1 {point} "),
        };
        self
    }

    pub fn curve(mut self, end_control: Point, start_control: Point, start: Point) -> Self {
        let _ = write!(self.path, "C {end_control} {start_control} {start}");
        self
    }
}
