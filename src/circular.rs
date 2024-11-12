pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use crate::render::{CircularCoords, Layout, LinearCoords, RegionStyle};
use crate::Point;
use core::f64::consts::PI;
use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct FigCx {
    length: f64,
    center: Point,
    base_radius: f64,
    track_spacing: f64,
    track_height: f64,
    //    layout: Layout,
}

#[component]
pub fn Figure(#[prop(into)] length: Signal<u32>, children: Children) -> impl IntoView {
    let cx = create_memo(move |_| FigCx {
        length: length() as f64,
        center: Point { x: 400.0, y: 400.0 },
        base_radius: 300.0,
        track_spacing: 10.0,
        track_height: 12.0,
        //layout: Layout::Circular,
    });
    provide_context(cx);

    view! {
        {move || format!("len {}", length())}
        <svg viewBox="0 0 800 800">{children()}</svg>
    }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: Children) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let track_radius = create_memo(move |_| {
        let fig = cx();
        Layout::Circular(CircularCoords {
            length: fig.length,
            radius: fig.base_radius + (index as f64 * fig.track_spacing),
            height: fig.track_height,
            center: fig.center,
        })
    });

    provide_context(track_radius);

    view! { <g>{children()}</g> }
}

#[component]
pub fn Bar(
    #[prop(into)] start: u32,
    #[prop(into)] end: u32,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color: Colour,
    #[prop(optional)] style: ElemStyle,
) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let start = start as f64;
    let end = end as f64;

    let path = create_memo(move |_| {
        let track = parent();
        track.draw(start, end, RegionStyle::Bar, style)
    });

    view! { <path d=path stroke=color.to_string() stroke_width="2" fill="none" /> }
}

#[component]
pub fn Label(
    #[prop(into)] pos: u32,
    #[prop(optional)] color: Option<Colour>,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let pos = pos as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        track.label(pos)
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

    let pos = pos as f64;

    let path = create_memo(move |_| {
        let track = parent();

        /*
                let angle = (pos / track.length) * 2.0 * PI - PI / 2.0;

                let inner_radius = track.radius - track.track_height / 2.0;
                let outer_radius = track.radius + track.track_height + 40.0;

                let mk_point = |radius: f64, angle: f64| Point {
                    x: track.center.x + radius * angle.cos(),
                    y: track.center.y + radius * angle.sin(),
                };

                let inner = mk_point(inner_radius, angle);

                let outer = mk_point(outer_radius, angle);
        */
        format!("M 0.0 L 0.0")
    });

    view! {
        <g>
            <path d=path stroke="black" stroke-width="1" fill="none" />
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
    //    let cx = use_context::<ReadSignal<FigCx>>().expect("Region must be descendent of Figure");
    let parent = use_context::<Memo<Layout>>().expect("Region must be child of Track");

    let start = start as f64;
    let end = end as f64;

    let text_pos = create_memo(move |_| {
        let track = parent();
        track.label(start)
    });

    let path = create_memo(move |_| {
        let track = parent();
        track.draw(start, end, RegionStyle::Full, style)
    });

    view! {
        <>
            <path d=path fill=color.to_string() />
        </>
    }
}
