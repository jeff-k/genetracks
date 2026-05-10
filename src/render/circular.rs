use crate::ElemStyle;
use crate::components::FigCx;
use crate::path::PathBuilder;
use crate::render::Layout;
use crate::{Bp, Point};
use core::f64::consts::{PI, TAU};

const HALF_PI: f64 = PI / 2.0;

struct AnchorCtx<'a> {
    cc: &'a CircularCoords,

    start_angle: f64,
    end_angle: f64,
    inner_radius: f64,
    outer_radius: f64,
    mid_radius: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularCoords {
    pub length: f64,
    pub radius: f64,
    pub height: f64,
    pub center: Point,
}
impl Layout for CircularCoords {
    fn draw_ribbon(&self, start: (u32, u32), end: (u32, u32), _target: Option<u32>) -> String {
        let (from_start, from_end) = end;
        let (to_start, to_end) = start;

        let from_start_angle = (f64::from(from_start) / self.length) * TAU - HALF_PI;
        let from_end_angle = (f64::from(from_end) / self.length) * TAU - HALF_PI;
        let to_start_angle = (f64::from(to_start) / self.length) * TAU - HALF_PI;
        let to_end_angle = (f64::from(to_end) / self.length) * TAU - HALF_PI;

        let mk_point = |radius: f64, angle: f64| Point {
            x: self.center.x + radius * angle.cos(),
            y: self.center.y + radius * angle.sin(),
        };

        let radius = self.radius;
        let control = radius * 0.5;

        let from_start_point = mk_point(radius, from_start_angle);

        let from_end_point = mk_point(radius, from_end_angle);

        let to_start_point = mk_point(radius, to_start_angle);

        let to_end_point = mk_point(radius, to_end_angle);

        let from_start_control = mk_point(control, from_start_angle);

        let from_end_control = mk_point(control, from_end_angle);

        let to_start_control = mk_point(control, to_start_angle);

        let to_end_control = mk_point(control, to_end_angle);

        let from_large_arc = (from_end_angle - from_start_angle).rem_euclid(TAU) > PI;
        let to_large_arc = (to_end_angle - to_start_angle).rem_euclid(TAU) > PI;

        /*
        let from_large_arc_flag = if (from_end_angle - from_start_angle).rem_euclid(TAU) > PI {
            "1"
        } else {
            "0"
        };
        let to_large_arc_flag = if (to_end_angle - to_start_angle).rem_euclid(TAU) > PI {
            "1"
        } else {
            "0"
        };
        */

        PathBuilder::new()
            .mv(from_start_point)
            .arc(radius, from_end_point, from_large_arc, true)
            .curve(from_end_control, to_start_control, to_start_point)
            .arc(radius, to_end_point, to_large_arc, true)
            .curve(to_end_control, from_start_control, from_start_point)
            .close()
        /*        format!(
            "M {from_start_point} \
        A {radius} {radius} 0 {from_large_arc} 1 {from_end_point} \
        C {from_end_control} {to_start_control} {to_start_point} \
        A {radius} {radius} 0 {to_large_arc} 1 {to_end_point} \
        C {to_end_control} {from_start_control} {from_start_point} \
        Z",
        )
        */
    }
    fn update(&mut self, cx: &FigCx, index: f64) {
        let FigCx {
            width,
            length,
            height: _,
            track_height,
            view: _,
            center,
            scale: _,
            circular: _,
        } = cx;

        self.length = *length;
        self.radius = (width / 2.0) - (index * track_height);
        self.height = *track_height;
        self.center = *center;
    }
    fn map_pos(&self, pos: u32) -> Point {
        let mid_angle = (f64::from(pos) / self.length) * TAU - HALF_PI;

        let radius = self.radius - (self.height / 2.0);

        let x = self.center.x + radius * mid_angle.cos();
        let y = self.center.y + radius * mid_angle.sin();
        Point { x, y }
    }

    fn draw_bar(&self, start: u32, end: u32, decoration: ElemStyle) -> String {
        let radius = self.radius;
        let length = self.length;
        let height = self.height;
        let arrow_head = height * 0.5;

        let start_angle = (f64::from(start) / length) * TAU - HALF_PI;
        let end_angle = (f64::from(end) / length) * TAU - HALF_PI;

        let inner_radius = self.radius - height;
        let mid_radius = self.radius - (height / 2.0);
        let outer_radius = self.radius;

        let mk_point = |radius: f64, angle: f64| Point {
            x: self.center.x + radius * angle.cos(),
            y: self.center.y + radius * angle.sin(),
        };

        let start_top = mk_point(outer_radius, start_angle);
        let start_mid = mk_point(mid_radius, start_angle);
        let start_bottom = mk_point(inner_radius, start_angle);

        let end_top = mk_point(outer_radius, end_angle);
        let end_mid = mk_point(mid_radius, end_angle);
        let end_bottom = mk_point(inner_radius, end_angle);

        let start_base_top = mk_point(outer_radius, start_angle + 0.02);
        let start_base_bottom = mk_point(inner_radius, start_angle + 0.02);

        let start_wide_base_top = mk_point(outer_radius - arrow_head, start_angle + 0.02);
        let start_wide_base_bottom = mk_point(inner_radius - arrow_head, start_angle + 0.02);

        let end_base_top = mk_point(outer_radius, end_angle - 0.02);
        let end_base_bottom = mk_point(inner_radius, end_angle - 0.02);

        let end_wide_base_top = mk_point(outer_radius + arrow_head, end_angle - 0.02);
        let end_wide_base_bottom = mk_point(inner_radius - arrow_head, end_angle - 0.02);

        let large_arc_flag = if (end_angle - start_angle).rem_euclid(TAU) <= PI {
            "0"
        } else {
            "1"
        };

        match decoration {
            ElemStyle::None => {
                format!(
                    "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                )
            }

            ElemStyle::Line => {
                format!(
                    "M {start_mid} A {inner_radius} {inner_radius} 0 {large_arc_flag} 1 {end_mid}"
                )
            }
            ElemStyle::DoubleLine => {
                format!(
                    "M {start_mid} A {mid_radius} {mid_radius} 0 {large_arc_flag} 1 {end_mid} M {start_bottom} A {inner_radius} {inner_radius} 0 {large_arc_flag} 1 {end_bottom}"
                )
            }
            ElemStyle::Left => {
                if end_angle - start_angle <= 0.02 {
                    format!(
                        "M {start_base_top} \
                            L {start_mid} \
                            L {start_base_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid}"
                    )
                } else {
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
            }
            ElemStyle::Right => {
                if end_angle - start_angle <= 0.02 {
                    format!(
                        "M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom}"
                    )
                } else {
                    format!(
                        "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom}"
                    )
                }
            }
            ElemStyle::ArrowLeft => {
                if end_angle - start_angle <= 0.02 {
                    format!(
                        "M {start_wide_base_top} \
                            L {start_mid} \
                            L {start_wide_base_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid}"
                    )
                } else {
                    format!(
                        "M {start_wide_base_top} \
                            L {start_mid} \
                            L {start_wide_base_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                    )
                }
            }
            ElemStyle::ArrowRight => {
                if end_angle - start_angle <= 0.02 {
                    format!(
                        "M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom}"
                    )
                } else {
                    format!(
                        "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            A {radius} {radius} 0 {large_arc_flag} 1 {end_mid} \
                            M {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom}"
                    )
                }
            }
            ElemStyle::Bar | ElemStyle::BarLeft | ElemStyle::BarRight => todo!(),
        }
    }

    fn draw_filled(&self, start: u32, end: u32, decoration: ElemStyle) -> String {
        let length = self.length;
        let height = self.height;
        let arrow_head = self.height * 0.5;

        let start = f64::from(start);
        let end = f64::from(end);

        let start_angle = (start / length) * TAU - HALF_PI;
        //        let end_angle = (f64::from(end) / length) * TAU - HALF_PI;

        let full_circle = start == 0.0 && end == length;
        let end_mod = if end == length { 0.0 } else { end };

        let span_bp: f64 = if full_circle {
            length
        } else if end_mod >= start {
            end_mod - start
        } else {
            length - start + end_mod
        };

        let span_angle = (span_bp / length) * TAU;
        let end_angle = start_angle + span_angle;

        let inner_radius = self.radius - height;
        let mid_radius = self.radius - (height / 2.0);
        let outer_radius = self.radius;

        let mk_point = |radius: f64, angle: f64| Point {
            x: self.center.x + radius * angle.cos(),
            y: self.center.y + radius * angle.sin(),
        };

        let start_top = mk_point(outer_radius, start_angle);
        let start_mid = mk_point(mid_radius, start_angle);
        let start_bottom = mk_point(inner_radius, start_angle);

        let end_top = mk_point(outer_radius, end_angle);
        let end_mid = mk_point(mid_radius, end_angle);
        let end_bottom = mk_point(inner_radius, end_angle);

        let start_base_top = mk_point(outer_radius, start_angle + 0.02);
        let start_base_bottom = mk_point(inner_radius, start_angle + 0.02);

        let start_wide_base_top = mk_point(outer_radius + arrow_head, start_angle + 0.02);
        let start_wide_base_bottom = mk_point(inner_radius - arrow_head, start_angle + 0.02);

        let end_base_top = mk_point(outer_radius, end_angle - 0.02);
        let end_base_bottom = mk_point(inner_radius, end_angle - 0.02);

        let end_wide_base_top = mk_point(outer_radius + arrow_head, end_angle - 0.02);
        let end_wide_base_bottom = mk_point(inner_radius - arrow_head, end_angle - 0.02);

        let large_arc = span_angle > PI;

        let large_arc_flag = if large_arc { "1" } else { "0" };

        let is_tiny = span_angle <= 0.02;

        match decoration {
            ElemStyle::None => PathBuilder::new()
                .mv(start_top)
                .arc(outer_radius, end_top, large_arc, true)
                .line(end_bottom)
                .arc(inner_radius, start_bottom, large_arc, false)
                .close(),
            ElemStyle::Line => PathBuilder::new()
                .mv(start_mid)
                .arc(inner_radius, end_mid, large_arc, false)
                .close(),
            ElemStyle::DoubleLine => {
                unimplemented!()
            }
            ElemStyle::Left => {
                if is_tiny {
                    PathBuilder::new()
                        .mv(start_mid)
                        .line(start_base_top)
                        .arc(inner_radius, start_base_bottom, large_arc, false)
                        .line(start_mid)
                        .close()
                } else {
                    PathBuilder::new()
                        .mv(start_mid)
                        .line(start_base_top)
                        .arc(outer_radius, end_top, large_arc, true)
                        .line(end_bottom)
                        .arc(inner_radius, start_base_bottom, large_arc, false)
                        .line(start_mid)
                        .close()
                    /*                    format!(
                        "M {start_mid} \
                            L {start_base_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_top} \
                            L {end_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_base_bottom} \
                            L {start_mid} \
                            Z"
                    )*/
                }
            }

            ElemStyle::Right => {
                if end_angle - start_angle <= 0.02 {
                    PathBuilder::new()
                        .mv(end_mid)
                        .line(end_base_top)
                        .arc(outer_radius, end_base_bottom, large_arc, true)
                        .line(end_mid)
                        .close()

                    /*
                    format!(
                        "M {end_mid} \
                                L {end_base_top}
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_base_bottom} \
                            L {end_mid} \
                            Z"
                    )
                    */
                } else {
                    PathBuilder::new()
                        .mv(start_top)
                        .arc(outer_radius, end_base_top, large_arc, true)
                        .line(end_mid)
                        .line(end_base_bottom)
                        .arc(inner_radius, start_bottom, large_arc, false)
                        .line(start_top)
                        .close()
                    /*format!(
                        "M {start_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_bottom} \
                            L {start_top} \
                            Z"
                    )*/
                }
            }
            ElemStyle::ArrowLeft => {
                if end_angle - start_angle <= 0.02 {
                    PathBuilder::new()
                        .mv(start_mid)
                        .line(start_wide_base_top)
                        .arc(inner_radius, start_wide_base_bottom, large_arc, false)
                        .line(start_mid)
                        .close()
                } else {
                    PathBuilder::new()
                        .mv(start_mid)
                        .line(start_wide_base_top)
                        .line(start_base_top)
                        .arc(outer_radius, end_top, large_arc, true)
                        .line(end_bottom)
                        .arc(inner_radius, start_base_bottom, large_arc, false)
                        .line(start_wide_base_bottom)
                        .line(start_mid)
                        .close()
                    /*
                    format!(
                        "M {start_mid} \
                            L {start_wide_base_top} \
                            L {start_base_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_top} \
                            L {end_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_base_bottom} \
                            L {start_wide_base_bottom} \
                            L {start_mid} \
                            Z"
                    )
                    */
                }
            }
            ElemStyle::ArrowRight => {
                if end_angle - start_angle <= 0.02 {
                    format!(
                        "M {end_mid} \
                            L {end_wide_base_top}
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_wide_base_bottom} \
                            L {end_mid} \
                            Z"
                    )
                } else {
                    format!(
                        "M {start_top} \
                            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_base_top} \
                            L {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom} \
                            L {end_base_bottom} \
                            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_bottom} \
                            L {start_top} \
                            Z"
                    )
                }
            }
            ElemStyle::Bar | ElemStyle::BarLeft | ElemStyle::BarRight => todo!(),
        }
    }
    fn draw_tick(&self, pos: u32) -> String {
        let center = self.center;
        let length = self.length;
        let radius = self.radius;
        let height = self.height;
        let pos = f64::from(pos);

        let mid_angle = (pos / length) * TAU - HALF_PI;
        let outer_radius = radius;
        let mid_radius = radius - (height / 2.0);

        let start = Point {
            x: center.x + mid_radius * mid_angle.cos(),
            y: center.y + mid_radius * mid_angle.sin(),
        };
        let end = Point {
            x: center.x + outer_radius * mid_angle.cos(),
            y: center.y + outer_radius * mid_angle.sin(),
        };

        PathBuilder::new().mv(start).line(end).close()
        //        format!("M {start} L {end}")
    }
}

pub fn draw_sector(
    origin: Point,
    length: f64,
    start: f64,
    end: f64,
    outer_radius: f64,
    inner_radius: f64,
) -> String {
    let start_angle = (start / length) * TAU - HALF_PI;
    let end_angle = (end / length) * TAU - HALF_PI;

    let start_top = Point {
        x: origin.x + outer_radius * start_angle.cos(),
        y: origin.y + outer_radius * start_angle.sin(),
    };

    let end_top = Point {
        x: origin.x + outer_radius * end_angle.cos(),
        y: origin.y + outer_radius * end_angle.sin(),
    };

    let start_bottom = Point {
        x: origin.x + inner_radius * start_angle.cos(),
        y: origin.y + inner_radius * start_angle.sin(),
    };

    let end_bottom = Point {
        x: origin.x + inner_radius * end_angle.cos(),
        y: origin.y + inner_radius * end_angle.sin(),
    };

    let large_arc = (end_angle - start_angle).rem_euclid(TAU) <= PI;

    //    let large_arc_flag = if large_arc { "0" } else { "1" };

    PathBuilder::new()
        .mv(start_top)
        .arc(outer_radius, end_top, large_arc, true)
        .line(end_bottom)
        .arc(inner_radius, start_bottom, large_arc, false)
        .close()
    /*    format!(
        "M {start_top} \
        A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_top} \
        L {end_bottom} \
        A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_bottom} \
        Z"
    )
    */
}

pub fn svg_arc_path(cc: &CircularCoords, pos: u32, arc_span: f64) -> String {
    let radius = cc.radius - (cc.height / 2.0);

    let angle = (f64::from(pos) / cc.length) * TAU - HALF_PI;
    let span = (arc_span / cc.length) * TAU;
    let start_angle = angle - span / 2.0;
    let end_angle = angle + span / 2.0;

    let (start_x, start_y) = (
        cc.center.x + radius * start_angle.cos(),
        cc.center.y + radius * start_angle.sin(),
    );

    let (end_x, end_y) = (
        cc.center.x + radius * end_angle.cos(),
        cc.center.y + radius * end_angle.sin(),
    );

    let delta = (end_angle - start_angle).rem_euclid(TAU);

    let large_arc = if delta > PI { "1" } else { "0" };

    if angle > 0.0 && angle < PI {
        format!("M {end_x} {end_y} A {radius} {radius} 0 {large_arc} 0 {start_x} {start_y}")
    } else {
        format!("M {start_x} {start_y} A {radius} {radius} 0 {large_arc} 1 {end_x} {end_y}")
    }
}
