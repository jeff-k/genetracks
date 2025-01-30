pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};

use crate::render::{draw_highlight, draw_sector, CircularCoords, Layout, LinearCoords};
use crate::Point;
use leptos::either::Either;
//use leptos::logging;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FigCx {
    pub(crate) length: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) track_height: f64,
    pub(crate) view: (u32, u32),
    pub(crate) center: Point,
    pub(crate) scale: f64,
    pub(crate) circular: bool,
}

#[component]
pub fn Circular(
    #[prop(into)] length: Signal<u32>,
    #[prop(into)] width: Signal<u32>,
    children: ChildrenFn,
) -> impl IntoView {
    let cx = Memo::new(move |_| {
        let length = f64::from(length());
        let width = f64::from(width());

        FigCx {
            length,
            width,
            height: width,
            track_height: 12.0,
            view: (0, 0),
            center: Point {
                x: width / 2.0,
                y: width / 2.0,
            },
            scale: 1.0,
            circular: true,
        }
    });

    view! {
        {move || {
            provide_context(cx);
            let FigCx {
                length: _,
                width,
                height,
                center: _,
                track_height: _,
                view: _,
                scale: _,
                circular: _,
            } = cx();
            view! {
                <svg
                    width=move || format!("{width}px")
                    height=move || format!("{height}px")
                    viewBox=move || { format!("0 0 {width} {height}") }
                >
                    {children()}
                </svg>
            }
        }}
    }
}

#[component]
pub fn Figure(
    #[prop(into)] length: Signal<u32>,
    #[prop(into)] width: Signal<u32>,
    #[prop(default = 5)] tracks: u32,
    #[prop(default = 12.0)] track_height: f64,
    #[prop(default = None)] view: Option<ReadSignal<(u32, u32)>>,
    children: ChildrenFn,
) -> impl IntoView {
    //        logging::log!("updating figure context: {} {} {}", start, end, width());

    let cx = Memo::new(move |_| {
        let height = track_height * f64::from(tracks);
        let u_len = length();
        let length = f64::from(u_len);
        let width = f64::from(width());
        let (start, end) = match view {
            Some(vr) => vr(),
            None => (0, u_len),
        };
        let scale = width / f64::from(end - start);
        //logging::log!("updating scale {scale}");
        FigCx {
            length,
            width,
            height,
            track_height,
            view: (start, end),
            center: Point { x: 0.0, y: 0.0 },
            scale,
            circular: false,
        }
    });

    view! {
        {move || {
            provide_context(cx);
            let FigCx {
                length: _,
                width,
                height,
                track_height: _,
                view: (start, _),
                scale,
                center: _,
                circular: _,
            } = cx();
            view! {
                <svg
                    width=move || format!("{width}px")
                    height=move || format!("{height}px")
                    viewBox=move || {
                        let start = f64::from(start) * scale;
                        format!("{start} 0 {width} {height}")
                    }
                >
                    {children()}
                </svg>
            }
        }}
    }
}

#[component]
pub fn Highlight(
    top: u32,
    bottom: u32,
    #[prop(into)] range: Signal<(u32, u32)>,
    #[prop(default = None)] bottom_range: Option<Signal<(u32, u32)>>,
    #[prop(default = Colour::LightGrey)] color: Colour,
) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Highlight must be descendent of Figure");

    let path = Memo::new(move |_| {
        let fig = cx();
        let (start, end) = range();
        let (_b_start, _b_end) = match bottom_range {
            Some(b_range) => b_range(),
            None => (start, end),
        };

        if fig.circular {
            let radius = fig.width / 2.0;
            let start = f64::from(start);
            let end = f64::from(end);
            let top = f64::from(top);
            let bottom = f64::from(bottom);

            let length = fig.length;
            let origin = fig.center;

            let outer_radius = radius - (top * fig.track_height);
            let inner_radius = radius - (bottom * fig.track_height);

            draw_sector(origin, length, start, end, outer_radius, inner_radius)
        } else {
            let top = f64::from(top) * fig.track_height;
            let bottom = f64::from(bottom) * fig.track_height;
            let (start, end) = range();

            draw_highlight(
                f64::from(start) * fig.scale,
                f64::from(end) * fig.scale,
                top,
                bottom,
            )
        }
    });

    view! { <path d=path fill=color.to_string() /> }
}

#[component]
pub fn Track(#[prop(into)] index: u32, children: ChildrenFn) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let FigCx {
        width,
        length,
        height: _,
        track_height,
        view: _,
        center,
        scale,
        circular,
    } = cx();

    let index = f64::from(index);

    let (layout, set_layout) = signal({
        if circular {
            Box::new(CircularCoords {
                length,
                radius: (width / 2.0) - (index * track_height),
                height: track_height,
                center,
            }) as Box<dyn Layout>
        } else {
            let y = track_height * index;
            Box::new(LinearCoords {
                origin: Point { x: 0.0, y },
                scale,
                height: track_height,
            }) as Box<dyn Layout>
        }
    });

    Effect::new(move |_| {
        let fig = cx();
        //logging::log!("linear layout scale {}", fig.scale);
        set_layout.update(|l| l.update(&fig, index));
    });

    view! {
        {move || {
            provide_context(layout);
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
    let layout =
        use_context::<ReadSignal<Box<dyn Layout>>>().expect("Region must be child of Track");

    view! {
        <g>
            <path
                d=move || layout.with(|l| { l.draw_bar(start, end, style) })
                stroke=color.to_string()
                stroke_width="2"
                fill="none"
            />
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
    //    let layout = use_context::<Memo<LinearCoords>>().expect("Region must be child of Track");
    let layout =
        use_context::<ReadSignal<Box<dyn Layout>>>().expect("Region must be child of Track");
    layout.with(|l| {
        let p = l.map_pos(pos);
        //logging::log!("printing to {} {}", p.x, p.y);
        view! {
            <text
                x=p.x
                y=p.y
                text-anchor="middle"
                dominant-baseline="middle"
                fill=color.unwrap_or(Colour::Black).to_string()
                font-size="smaller"
                font-family="monospace"
            >
                {children()}
            </text>
        }
    })
}

#[component]
pub fn Tick(#[prop(into)] pos: u32, #[prop(optional)] label: Option<String>) -> impl IntoView {
    //    let layout = use_context::<Memo<LinearCoords>>().expect("Region must be child of Track");
    let layout =
        use_context::<ReadSignal<Box<dyn Layout>>>().expect("Region must be child of Track");
    view! {
        <g>
            <path
                d=move || { layout.with(|l| l.draw_tick(pos)) }
                stroke="black"
                stroke-width="1"
                fill="none"
            />
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
    //    let layout = use_context::<Memo<LinearCoords>>().expect("Region must be child of Track");

    let layout =
        use_context::<ReadSignal<Box<dyn Layout>>>().expect("Region must be child of Track");
    view! {
        <g>
            <path
                d=move || { layout.with(|l| l.draw_filled(start, end, style)) }
                fill=color.to_string()
            />
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

#[component]
pub fn Ribbon(
    #[prop(into)] start: (u32, u32),
    #[prop(into)] end: (u32, u32),
    #[prop(default = None)] target: Option<u32>,
    #[prop(default = Colour::OrangeRed)] color: Colour,
    #[prop(default = 0.2)] opacity: f64,
) -> impl IntoView {
    let layout =
        use_context::<ReadSignal<Box<dyn Layout>>>().expect("Ribbon must be child of Track");
    view! {
            <g>
    //      { logging::log!("updating figure context {:?} {:?}", start ,end) }
                <path
                    d=move || { layout.with(|l| l.draw_ribbon(start, end, target)) }
                    fill=color.to_string()
                    opacity=opacity.to_string()
                />
            </g>
        }
}
