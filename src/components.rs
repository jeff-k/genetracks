pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use crate::render::{draw_sector, CircularCoords, Layout, LinearCoords, RegionStyle};
use crate::Point;
use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct FigCx {
    length: f64,
    width: f64,
    center: Point,
    base_radius: f64,
    track_spacing: f64,
    track_height: f64,
    circular: bool,
    start: f64,
    end: f64,
    //    layout: Layout,
}

#[component]
pub fn Figure(
    #[prop(into)] length: Signal<u32>,
    #[prop(into)] width: Signal<u32>,
    #[prop(into)] view_range: Signal<(u32, u32)>,
    #[prop(default = false)] circular: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let cx = create_memo(move |_| {
        let (start, end) = view_range();
        //        logging::log!("updating figure context: {} {} {}", start, end, width());
        FigCx {
            length: length() as f64,
            width: width() as f64,
            center: Point {
                x: width() as f64 / 2.0,
                y: width() as f64 / 2.0,
            },
            base_radius: width() as f64 / 2.0,
            track_spacing: 10.0,
            track_height: 12.0,
            circular,
            start: start as f64,
            end: end as f64,
        }
    });

    let scale = move || {
        let (start, end) = view_range();
        width() as f64 / (end - start) as f64
    };

    let height = move || {
        if circular {
            width()
        } else {
            //12 * children().iter().collect().len() as u32
            84
        }
    };

    view! {
        {move || {
            provide_context(cx);
            view! {
                <svg
                    width=move || format!("{}px", width())
                    height=move || format!("{}px", height())
                    viewBox=move || {
                        if circular {
                            format!("0 0 {} {}", width(), height())
                        } else {
                            let (start, _) = view_range();
                            format!("{} 0 {} {}", start as f64 * scale(), width() as f64, height())
                        }
                    }
                >
                    {children()}
                </svg>
            }
        }}
    }
}

#[component]
pub fn Sector(
    #[prop(into)] range: Signal<(u32, u32)>,
    top: i32,
    bottom: i32,
    #[prop(default = Colour::LightGrey)] color: Colour,
) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Sector must be descendent of Figure");

    let path = create_memo(move |_| {
        let fig = cx();
        let (start, end) = range();
        if fig.circular {
            let start = start as f64;
            let end = end as f64;

            let length = fig.length;
            let origin = fig.center;

            let outer_radius = fig.base_radius - (top as f64 * fig.track_spacing);
            let inner_radius = fig.base_radius - (bottom as f64 * fig.track_spacing);

            draw_sector(origin, length, start, end, outer_radius, inner_radius)
        } else {
            format!("")
        }
    });
    view! { <path d=path fill=color.to_string() /> }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: ChildrenFn) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let track_radius = create_memo(move |_| {
        let fig = cx();

        if fig.circular {
            Layout::Circular(CircularCoords {
                length: fig.length,
                radius: fig.base_radius - (index as f64 * fig.track_spacing),
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
                scale: fig.width / (fig.end - fig.start),
                height: fig.track_height,
            })
        }
    });

    view! {
        {move || {
            provide_context(track_radius);
            view! { <g>{children()}</g> }
        }}
    }
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
        <g>
            <path d=path stroke=color.to_string() stroke_width="2" fill="none" />
            {label
                .map(|text| {
                    view! { <Label pos=(start + end) / 2>{text}</Label> }
                })}
        </g>
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
