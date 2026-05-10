use crate::ElemStyle;
use crate::Layout;
use crate::Point;
use crate::components::FigCx;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearCoords {
    pub origin: Point,
    pub scale: f64,
    pub height: f64,
}

impl Layout for LinearCoords {
    fn draw_ribbon(&self, start: (u32, u32), end: (u32, u32), target: Option<u32>) -> String {
        let bottom = match target {
            Some(track) => f64::from(track) * self.height,
            None => self.height,
        };
        let y = self.origin.y;

        let t_start = f64::from(start.0);
        let t_end = f64::from(start.1);

        let b_start = f64::from(end.0);
        let b_end = f64::from(end.1);

        let start_top = Point { x: t_start, y };

        let start_bottom = Point {
            x: b_start,
            y: y + bottom,
        };

        let end_top = Point { x: t_end, y };

        let end_bottom = Point {
            x: b_end,
            y: y + bottom,
        };
        format!(
            "M {start_top} \
                            L {start_bottom} \
                            L {end_bottom} \
                            L {end_top}"
        )
    }
    fn update(&mut self, cx: &FigCx, index: f64) {
        let FigCx {
            width: _,
            length: _,
            height: _,
            track_height,
            view: _,
            center: _,
            scale,
            circular: _,
        } = cx;

        let y = index * track_height;
        self.origin.y = y; // = Point { x: 0.0, y };
        self.scale = *scale;
        self.height = *track_height;
    }
    fn map_pos(&self, pos: u32) -> Point {
        let x = f64::from(pos);
        let y = self.origin.y + (self.height / 2.0);
        Point { x, y }
    }
    fn draw_bar(&self, start: u32, end: u32, decoration: ElemStyle) -> String {
        let start = f64::from(start);
        let end = f64::from(end);
        let start_x = self.origin.x + start;
        let end_x = self.origin.x + end;
        let y = self.origin.y;
        let height = self.height;
        let arrow = 10.0 / self.scale;
        let arrow_head = height * 0.5;

        let start_base_top = Point {
            x: start_x + arrow,
            y,
        };
        let start_base_bottom = Point {
            x: start_x + arrow,
            y: y + height,
        };

        let start_wide_base_top = Point {
            x: start_x + arrow,
            y: y - arrow_head,
        };

        let start_wide_base_bottom = Point {
            x: start_x + arrow,
            y: y + height + arrow_head,
        };

        let end_base_top = Point {
            x: end_x - arrow,
            y,
        };
        let end_base_bottom = Point {
            x: end_x - arrow,
            y: y + height,
        };

        let end_wide_base_top = Point {
            x: end_x - arrow,
            y: y - arrow_head,
        };

        let end_wide_base_bottom = Point {
            x: end_x - arrow,
            y: y + height + arrow_head,
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

        match decoration {
            ElemStyle::None => {
                format!(
                    "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            L {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                )
            }
            ElemStyle::Line => {
                format!(
                    "M {start_mid} \
                        L {end_mid}"
                )
            }
            ElemStyle::DoubleLine => {
                format!(
                    "M {start_top} \
                        L {end_top} \
                        M {start_bottom} \
                        L {end_bottom}"
                )
            }
            ElemStyle::Left => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_base_top} \
                            L {start_mid} \
                            L {start_base_bottom} \
                            M {start_mid} \
                            L {end_mid} "
                    )
                } else {
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
            }
            ElemStyle::Right => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_mid} \
                            L {end_mid} \
                            M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom}"
                    )
                } else {
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
            }

            ElemStyle::ArrowLeft => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_wide_base_top} \
                            L {start_mid} \
                            L {start_wide_base_bottom} \
                            M {start_mid} \
                            L {end_mid} "
                    )
                } else {
                    format!(
                        "M {start_wide_base_top} \
                            L {start_mid} \
                            L {start_wide_base_bottom} \
                            M {start_mid} \
                            L {end_mid} \
                            M {end_top} \
                            L {end_bottom}"
                    )
                }
            }

            ElemStyle::ArrowRight => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_mid} \
                            L {end_mid} \
                            M {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom}"
                    )
                } else {
                    format!(
                        "M {start_top} \
                            L {start_bottom} \
                            M {start_mid} \
                            L {end_mid} \
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
        let start = f64::from(start);
        let end = f64::from(end);
        let start_x = self.origin.x + start;
        let end_x = self.origin.x + end;
        let y = self.origin.y;
        let height = self.height;
        let arrow = 10.0 / self.scale;
        let arrow_head = height * 0.5;

        let start_base_top = Point {
            x: start_x + arrow,
            y,
        };
        let start_base_bottom = Point {
            x: start_x + arrow,
            y: y + height,
        };

        let start_wide_base_top = Point {
            x: start_x + arrow,
            y: y - arrow_head,
        };

        let start_wide_base_bottom = Point {
            x: start_x + arrow,
            y: y - arrow_head,
        };

        let end_base_top = Point {
            x: end_x - arrow,
            y,
        };
        let end_base_bottom = Point {
            x: end_x - arrow,
            y: y + height,
        };

        let end_wide_base_top = Point {
            x: end_x - arrow,
            y: y - arrow_head,
        };

        let end_wide_base_bottom = Point {
            x: end_x - arrow,
            y: y + height + arrow_head,
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

        match decoration {
            ElemStyle::None => {
                format!(
                    "M {start_top} \
                            L {end_top} \
                            L {end_bottom} \
                            L {start_bottom} \
                            Z"
                )
            }

            ElemStyle::Line => {
                format!("M {start_mid} L {end_mid} Z")
            }
            ElemStyle::DoubleLine => {
                format!("M {start_top} L {end_top} M {start_bottom} L {end_bottom} Z")
            }
            ElemStyle::Left => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_mid} \
                            L {start_base_top} \
                            L {start_base_bottom} \
                            L {start_mid} \
                            Z",
                    )
                } else {
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
            }

            ElemStyle::Right => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {end_base_top} \
                            L {end_mid} \
                            L {end_base_bottom} \
                            L {end_base_top} \
                            Z"
                    )
                } else {
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
            ElemStyle::ArrowLeft => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {start_mid} \
                            L {start_wide_base_top} \
                            L {start_wide_base_bottom} \
                            L {start_mid} \
                            Z",
                    )
                } else {
                    format!(
                        "M {start_mid} \
                            L {start_wide_base_top} \
                            L {start_base_top} \
                            L {end_top} \
                            L {end_bottom} \
                            L {start_base_bottom} \
                            L {start_wide_base_bottom} \
                            L {start_mid} \
                            Z",
                    )
                }
            }
            ElemStyle::ArrowRight => {
                if (end - start) * self.scale <= 10.0 {
                    format!(
                        "M {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom} \
                            L {end_wide_base_top} \
                            Z"
                    )
                } else {
                    format!(
                        "M {start_top} \
                            L {end_base_top} \
                            L {end_wide_base_top} \
                            L {end_mid} \
                            L {end_wide_base_bottom} \
                            L {end_base_bottom} \
                            L {start_bottom} \
                            L {start_top} \
                            Z"
                    )
                }
            }
            ElemStyle::Bar | ElemStyle::BarLeft | ElemStyle::BarRight => todo!(),
        }
    }
    fn draw_tick(&self, pos: u32) -> String {
        let pos = f64::from(pos);
        let start = Point {
            x: pos,
            y: self.origin.y + (self.height / 2.0),
        };

        let end = Point {
            x: pos,
            y: self.origin.y,
        };

        format!("M {start} L {end}")
    }

    fn draw_coverage(&self, data: &[(u32, u32, f32)], max_len: f32) -> String {
        unimplemented!()
    }
}
