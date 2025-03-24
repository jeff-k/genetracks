pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};

use crate::Point;
use crate::render::{
    CircularCoords, Layout, LayoutWrapper, LinearCoords, draw_highlight, draw_sector,
};
//use leptos::either::Either;
//use leptos::logging;
use leptos::context::Provider;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::svg;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::{ResizeObserver, ResizeObserverEntry};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FigCx {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub track_height: f64,
    pub view: (u32, u32),
    pub center: Point,
    pub scale: f64,
    pub circular: bool,
}

impl FigCx {
    fn viewbox(&self) -> String {
        format!("0 0 {} {}", self.width, self.height)
    }

    fn view_width(&self) -> String {
        format!("{}", self.width)
    }

    fn view_height(&self) -> String {
        format!("{}", self.height)
    }
}

#[component]
pub fn Circular(
    #[prop(into)] length: Signal<u32>,
    #[prop(into, optional)] width: Option<Signal<u32>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref: NodeRef<svg::Svg> = NodeRef::new();

    let (container_width, set_width) = signal(800f64);

    let fig_width: Signal<f64> = match width {
        Some(w) => Signal::derive(move || f64::from(w())),
        None => Signal::from(container_width),
    };

    let cx = Memo::new(move |_| {
        let length = f64::from(length());
        let width = fig_width();

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

    Effect::new(move |_| {
        if width.is_none() {
            //logging::log!("creating width effect");
            if let Some(elem) = node_ref.get() {
                if let Some(parent) = elem.parent_element() {
                    let rect = parent.get_bounding_client_rect();
                    let w = rect.width();
                    set_width.update(|s| *s = w);

                    let cb = Closure::wrap(Box::new(
                        move |es: Vec<ResizeObserverEntry>, _: ResizeObserver| {
                            if let Some(e) = es.first() {
                                let rect = e.content_rect();
                                set_width.update(|s| *s = rect.width());
                            }
                        },
                    )
                        as Box<dyn FnMut(Vec<ResizeObserverEntry>, ResizeObserver)>);

                    let observer = ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();

                    observer.observe(&parent);

                    // leaks memory?
                    cb.forget();
                }
            }
        }
    });

    let width = Memo::new(move |_| cx().view_width());
    let height = Memo::new(move |_| cx().view_height());
    let viewBox = Memo::new(move |_| cx().viewbox());

    view! {
        <svg node_ref=node_ref width=width height=height viewBox=viewBox>
            <Provider value=cx>{children.map(|children_fn| children_fn())}</Provider>
        </svg>
    }
}

#[component]
pub fn Figure(
    #[prop(into)] length: Signal<u32>,
    #[prop(into, optional)] width: Option<Signal<u32>>,
    #[prop(default = 5)] tracks: u32,
    #[prop(default = 12.0)] track_height: f64,
    #[prop(into, optional)] view: Option<Signal<(u32, u32)>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    //        logging::log!("updating figure context: {} {} {}", start, end, width());

    let node_ref: NodeRef<svg::Svg> = NodeRef::new();

    let (container_width, set_width) = signal(800f64);

    let fig_width: Signal<f64> = match width {
        Some(w) => Signal::derive(move || f64::from(w())),
        None => Signal::from(container_width),
    };

    let fig_length = Memo::new(move |_| f64::from(length()));
    //    let fig_width = Memo::new(move |_| fig_width());
    let viewbox = Memo::new(move |_| match view {
        Some(viewfn) => viewfn(),
        None => (0, length()),
    });

    let height = track_height * f64::from(tracks);

    let scale = Memo::new(move |_| {
        let (start, end) = viewbox();
        fig_width() / f64::from(end - start)
    });

    let cx: Memo<FigCx> = Memo::new(move |_| FigCx {
        length: fig_length(),
        width: fig_width(),
        height,
        track_height,
        view: viewbox(),
        center: Point { x: 0.0, y: 0.0 },
        scale: scale(),
        circular: false,
    });

    Effect::new(move |_| {
        if width.is_none() {
            if let Some(elem) = node_ref.get() {
                if let Some(parent) = elem.parent_element() {
                    let rect = parent.get_bounding_client_rect();
                    let w = rect.width();
                    set_width.update(|s| *s = w);

                    let cb = Closure::wrap(Box::new(
                        move |es: Vec<ResizeObserverEntry>, _: ResizeObserver| {
                            if let Some(e) = es.first() {
                                let rect = e.content_rect();
                                set_width.update(|s| *s = rect.width());
                            }
                        },
                    )
                        as Box<dyn FnMut(Vec<ResizeObserverEntry>, ResizeObserver)>);

                    let observer = ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();

                    observer.observe(&parent);

                    // leaks memory?
                    cb.forget();
                }
            }
        }
    });

    let svg_width = Memo::new(move |_| format!("{}px", fig_width()));
    let svg_viewbox = Memo::new(move |_| {
        let (start, _) = viewbox();
        let scaled = f64::from(start) * scale();
        format!("{scaled} 0 {} {height}", fig_width())
    });

    view! {
        <svg node_ref=node_ref width=svg_width height=format!("{height}px") viewBox=svg_viewbox>
            <Provider value=cx>{children.map(|children_fn| children_fn())}</Provider>
        </svg>
    }
}

#[component]
pub fn Highlight(
    #[prop(into)] top: u32,
    #[prop(into)] bottom: u32,
    #[prop(into)] range: Signal<(u32, u32)>,
    //    #[prop(default = None, into, optional)] bottom_range: Option<Signal<(u32, u32)>>,
    #[prop(default = Signal::derive(move || Colour::LightGrey), into, optional)] color: Signal<
        Colour,
    >,
) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Highlight must be descendent of Figure");

    let path = Memo::new(move |_| {
        cx.with(|fig| {
            let (start, end) = range();

            // TODO
            //            let b_range = match bottom_range {
            //                Some(br) => br,
            //                None => range,
            //            };

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
        })
    });

    view! { <path d=path fill=move || color().to_string() /> }
}

#[component]
pub fn Track(
    #[prop(into)] index: u32,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Track must be descendent of Figure");

    let index = f64::from(index);

    let layout: Memo<LayoutWrapper> = Memo::new(move |_| {
        cx.with(|fig| {
            if fig.circular {
                LayoutWrapper::Circular(CircularCoords {
                    length: fig.length,
                    radius: (fig.width / 2.0) - (index * fig.track_height),
                    height: fig.track_height,
                    center: fig.center,
                })
            } else {
                LayoutWrapper::Linear(LinearCoords {
                    origin: Point {
                        x: 0.0,
                        y: index * fig.track_height,
                    },
                    scale: fig.scale,
                    height: fig.track_height,
                })
            }
        })
    });

    view! {
        <Provider value=layout>
            <g>{children.map(|children_fn| children_fn())}</g>
        </Provider>
    }
}

#[component]
pub fn Bar(
    #[prop(into)] start: Signal<u32>,
    #[prop(into)] end: Signal<u32>,
    #[prop(default = Signal::derive(move || Colour::Black), into, optional)] color: Signal<Colour>,
    #[prop(optional)] style: ElemStyle,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_hover: Option<Callback<bool>>,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Bar must be child of Track");

    let handle_leave = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(false);
        }
    };

    let handle_enter = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(true);
        }
    };

    let on_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <g>
            <path
                d=move || layout.with(|l| { l.draw_bar(start(), end(), style) })
                stroke=move || color().to_string()
                stroke_width="2"
                fill="none"
                on:mouseenter=handle_enter
                on:mouseleave=handle_leave
                on:click=on_click
            />
            <Provider value=layout>{children.map(|children_fn| children_fn())}</Provider>
        </g>
    }
}

#[component]
pub fn Label(
    #[prop(into)] pos: Signal<u32>,
    #[prop(default = Signal::derive(move || Colour::Black), into, optional)] color: Signal<Colour>,
    children: Children,
) -> impl IntoView {
    //    let layout = use_context::<Memo<LinearCoords>>().expect("Region must be child of Track");
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Label must be child of Track");

    let pos = Memo::new(move |_| layout.with(|l| l.map_pos(pos())));

    view! {
        <text
            x=move || pos.with(|p| p.x)
            y=move || pos.with(|p| p.y)
            text-anchor="middle"
            dominant-baseline="middle"
            fill=move || color().to_string()
            font-size="smaller"
            font-family="monospace"
        >
            {children()}
        </text>
    }
}

#[component]
pub fn Tick(
    #[prop(into)] pos: Signal<u32>,
    #[prop(optional)] label: Option<String>,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Tick must be child of Track");
    let pos = Memo::new(move |_| pos());
    view! {
        <g>
            <path
                d=move || { layout.with(|l| l.draw_tick(pos())) }
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
    #[prop(into)] start: Signal<u32>,
    #[prop(into)] end: Signal<u32>,
    #[prop(optional)] style: ElemStyle,
    #[prop(default = Signal::derive(move || Colour::LightGrey), into, optional)] color: Signal<
        Colour,
    >,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_hover: Option<Callback<bool>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Region must be child of Track");
    //    let label_pos = Memo::new(move |_| u32::midpoint(start(), end()));

    let handle_leave = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(false);
        }
    };

    let handle_enter = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(true);
        }
    };

    let on_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };
    view! {
        <g>
            <path
                d=move || { layout.with(|l| l.draw_filled(start(), end(), style)) }
                fill=move || color().to_string()
                on:mouseenter=handle_enter
                on:mouseleave=handle_leave
                on:click=on_click
            />
            <Provider value=layout>{children.map(|children_fn| children_fn())}</Provider>
        </g>
    }
}

#[component]
pub fn Ticks(
    #[prop(into)] n: u32,
    #[prop(into)] range: Signal<(u32, u32)>,
    #[prop(default = false)] text: bool,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Ticks must be child of Track");

    let ticks: Memo<Vec<u32>> = Memo::new(move |_| {
        let (start, end) = range();
        let m = ((end - start) / n).max(1);
        //logging::log!("{m}");
        (0..=n).map(|i| start + (i * m)).collect::<Vec<_>>()
    });

    if text {
        view! {
            <For
                each=move || ticks()
                key=|pos| *pos
                children=move |pos| {
                    view! {
                        <Provider value=layout>
                            <Label pos=pos>{pos}</Label>
                        </Provider>
                    }
                }
            />
        }
        .into_any()
    } else {
        view! {
            <For
                each=move || ticks()
                key=|pos| *pos
                children=move |pos| {
                    view! {
                        <Provider value=layout>
                            <Tick pos=pos />
                        </Provider>
                    }
                }
            />
        }
        .into_any()
    }
}

#[component]
pub fn Ribbon(
    #[prop(into)] start: Signal<(u32, u32)>,
    #[prop(into)] end: Signal<(u32, u32)>,
    #[prop(default = None)] target: Option<u32>,
    #[prop(default = Signal::derive(move || Colour::OrangeRed), into, optional)] color: Signal<
        Colour,
    >,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_hover: Option<Callback<bool>>,
    #[prop(default = 0.2)] opacity: f64,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Ribbon must be child of Track");

    let handle_leave = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(false);
        }
    };

    let handle_enter = move |_: MouseEvent| {
        if let Some(cb) = on_hover {
            cb.run(true);
        }
    };

    let on_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <g>
            <path
                d=move || { layout.with(|l| l.draw_ribbon(start(), end(), target)) }
                fill=move || color().to_string()
                opacity=opacity.to_string()
                on:mouseenter=handle_enter
                on:mouseleave=handle_leave
                on:click=on_click
            />
        </g>
    }
}
