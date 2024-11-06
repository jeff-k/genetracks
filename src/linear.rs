pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use crate::Point;
//use core::fmt;
use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct FigCx {
    length: f64,
    width: f64,
    track_spacing: f64,
    track_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TrackCx {
    length: f64,
    width: f64,
    center: Point,
    track_spacing: f64,
    track_height: f64,
}

#[component]
pub fn Figure(#[prop(into)] length: Signal<u32>, children: Children) -> impl IntoView {
    let cx = create_memo(move |_| FigCx {
        length: length() as f64,
        width: 700.0,
        track_spacing: 10.0,
        track_height: 12.0,
    });
    provide_context(cx);

    view! {
        {move || format!("len {}", length())}
        <svg viewBox="0 0 800 400">{children()}</svg>
    }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: Children) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let track_cx = create_memo(move |_| {
        let fig = cx();
        TrackCx {
            length: fig.length,
            width: fig.width,
            center: Point {
                x: fig.width / 2.0,
                y: index as f64 * fig.track_height,
            },
            track_spacing: fig.track_spacing,
            track_height: fig.track_height,
        }
    });

    provide_context(track_cx);

    view! { <g>{children()}</g> }
}

#[component]
pub fn Region(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
    #[prop(optional)] style: ElemStyle,
) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("Region must be child of Track");

    let start = start as f64;
    let end = end as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        (track.center.x, track.center.y)
    });

    let path = create_memo(move |_| {
        let track = parent();

        let origin = track.center;
        let start_x = origin.x;
        let end_x = end;
        let end_y = origin.y + track.track_height;
        let peak_x = end + 10.0;

        match style {
            ElemStyle::Rect => format!("M {origin} H {end_x} V {end_y} H {start_x} Z "),
            ElemStyle::Right => {
                format!("M {origin} L {peak_x} {end_y} L {end_x} {end_y} V {end_y} Z")
            }
            ElemStyle::Left => {
                format!("M {origin} H {peak_x} V {peak_x} L {peak_x} {peak_x} V {peak_x} Z")
            }
            _ => format!(""),
        }
    });

    view! {
        <>
            <path d=path stroke=color.to_string() stroke_width="2" fill="none" />
            {label
                .map(|text| {
                    view! {
                        <text
                            x=move || text_pos().0
                            y=move || text_pos().1
                            text-anchor="middle"
                            dominant-baseline="middle"
                            fill="black"
                            font-size="smaller"
                            font-family="monospace"
                        >
                            {text}
                        </text>
                    }
                })}
        </>
    }
}

#[component]
pub fn Label(
    #[prop(into)] pos: u32,
    #[prop(optional)] label: String,
    #[prop(optional)] color: Colour,
) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("Region must be child of Track");

    let pos = pos as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        Point {
            x: pos,
            y: track.center.y,
        }
    });

    view! {
        <text
            x=move || text_pos().x
            y=move || text_pos().y
            text-anchor="middle"
            dominant-baseline="middle"
            fill=color.to_string()
            font-size="smaller"
            font-family="monospace"
        >
            {label}
        </text>
    }
}

#[component]
pub fn Tick(#[prop(into)] pos: u32, #[prop(optional)] label: Option<String>) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("must have track as parent");

    let pos = pos as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        Point {
            x: pos,
            y: track.center.y,
        }
    });

    let path = create_memo(move |_| {
        let track = parent();

        let origin = track.center;
        let end = track.center;

        format!("M {origin} L {end}")
    });

    view! {
        <g>
            <path d=path stroke="black" stroke-width="1" fill="none" />

            {label
                .map(|text| {
                    view! {
                        <text
                            x=move || text_pos().x
                            y=move || text_pos().y
                            text-anchor="middle"
                            dominant-baseline="middle"
                            fill="black"
                            font-size="smaller"
                            font-family="monospace"
                        >
                            {text}
                        </text>
                    }
                })}

        </g>
    }
}

/*
#[component]
pub fn Region(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] style: ElemStyle,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
) -> impl IntoView {
    //    let cx = use_context::<ReadSignal<FigCx>>().expect("Region must be descendent of Figure");
    let parent = use_context::<Memo<TrackCx>>().expect("Region must be child of Track");

    let start = start as f64;
    let end = end as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();

        let start_angle = (start / track.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / track.length) * 2.0 * PI - PI / 2.0;
        let mid_angle = (start_angle + end_angle) / 2.0;

        let x = track.center.x + track.radius * mid_angle.cos();
        let y = track.center.y + track.radius * mid_angle.sin();

        (x, y)
    });

    let path = create_memo(move |_| {
        let track = parent();
        //let radius = parent();

        let start_angle = (start / track.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / track.length) * 2.0 * PI - PI / 2.0;

        let inner_radius = track.radius - track.track_height / 2.0;
        let outer_radius = track.radius + track.track_height / 2.0;

        let center_radius = track.radius;

        let mk_point = |radius: f64, angle: f64| Point {
            x: track.center.x + radius * angle.cos(),
            y: track.center.y + radius * angle.sin(),
        };

        let (base_start_angle, base_end_angle) = match style {
            ElemStyle::Left => (start_angle + 0.03, end_angle),
            ElemStyle::Right => (start_angle, end_angle - 0.03),
            _ => (start_angle, end_angle),
        };

        let start_outer = mk_point(outer_radius, base_start_angle);

        let end_outer = mk_point(outer_radius, base_end_angle);

        let start_inner = mk_point(inner_radius, base_start_angle);

        let end_inner = mk_point(inner_radius, base_end_angle);

        let large_arc_flag = if end_angle - start_angle <= PI {
            "0"
        } else {
            "1"
        };

        let peak = match style {
            ElemStyle::Left => mk_point(center_radius, start_angle),
            ElemStyle::Right => mk_point(center_radius, end_angle),
            _ => Point { x: 0.0, y: 0.0 },
        };

        match style {
            ElemStyle::Rect => {
                format!("M {start_outer} A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} L {end_inner} A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} Z")
            }
            ElemStyle::Left => {
                format!(
                    "M {start_outer} \
            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} \
            L {end_inner} \
            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} \
            L {peak} \
            Z"
                )
            }
            ElemStyle::Right => {
                format!(
                    "M {start_outer} \
            A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer} \
            L {peak} \
            L {end_inner} \
            A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner} \
            Z"
                )
            }
            _ => {
                format!("")
            }
        }
    });

    view! {
        <>
            <path d=path fill=color.to_string() />
            {label
                .map(|text| {
                    view! {
                        <text
                            x=move || text_pos().0
                            y=move || text_pos().1
                            text-anchor="middle"
                            dominant-baseline="middle"
                            fill="black"
                            font-size="smaller"
                            font-family="monospace"
                        >
                            {text}
                        </text>
                    }
                })}
        </>
    }
}
*/
