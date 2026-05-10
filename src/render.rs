mod circular;
mod linear;

use crate::ElemStyle;
use crate::Point;
use crate::components::FigCx;
pub use circular::CircularCoords;
pub use circular::draw_sector;
pub use circular::svg_arc_path;
pub use linear::LinearCoords;

#[derive(PartialEq)]
pub enum LayoutWrapper {
    Linear(LinearCoords),
    Circular(CircularCoords),
}

pub trait Layout: Sync + Send + PartialEq {
    fn map_pos(&self, pos: u32) -> Point;
    fn draw_bar(&self, start: u32, end: u32, style: ElemStyle) -> String;
    fn draw_filled(&self, start: u32, end: u32, style: ElemStyle) -> String;
    fn draw_tick(&self, pos: u32) -> String;
    fn update(&mut self, cx: &FigCx, index: f64);
    fn draw_ribbon(&self, start: (u32, u32), end: (u32, u32), target: Option<u32>) -> String;
    fn draw_coverage(&self, data: &[(u32, u32, f32)], max_len: f32) -> String;
}

impl Layout for LayoutWrapper {
    fn map_pos(&self, pos: u32) -> Point {
        match self {
            LayoutWrapper::Linear(l) => l.map_pos(pos),
            LayoutWrapper::Circular(l) => l.map_pos(pos),
        }
    }
    fn draw_bar(&self, start: u32, end: u32, style: ElemStyle) -> String {
        match self {
            LayoutWrapper::Linear(l) => l.draw_bar(start, end, style),
            LayoutWrapper::Circular(l) => l.draw_bar(start, end, style),
        }
    }
    fn draw_filled(&self, start: u32, end: u32, style: ElemStyle) -> String {
        match self {
            LayoutWrapper::Linear(l) => l.draw_filled(start, end, style),
            LayoutWrapper::Circular(l) => l.draw_filled(start, end, style),
        }
    }
    fn draw_tick(&self, pos: u32) -> String {
        match self {
            LayoutWrapper::Linear(l) => l.draw_tick(pos),
            LayoutWrapper::Circular(l) => l.draw_tick(pos),
        }
    }
    fn update(&mut self, cx: &FigCx, index: f64) {
        match self {
            LayoutWrapper::Linear(l) => l.update(cx, index),
            LayoutWrapper::Circular(l) => l.update(cx, index),
        }
    }
    fn draw_ribbon(&self, start: (u32, u32), end: (u32, u32), target: Option<u32>) -> String {
        match self {
            LayoutWrapper::Linear(l) => l.draw_ribbon(start, end, target),
            LayoutWrapper::Circular(l) => l.draw_ribbon(start, end, target),
        }
    }

    fn draw_coverage(&self, data: &[(u32, u32, f32)], max_len: f32) -> String {
        match self {
            LayoutWrapper::Linear(l) => l.draw_coverage(data, max_len),
            LayoutWrapper::Circular(_) => unimplemented!(),
        }
    }
}

pub fn draw_highlight(start: f64, end: f64, top: f64, bottom: f64) -> String {
    let start_top = Point { x: start, y: top };
    let end_top = Point { x: end, y: top };
    let start_bottom = Point {
        x: start,
        y: bottom,
    };
    let end_bottom = Point { x: end, y: bottom };
    format!(
        "M {start_top} \
                            L {end_top} \
                            L {end_bottom} \
                            L {start_bottom} \
                           Z"
    )
}
