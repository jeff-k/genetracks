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
    origin: Point,
    scale: f64,
    height: f64,
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
            scale: fig.width / fig.length,
            height: fig.track_height,
            origin: Point {
                x: 0.0,
                y: index as f64 * fig.track_height,
            },
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
        Point {
            x: (start + end) / 2.0 * track.scale,
            y: track.origin.y,
        }
    });

    let path = create_memo(move |_| {
        let track = parent();

        let start_x = (track.origin.x + start) * track.scale;
        let end_x = (track.origin.x + end) * track.scale;
        let y = track.origin.y;

        let base_top = Point { x: start_x, y };
        let base_bottom = Point {
            x: start_x,
            y: y + track.height,
        };
        let end_top = Point { x: end_x, y };
        let end_bottom = Point {
            x: end_x,
            y: y + track.height,
        };

        match style {
            ElemStyle::Rect => {
                format!(
                    "M {base_top} \
                    H {end_x} \
                    V {} \
                    H {start_x} \
                    Z",
                    base_bottom.y
                )
            }
            ElemStyle::Right => {
                let peak = Point {
                    x: end_x + (10.0 * track.scale),
                    y: y + (track.height / 2.0),
                };
                format!(
                    "M {base_top} \
                    H {end_x} \
                    L {peak} \
                    L {end_bottom} \
                    H {start_x} \
                    Z"
                )
            }
            ElemStyle::Left => {
                let peak = Point {
                    x: start_x - (10.0 * track.scale),
                    y: y + (track.height / 2.0),
                };
                format!(
                    "M {base_top} \
                    H {end_x} \
                    V {} \
                    H {end_x} \
                    L {peak} \
                    Z",
                    end_bottom.y
                )
            }
            _ => format!(""),
        }
    });

    view! {
        <path d=path stroke="black" stroke-width="1" fill="none" />
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
            y: track.origin.y,
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
            y: track.origin.y,
        }
    });

    let path = create_memo(move |_| {
        let track = parent();

        let origin = (track.origin.x + pos) * track.scale;
        let end = track.origin.y + 10.0;

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

#[component]
pub fn Bar(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
    #[prop(optional)] style: ElemStyle,
) -> impl IntoView {
    let parent = use_context::<Memo<TrackCx>>().expect("Bar must be child of Track");

    let start = start as f64;
    let end = end as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        Point {
            x: track.origin.x + ((start + end) / 2.0 * track.scale),
            y: track.origin.y + (track.height / 2.0),
        }
    });

    let path = create_memo(move |_| {
        let track = parent();

        // Convert genome coordinates to screen coordinates
        let start_x = track.origin.x + (start * track.scale);
        let end_x = track.origin.x + (end * track.scale);
        let top_y = track.origin.y;
        let bottom_y = track.origin.y + track.height;
        let center_y = track.origin.y + (track.height / 2.0);

        match style {
            ElemStyle::Rect => {
                // Standard "|---|" bar
                format!(
                    "M {},{} \
                    L {},{} \
                    M {},{} \
                    H {} \
                    M {},{} \
                    L {},{}",
                    start_x,
                    top_y, // Start vertical line
                    start_x,
                    bottom_y,
                    start_x,
                    center_y, // Horizontal line
                    end_x,
                    end_x,
                    top_y, // End vertical line
                    end_x,
                    bottom_y
                )
            }
            ElemStyle::Left => {
                // "<---|" bar
                let peak = Point {
                    x: start_x - (10.0 * track.scale),
                    y: center_y,
                };
                format!(
                    "M {},{} \
                    L {} {} \
                    L {},{} \
                    M {},{} \
                    H {} \
                    M {},{} \
                    L {},{}",
                    start_x,
                    top_y, // Start peak top
                    peak.x,
                    peak.y, // Peak point
                    start_x,
                    bottom_y, // Start peak bottom
                    start_x,
                    center_y, // Horizontal line
                    end_x,
                    end_x,
                    top_y, // End vertical line
                    end_x,
                    bottom_y
                )
            }
            ElemStyle::Right => {
                // "|--->" bar
                let peak = Point {
                    x: end_x + (10.0 * track.scale),
                    y: center_y,
                };
                format!(
                    "M {},{} \
                    L {},{} \
                    M {},{} \
                    H {} \
                    M {},{} \
                    L {} {} \
                    L {},{}",
                    start_x,
                    top_y, // Start vertical line
                    start_x,
                    bottom_y,
                    start_x,
                    center_y, // Horizontal line
                    end_x,
                    end_x,
                    top_y, // End peak top
                    peak.x,
                    peak.y, // Peak point
                    end_x,
                    bottom_y // End peak bottom
                )
            }
            _ => format!(""),
        }
    });

    view! {
        <>
            <path d=path stroke=color.to_string() stroke-width="2" fill="none" />
            {label
                .map(|text| {
                    let pos = text_pos();
                    view! {
                        <text
                            x=pos.x
                            y=pos.y
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
