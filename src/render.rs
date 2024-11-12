use crate::ElemStyle;
use crate::Point;
use core::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularCoords {
    pub length: f64,
    pub radius: f64,
    pub height: f64,
    pub center: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearCoords {
    pub origin: Point,
    pub scale: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    Linear(LinearCoords),
    Circular(CircularCoords),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegionStyle {
    Bar,
    Full,
}

impl Layout {
    pub fn label(self, pos: f64) -> Point {
        match self {
            Self::Circular(CircularCoords {
                length,
                radius,
                height: _,
                center,
            }) => {
                let start_angle = (pos / length) * 2.0 * PI - PI / 2.0;
                let end_angle = (pos / length) * 2.0 * PI - PI / 2.0;
                let mid_angle = (start_angle + end_angle) / 2.0;

                let x = center.x + radius * mid_angle.cos();
                let y = center.y + radius * mid_angle.sin();
                Point { x, y }
            }
            _ => Point { x: 0.0, y: 0.0 },
        }
    }
    pub fn draw(self, start: f64, end: f64, style: RegionStyle, decoration: ElemStyle) -> String {
        match self {
            Self::Circular(CircularCoords {
                length,
                radius,
                height,
                center,
            }) => {
                let start_angle = (start / length) * 2.0 * PI - PI / 2.0;
                let end_angle = (end / length) * 2.0 * PI - PI / 2.0;

                let inner_radius = radius - height / 2.0;
                let outer_radius = radius + height / 2.0;
                //let center = radius;

                let mk_point = |radius: f64, angle: f64| Point {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                };

                let (base_start_angle, base_end_angle) = match decoration {
                    ElemStyle::Left => (start_angle + 0.02, end_angle),
                    ElemStyle::Right => (start_angle, end_angle - 0.02),
                    _ => (start_angle, end_angle),
                };

                let start_outer = mk_point(outer_radius, base_start_angle);

                let end_outer = mk_point(outer_radius, base_end_angle);

                let start_inner = mk_point(inner_radius, base_start_angle);

                let end_inner = mk_point(inner_radius, base_end_angle);

                let start_center = mk_point(radius, start_angle);

                let end_center = mk_point(radius, end_angle);

                let peak = match decoration {
                    ElemStyle::Left => mk_point(radius, start_angle),
                    ElemStyle::Right => mk_point(radius, end_angle),
                    _ => Point { x: 0.0, y: 0.0 },
                };

                let large_arc_flag = if end_angle - start_angle <= PI {
                    "0"
                } else {
                    "1"
                };

                let outer_radius = 0.0;
                let inner_radius = 0.0;

                match (style, decoration) {
                    (RegionStyle::Bar, ElemStyle::None) => {
                        format!(
                            "M {start_outer} \
                            L {start_inner} \
                            M {start_center} \
                            A {center} {center} 0 {large_arc_flag} 1 {end_center} \
                            M {end_outer} \
                            L {end_inner}"
                        )
                    }

                    (RegionStyle::Bar, ElemStyle::Left) => {
                        format!(
                            "M {start_outer} \
                            L {peak} \
                            L {start_inner} \
                            M {start_center} \
                            A {center} {center} 0 {large_arc_flag} 1 {end_center} \
                            M {end_outer} \
                            L {end_inner}"
                        )
                    }
                    (RegionStyle::Bar, ElemStyle::Right) => {
                        format!(
                            "M {start_outer} \
                            L {start_inner} \
                            M {start_center} \
                            A {center} {center} 0 {large_arc_flag} 1 {end_center} \
                            M {end_outer} \
                            L {peak} \
                            L {end_inner}"
                        )
                    } //self::circular(coords) => view! { <g></g> },

                    (RegionStyle::Full, ElemStyle::None) => {
                        format!(
                            "M {start_outer} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} \
                            L {end_inner} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Left) => {
                        format!(
                            "M {start_outer} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} \
                            L {end_inner} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} \
                            L {peak} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Right) => {
                        format!(
                            "M {start_outer} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} \
                            L {peak} \
                            L {end_inner} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} \
                            Z"
                        )
                    }
                }
            }
            // Linear
            Self::Linear(LinearCoords {
                origin,
                scale,
                height,
            }) => {
                let start_x = (origin.x + start) * scale;
                let end_x = (origin.x + end) * scale;
                let y = origin.y;

                let base_top = Point { x: start_x, y };
                let base_bottom = Point {
                    x: start_x,
                    y: y + height,
                };
                let end_top = Point { x: end_x, y };
                let end_bottom = Point {
                    x: end_x,
                    y: y + height,
                };

                let bottom_y = 0.0;
                let top_y = 0.0;
                let center_y = 0.0;
                let peak_y = 0.0;
                let peak_x = 0.0;
                let start_x_plus = 0.0;
                let end_x_minus = 0.0;
                let base_bottom_y = 0.0;
                let bar_end = 0.0;
                let peak = 0.0;
                let base_y = 0.0;
                let bar_start = 0.0;

                match (style, decoration) {
                    (RegionStyle::Bar, ElemStyle::None) => {
                        format!(
                            "M {start_x},{top_y} \
                            L {start_x},{bottom_y} \
                            M {start_x},{center_y} \
                            H {end_x} \
                            M {end_x},{top_y} \
                            L {end_x},{bottom_y}"
                        )
                    }

                    (RegionStyle::Bar, ElemStyle::Left) => {
                        format!(
                            "M {start_x_plus},{top_y} \
                            L {peak_x} {peak_y} \
                            L {start_x_plus},{bottom_y} \
                            M {start_x},{center_y} \
                            H {end_x} \
                            M {end_x},{top_y} \
                            L {end_x},{bottom_y}"
                        )
                    }
                    (RegionStyle::Bar, ElemStyle::Right) => {
                        format!(
                            "M {start_x},{top_y} \
                            L {start_x},{bottom_y} \
                            M {start_x},{center_y} \
                            H {end_x} \
                            M {end_x_minus},{top_y} \
                            L {peak_x} {peak_y} \
                            L {end_x_minus},{bottom_y}"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::None) => {
                        format!(
                            "M {base_top} \
                            H {end_x} \
                            V {base_bottom_y} \
                            H {start_x} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Left) => {
                        format!(
                            "M {bar_start},{y} \
                            H {end_x} \
                            V {base_y} \
                            H {bar_start} \
                            L {peak} \
                            Z",
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Right) => {
                        format!(
                            "M {base_top} \
                            H {bar_end} \
                            L {peak} \
                            L {end_bottom} \
                            H {start_x} \
                            Z"
                        )
                    }
                }
            }
        }
    }
}
