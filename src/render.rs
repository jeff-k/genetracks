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

                let inner_radius = radius - height;
                let mid_radius = radius - (height / 2.0);
                let outer_radius = radius;

                let mk_point = |radius: f64, angle: f64| Point {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                };

                let start_top = mk_point(outer_radius, start_angle);
                let start_mid = mk_point(mid_radius, start_angle);
                let start_bottom = mk_point(inner_radius, start_angle);

                let end_top = mk_point(outer_radius, end_angle);
                let end_mid = mk_point(mid_radius, end_angle);
                let end_bottom = mk_point(inner_radius, end_angle);

                let start_base_top = mk_point(outer_radius, start_angle + 0.02);
                let start_base_bottom = mk_point(inner_radius, start_angle + 0.02);
                let end_base_top = mk_point(outer_radius, end_angle - 0.02);
                let end_base_bottom = mk_point(inner_radius, end_angle - 0.02);

                let large_arc_flag = if end_angle - start_angle <= PI {
                    "0"
                } else {
                    "1"
                };

                match (style, decoration) {
                    (RegionStyle::Bar, ElemStyle::None) => {
                        format!(
                            "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                        )
                    }

                    (RegionStyle::Bar, ElemStyle::Left) => {
                        format!(
                            "M {start_base_top} \
                            L {start_mid} \
                            L {start_base_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                        )
                    }
                    (RegionStyle::Bar, ElemStyle::Right) => {
                        format!(
                            "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom}"
                        )
                    } //self::circular(coords) => view! { <g></g> },

                    (RegionStyle::Full, ElemStyle::None) => {
                        format!(
                            "M {start_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_top} \
                            L {end_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_bottom} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Left) => {
                        format!(
                            "M {start_mid} \
                            L {start_base_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_top} \
                            L {end_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_base_bottom} \
                            L {start_mid} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Right) => {
                        format!(
                            "M {start_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_bottom} \
                            L {start_top} \
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

                let start_base_top = Point {
                    x: start_x + 10.0,
                    y,
                };
                let start_base_bottom = Point {
                    x: start_x + 10.0,
                    y: y + height,
                };

                let end_base_top = Point { x: end_x - 10.0, y };
                let end_base_bottom = Point {
                    x: end_x - 10.0,
                    y: y + height,
                };

                let start_top = Point { x: start_x, y };
                let start_mid = Point {
                    x: start_x,
                    y: y + (height / 2.0),
                };
                let start_bottom = Point {
                    x: start_x,
                    y: y + height,
                };

                let end_top = Point { x: end_x, y };
                let end_mid = Point {
                    x: end_x,
                    y: y + (height / 2.0),
                };
                let end_bottom = Point {
                    x: end_x,
                    y: y + height,
                };

                match (style, decoration) {
                    (RegionStyle::Bar, ElemStyle::None) => {
                        format!(
                            "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            L {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                        )
                    }

                    (RegionStyle::Bar, ElemStyle::Left) => {
                        format!(
                            "M {start_base_top} \
                            L {start_mid} \
                            L {start_base_bottom} \
                            M {start_mid} \
                            L {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                        )
                    }
                    (RegionStyle::Bar, ElemStyle::Right) => {
                        format!(
                            "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            L {end_mid} \
                            M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom}"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::None) => {
                        format!(
                            "M {start_top} \
                            L {end_top} \
                            L {end_bottom} \
                            L {start_bottom} \
                            Z"
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Left) => {
                        format!(
                            "M {start_mid} \
                            L {start_base_top} \
                            L {end_top} \
                            L {end_bottom} \
                            L {start_base_bottom} \
                            L {start_mid} \
                            Z",
                        )
                    }

                    (RegionStyle::Full, ElemStyle::Right) => {
                        format!(
                            "M {start_top} \
                            L {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom} \
                            L {start_bottom} \
                            L {start_top} \
                            Z"
                        )
                    }
                }
            }
        }
    }
}
