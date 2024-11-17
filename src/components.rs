pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use crate::render::{CircularCoords, Layout, LinearCoords, RegionStyle};
use crate::Point;
use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct FigCx {
    length: f64,
    center: Point,
    base_radius: f64,
    track_spacing: f64,
    track_height: f64,
    circular: bool,
    //    layout: Layout,
}

#[component]
pub fn Figure(
    #[prop(into)] length: Signal<u32>,
    children: Children,
    #[prop(default = false)] circular: bool,
) -> impl IntoView {
    let cx = create_memo(move |_| FigCx {
        length: length() as f64,
        center: Point { x: 400.0, y: 400.0 },
        base_radius: 300.0,
        track_spacing: 10.0,
        track_height: 12.0,
        circular,
        //layout: Layout::Circular,
    });
    provide_context(cx);

    view! {
        // {move || format!("len {}", length())}
        <svg viewBox="0 0 800 200">{children()}</svg>
    }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: Children) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let track_radius = create_memo(move |_| {
        let fig = cx();

        if fig.circular {
            Layout::Circular(CircularCoords {
                length: fig.length,
                radius: fig.base_radius + (index as f64 * fig.track_spacing),
                height: fig.track_height,
                center: fig.center,
            })
        } else {
            /*
             */

            Layout::Linear(LinearCoords {
                origin: Point {
                    x: 0.0,
                    y: fig.track_height * index as f64,
                },
                scale: fig.length / 800.0,
                height: fig.track_height,
            })
        }
    });

    provide_context(track_radius);

    view! { <g>{children()}</g> }
}

#[component]
pub fn Bar(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] label: Option<String>,
    #[prop(default = Colour::Black)] color: Colour,
    #[prop(optional)] style: ElemStyle,
) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let path = create_memo(move |_| {
        let track = parent();
        track.draw(start as f64, end as f64, RegionStyle::Bar, style)
    });

    view! {
        <>
            <path d=path stroke=color.to_string() stroke_width="2" fill="none" />
            {label
                .map(|text| {
                    view! { <Label pos=(start + end) / 2>{text}</Label> }
                })}
        </>
    }
}

#[component]
pub fn Label(
    #[prop(into)] pos: u32,
    #[prop(optional)] color: Option<Colour>,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let text_pos = create_memo(move |_| {
        let track = parent();
        track.map_pos(pos as f64)
    });

    view! {
        <text
            x=move || text_pos().x
            y=move || text_pos().y
            text-anchor="middle"
            dominant-baseline="middle"
            fill=color.unwrap_or(Colour::Black).to_string()
            font-size="smaller"
            font-family="monospace"
        >
            {children()}
        </text>
    }
}

#[component]
pub fn Tick(#[prop(into)] pos: u32, #[prop(optional)] label: Option<String>) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("must have track as parent");

    let path = create_memo(move |_| {
        let track = parent();
        track.draw_tick(pos as f64)
    });

    view! {
        <g>
            <path d=path stroke="black" stroke-width="1" fill="none" />
            {label
                .map(|text| {
                    view! { <Label pos=pos>{text}</Label> }
                })}
        </g>
    }
}

#[component]
pub fn Region(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] style: ElemStyle,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let path = create_memo(move |_| {
        let track = parent();
        track.draw(start as f64, end as f64, RegionStyle::Full, style)
    });

    view! {
        <g>
            <path d=path fill=color.to_string() />
            {label
                .map(|text| {
                    view! { <Label pos=(start + end) / 2>{text}</Label> }
                })}
        </g>
    }
}
