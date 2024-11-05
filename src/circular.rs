pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use core::f64::consts::PI;
use leptos::*;

#[derive(Clone, Copy, Debug)]
struct FigCx {
    length: f64,
    center_x: f64,
    center_y: f64,
    base_radius: f64,
    track_spacing: f64,
    track_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TrackCx {
    length: f64,
    center_x: f64,
    center_y: f64,
    radius: f64,
    track_spacing: f64,
    track_height: f64,
}

#[component]
pub fn Figure(#[prop(into)] length: f64, children: Children) -> impl IntoView {
    let (cx, set_cx) = create_signal(FigCx {
        length: 4323.0,
        center_x: 400.0,
        center_y: 400.0,
        base_radius: 200.0,
        track_spacing: 10.0,
        track_height: 12.0,
    });
    provide_context(cx);

    view! { <svg viewBox="0 0 800 800">{children()}</svg> }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: Children) -> impl IntoView {
    let cx = use_context::<ReadSignal<FigCx>>().expect("Track must be descendent of Figure");

    let track_radius = create_memo(move |_| {
        let fig = cx();
        TrackCx {
            length: fig.length,
            center_x: fig.center_x,
            center_y: fig.center_y,
            radius: fig.base_radius + (index as f64 * fig.track_spacing),
            track_spacing: fig.track_spacing,
            track_height: fig.track_height,
        }
    });

    provide_context(track_radius);

    view! { <g>{children()}</g> }
}

#[component]
pub fn Bar(
    #[prop(into)] start: f64,
    #[prop(into)] end: f64,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("Region must be child of Track");

    let text_pos = create_memo(move |_| {
        let fig = parent();

        let start_angle = (start / fig.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / fig.length) * 2.0 * PI - PI / 2.0;
        let mid_angle = (start_angle + end_angle) / 2.0;

        let x = fig.center_x + fig.radius * mid_angle.cos();
        let y = fig.center_y + fig.radius * mid_angle.sin();

        (x, y)
    });

    let path = create_memo(move |_| {
        let fig = parent();
        //let radius = parent();

        let start_angle = (start / fig.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / fig.length) * 2.0 * PI - PI / 2.0;

        let inner_radius = fig.radius - fig.track_height / 2.0;
        let outer_radius = fig.radius + fig.track_height / 2.0;

        let (start_outer_x, start_outer_y) = (
            fig.center_x + outer_radius * start_angle.cos(),
            fig.center_y + outer_radius * start_angle.sin(),
        );

        let (end_outer_x, end_outer_y) = (
            fig.center_x + outer_radius * end_angle.cos(),
            fig.center_y + outer_radius * end_angle.sin(),
        );

        let (start_inner_x, start_inner_y) = (
            fig.center_x + inner_radius * start_angle.cos(),
            fig.center_y + inner_radius * start_angle.sin(),
        );

        let (end_inner_x, end_inner_y) = (
            fig.center_x + inner_radius * end_angle.cos(),
            fig.center_y + inner_radius * end_angle.sin(),
        );

        let large_arc_flag = if end_angle - start_angle <= PI {
            "0"
        } else {
            "1"
        };

        let centre = fig.radius;

        let (start_center_x, start_center_y) = (
            fig.center_x + centre * start_angle.cos(),
            fig.center_y + centre * start_angle.sin(),
        );

        let (end_center_x, end_center_y) = (
            fig.center_x + centre * end_angle.cos(),
            fig.center_y + centre * end_angle.sin(),
        );

        format!("M {start_outer_x},{start_outer_y} L {start_inner_x},{start_inner_y} M {start_center_x},{start_center_y} A {centre} {centre} 0 {large_arc_flag} 1 {end_center_x},{end_center_y} M {end_outer_x},{end_outer_y} L {end_inner_x},{end_inner_y}")
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
pub fn Tick(#[prop(into)] pos: f64, #[prop(optional)] label: Option<String>) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("must have track as parent");

    let text_pos = create_memo(move |_| {
        let track = parent();

        let angle = (pos / track.length) * 2.0 * PI - PI / 2.0;
        let text_radius = track.radius + track.track_height + 30.0;

        let x = track.center_x + text_radius * angle.cos();
        let y = track.center_y + text_radius * angle.sin();

        (x, y)
    });

    let path = create_memo(move |_| {
        let track = parent();

        let angle = (pos / track.length) * 2.0 * PI - PI / 2.0;

        let inner_radius = track.radius - track.track_height / 2.0;
        let outer_radius = track.radius + track.track_height + 20.0;

        let (inner_x, inner_y) = (
            track.center_x + inner_radius * angle.cos(),
            track.center_y + inner_radius * angle.sin(),
        );

        let (outer_x, outer_y) = (
            track.center_x + outer_radius * angle.cos(),
            track.center_y + outer_radius * angle.sin(),
        );

        format!("M {inner_x},{inner_y} L {outer_x},{outer_y}")
    });

    view! {
        <g style="z-index: 10000">
            <path d=path stroke="black" stroke-width="1" fill="none" />

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

        </g>
    }
}

#[component]
pub fn Region(
    #[prop(into)] start: f64,
    #[prop(into)] end: f64,
    #[prop(optional)] style: ElemStyle,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
) -> impl IntoView {
    //    let cx = use_context::<ReadSignal<FigCx>>().expect("Region must be descendent of Figure");
    let parent = use_context::<Memo<TrackCx>>().expect("Region must be child of Track");

    let text_pos = create_memo(move |_| {
        let fig = parent();

        let start_angle = (start / fig.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / fig.length) * 2.0 * PI - PI / 2.0;
        let mid_angle = (start_angle + end_angle) / 2.0;

        let x = fig.center_x + fig.radius * mid_angle.cos();
        let y = fig.center_y + fig.radius * mid_angle.sin();

        (x, y)
    });

    let path = create_memo(move |_| {
        let fig = parent();
        //let radius = parent();

        let start_angle = (start / fig.length) * 2.0 * PI - PI / 2.0;
        let end_angle = (end / fig.length) * 2.0 * PI - PI / 2.0;

        let inner_radius = fig.radius - fig.track_height / 2.0;
        let outer_radius = fig.radius + fig.track_height / 2.0;

        let (start_outer_x, start_outer_y) = (
            fig.center_x + outer_radius * start_angle.cos(),
            fig.center_y + outer_radius * start_angle.sin(),
        );

        let (end_outer_x, end_outer_y) = (
            fig.center_x + outer_radius * end_angle.cos(),
            fig.center_y + outer_radius * end_angle.sin(),
        );

        let (start_inner_x, start_inner_y) = (
            fig.center_x + inner_radius * start_angle.cos(),
            fig.center_y + inner_radius * start_angle.sin(),
        );

        let (end_inner_x, end_inner_y) = (
            fig.center_x + inner_radius * end_angle.cos(),
            fig.center_y + inner_radius * end_angle.sin(),
        );

        let large_arc_flag = if end_angle - start_angle <= PI {
            "0"
        } else {
            "1"
        };

        match style {
            ElemStyle::Rect => {
                format!("M {start_outer_x},{start_outer_y} A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer_x}, {end_outer_y} L {end_inner_x},{end_inner_y} A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner_x},{start_inner_y} Z")
            }
            ElemStyle::Left => {
                let peak_angle = start_angle - 0.03;
                let (peak_point_x, peak_point_y) = (
                    fig.center_x + fig.radius * peak_angle.cos(),
                    fig.center_y + fig.radius * peak_angle.sin(),
                );

                format!("M {start_outer_x},{start_outer_y} A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer_x}, {end_outer_y} L {end_inner_x},{end_inner_y} A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner_x},{start_inner_y} L {peak_point_x},{peak_point_y} Z")
            }
            ElemStyle::Right => {
                let peak_angle = end_angle + 0.03;
                let (peak_point_x, peak_point_y) = (
                    fig.center_x + fig.radius * peak_angle.cos(),
                    fig.center_y + fig.radius * peak_angle.sin(),
                );

                format!("M {start_outer_x},{start_outer_y} A {outer_radius} {outer_radius} 0 {large_arc_flag} 1 {end_outer_x}, {end_outer_y} L {peak_point_x},{peak_point_y} L {end_inner_x},{end_inner_y} A {inner_radius} {inner_radius} 0 {large_arc_flag} 0 {start_inner_x},{start_inner_y} Z")
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
