pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};
use crate::render::{draw_sector, CircularCoords, Layout, LinearCoords, RegionStyle};
use crate::Point;
use leptos::either::Either;
use leptos::prelude::*;

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
    let cx = Memo::new(move |_| {
        let (start, end) = view_range();
        //        logging::log!("updating figure context: {} {} {}", start, end, width());

        let length = f64::from(length());
        let width = f64::from(width());

        FigCx {
            length,
            width,
            center: Point {
                x: width / 2.0,
                y: width / 2.0,
            },
            base_radius: width / 2.0,
            track_spacing: 10.0,
            track_height: 12.0,
            circular,
            start: f64::from(start),
            end: f64::from(end),
        }
    });

    let scale = move || {
        let (start, end) = view_range();
        f64::from(width()) / f64::from(end - start)
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
                            let start = f64::from(start);
                            format!("{} 0 {} {}", start * scale(), width(), height())
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

    let path = Memo::new(move |_| {
        let fig = cx();
        let (start, end) = range();
        if fig.circular {
            let start = f64::from(start);
            let end = f64::from(end);
            let top = f64::from(top);
            let bottom = f64::from(bottom);

            let length = fig.length;
            let origin = fig.center;

            let outer_radius = fig.base_radius - (top * fig.track_spacing);
            let inner_radius = fig.base_radius - (bottom * fig.track_spacing);

            draw_sector(origin, length, start, end, outer_radius, inner_radius)
        } else {
            String::new()
        }
    });
    view! { <path d=path fill=color.to_string() /> }
}

#[component]
pub fn Track(#[prop(into)] index: i32, children: ChildrenFn) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let track_radius = Memo::new(move |_| {
        let fig = cx();

        if fig.circular {
            Layout::Circular(CircularCoords {
                length: fig.length,
                radius: fig.base_radius - (f64::from(index) * fig.track_spacing),
                height: fig.track_height,
                center: fig.center,
            })
        } else {
            /*
             */

            Layout::Linear(LinearCoords {
                origin: Point {
                    x: 0.0,
                    y: fig.track_height * f64::from(index),
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

    let path = Memo::new(move |_| {
        let track = parent();
        track.draw(f64::from(start), f64::from(end), RegionStyle::Bar, style)
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

    let text_pos = Memo::new(move |_| {
        let track = parent();
        track.map_pos(f64::from(pos))
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

    let path = Memo::new(move |_| {
        let track = parent();
        track.draw_tick(f64::from(pos))
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

    let path = Memo::new(move |_| {
        let track = parent();
        track.draw(f64::from(start), f64::from(end), RegionStyle::Full, style)
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

#[component]
pub fn Ticks(
    #[prop(into)] n: u32,
    #[prop(into)] range: Signal<(u32, u32)>,
    #[prop(default = false)] text: bool,
) -> impl IntoView {
    //    let ns: Vec<u32> = (0..=n).filter(|x| (x % 50) == 0).collect();
    let ns: Memo<Vec<u32>> = Memo::new(move |_| {
        let (start, end) = range();
        let m = ((end - start) / n).max(1);
        //logging::log!("{m}");
        (start..=end).filter(|x| (x % m) == 0).collect()
    });

    move || {
        if text {
            Either::Left(view! {
                {ns()
                    .into_iter()
                    .map(|n| {
                        view! { <Label pos=n>{format!("{n}")}</Label> }
                    })
                    .collect::<Vec<_>>()}
            })
        } else {
            Either::Right(view! {
                {ns()
                    .into_iter()
                    .map(|n| {
                        view! { <Tick pos=n /> }
                    })
                    .collect::<Vec<_>>()}
            })
        }
    }
}
