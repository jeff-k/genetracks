pub use crate::elements::{Colour, ElemStyle, ElementData, TrackData};

use crate::Point;
use crate::render::{
    CircularCoords, Layout, LayoutWrapper, LinearCoords, draw_highlight, draw_sector, svg_arc_path,
};
//use leptos::either::Either;
use leptos::context::Provider;
use leptos::ev::MouseEvent;
use leptos::ev::WheelEvent;
use leptos::logging;
use leptos::prelude::*;
use leptos::svg;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::{DragEvent, Event, ResizeObserver, ResizeObserverEntry};

//use core::f64::consts::{FRAC_2_PI, PI, TAU};
use std::sync::atomic::{AtomicU32, Ordering};

static LABEL_ID: AtomicU32 = AtomicU32::new(0);

fn next_label_id() -> String {
    let id = LABEL_ID.fetch_add(1, Ordering::Relaxed);
    logging::log!("label-path-{id}");
    format!("label-path-{id}")
}

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
            logging::log!("creating width effect");
            if let Some(elem) = node_ref.get()
                && let Some(parent) = elem.parent_element()
            {
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
    });

    let width = Memo::new(move |_| {
        logging::log!("changing width {}", cx().view_width());
        cx().view_width()
    });
    let height = Memo::new(move |_| cx().view_height());
    let view_box = Memo::new(move |_| cx().viewbox());

    view! {
        <svg
            node_ref=node_ref
            width=width
            height=height
            viewBox=view_box
            // on:wheel=on_wheel
            on:dragstart=|ev: DragEvent| ev.prevent_default()
            on:selectstart=|ev: Event| ev.prevent_default()
            draggable="false"
            style="touch-action: none; \
            user-select: none; \
            -webkit-user-select: none; \
            -webkit-user-drag: none; \
            -webkit-tap-highlight-color: transparent;"
        >
            <defs>
                <filter id="glow-effect" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="4" result="coloredBlur" />
                    <feMerge>
                        <feMergeNode in="coloredBlur" />
                        <feMergeNode in="sourceGraphic" />
                    </feMerge>
                </filter>
            </defs>
            <Provider value=cx>{children.map(|children_fn| children_fn())}</Provider>
        </svg>
    }
}

#[component]
pub fn Figure(
    #[prop(into)] length: Signal<u32>,
    #[prop(into, optional)] width: Option<Signal<u32>>, // reactive pixel width
    #[prop(default = 5)] tracks: u32,
    #[prop(default = 12.0)] track_height: f64,
    #[prop(into, optional)] view: Option<Signal<(u32, u32)>>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] on_scroll: Option<WriteSignal<(u32, u32)>>,
) -> impl IntoView {
    let node_ref: NodeRef<svg::Svg> = NodeRef::new();

    let (container_width, set_width) = signal(800f64);

    let fig_width: Signal<f64> = match width {
        Some(w) => Signal::derive(move || f64::from(w())),
        None => Signal::from(container_width),
    };

    let fig_length = Memo::new(move |_| f64::from(length()));
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
        if width.is_none()
            && let Some(elem) = node_ref.get()
            && let Some(parent) = elem.parent_element()
        {
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
    });

    let on_wheel = move |ev: WheelEvent| {
        let Some(zoom) = on_scroll else { return };
        ev.prevent_default();

        cx.with(|fig| {
            let (vs, ve) = fig.view;
            let length = fig.length as u32;
            let vwidth = ve - vs;
            let min_v = 100u32;

            let cursor = f64::from(ev.offset_x()) / fig.scale;

            let pos: u32 = cursor as u32;
            //logging::log!("cursor pos {pos} {vwidth} {cursor} {vs} {ve}");

            if ev.delta_y() != 0.0 {
                let zoom_factor: f64 = if ev.delta_y() > 0.0 { 1.1 } else { 0.9 };

                zoom.update(|(s, e)| {
                    let mut new_width = (f64::from(vwidth) * zoom_factor) as u32;
                    new_width = new_width.clamp(min_v, length);

                    let cursor_offset = new_width / 2;

                    let new_start: u32 = pos.saturating_sub(cursor_offset);

                    let new_end = new_start + new_width;

                    *s = new_start;
                    *e = new_end;
                });
            }

            if ev.delta_x() != 0.0 {
                zoom.update(|(s, e)| {
                    if ev.delta_x() < 0.0 {
                        *s = s.saturating_sub(10);
                        *e = *s + vwidth;
                    } else {
                        *e = e.saturating_add(10).min(length);
                        *s = e.saturating_sub(vwidth);
                    }
                });
            }
        });
    };

    let svg_width = Memo::new(move |_| format!("{}px", fig_width()));
    let svg_viewbox = Memo::new(move |_| {
        let (start, end) = viewbox();
        format!("{start} 0 {} {height}", end.saturating_sub(start))
    });

    view! {
        <svg
            node_ref=node_ref
            width=svg_width
            height=format!("{height}px")
            viewBox=svg_viewbox
            on:wheel=on_wheel
            style="touch-action: none; user-select: none;"
        >
            <defs>
                <filter id="glow-effect" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="4" result="coloredBlur" />
                    <feMerge>
                        <feMergeNode in="coloredBlur" />
                        <feMergeNode in="sourceGraphic" />
                    </feMerge>
                </filter>
            </defs>
            <Provider value=cx>{children.map(|children_fn| children_fn())}</Provider>
        </svg>
    }
}

#[component]
pub fn Highlight(
    #[prop(into)] top: u32,
    #[prop(into)] bottom: u32,
    #[prop(into)] range: Signal<(u32, u32)>,
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

                draw_highlight(f64::from(start), f64::from(end), top, bottom)
            }
        })
    });

    view! { <path d=path fill=move || color().to_string() /> }
}

#[component]
pub fn FigureTitle(children: Children) -> impl IntoView {
    let cx = use_context::<Memo<FigCx>>().expect("Figure title must be child of circular figure");

    view! {
        <text
            x=move || cx.with(|f| f.center.x)
            y=move || cx.with(|f| f.center.y)
            text-anchor="middle"
            dominant-baseline="middle"
            font-size="14"
            font-family="sans-serif"
        >
            {children()}
        </text>
    }
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

    let transform = Memo::new(move |_| {
        if cx.with(|f| f.circular) {
            String::new()
        } else {
            let row = index * cx.with(|f| f.track_height);
            format!("translate(0, {row})")
        }
    });

    view! {
        <g transform=transform>
            <Provider value=layout>{children.map(|children_fn| children_fn())}</Provider>
        </g>
    }
}

#[component]
pub fn Bar(
    #[prop(into)] start: Signal<u32>,
    #[prop(into)] end: Signal<u32>,
    #[prop(default = Signal::derive(move || Colour::Black), into, optional)] color: Signal<Colour>,
    #[prop(optional)] style: ElemStyle,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] on_click: Option<Callback<u32>>,
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
            cb.run(0);
        }
    };

    view! {
        <g>
            <path
                d=move || layout.with(|l| { l.draw_bar(start(), end(), style) })
                stroke=move || color().to_string()
                stroke_width="2"
                fill="none"
                vector-effect="non-scaling-stroke"
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
    #[prop(default = true)] curve: bool,
    #[prop(default = Signal::derive(move || Colour::Black), into, optional)] color: Signal<Colour>,
    #[prop(optional)] on_click: Option<Callback<u32>>,
    #[prop(optional)] on_hover: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let layout = use_context::<Memo<LayoutWrapper>>().expect("Label must be child of Track");

    let mapped_pos = Memo::new(move |_| layout.with(|l| l.map_pos(pos())));

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
            cb.run(0);
        }
    };
    let transform = Memo::new(move |_| {
        layout.with(|l| match l {
            LayoutWrapper::Linear(lc) => {
                let p = l.map_pos(pos());
                let inv_scale = 1.0 / lc.scale;
                format!(
                    "translate({} {}) scale({}) translate({}, {})",
                    p.x, p.y, inv_scale, -p.x, -p.y
                )
            }
            LayoutWrapper::Circular(_) => String::new(),
        })
    });

    let path_id = next_label_id();
    let path_id_clone = path_id.clone();

    let circular: bool = layout.with(|l| matches!(l, LayoutWrapper::Circular(_)));

    let use_curve = circular && curve;

    let curve_path = Memo::new(move |_| {
        layout.with(|l| {
            if let LayoutWrapper::Circular(cc) = l {
                let pos = pos();
                svg_arc_path(cc, pos, 200.0)
            } else {
                String::new()
            }
        })
    });

    if use_curve {
        logging::log!("label_id: {}, path: {}", path_id.clone(), curve_path());
        view! {
            <defs>
                <path id=path_id.clone() d=curve_path file="none" />
            </defs>

            <text fill=move || color().to_string() font-size="10" font-family="monospace">
                <textPath
                    href=format!("#{path_id_clone}")
                    startOffset="50%"
                    text-anchor="middle"
                    dominant-baseline="middle"
                    on:mouseenter=handle_enter
                    on:mouseleave=handle_leave
                    on:click=on_click
                >

                    {children()}
                </textPath>
            </text>
        }
        .into_any()
    } else {
        view! {
            <text
                x=move || mapped_pos.with(|p| p.x)
                y=move || mapped_pos.with(|p| p.y)
                fill=move || color().to_string()
                font-size="10"
                font-family="monospace"
                text-anchor="middle"
                dominant-baseline="middle"
                transform=transform
            >

                {children()}
            </text>
        }
        .into_any()
    }
}

#[component]
pub fn Tick(
    #[prop(into)] pos: Signal<u32>,
    #[prop(optional)] end: Option<u32>,
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
                vector-effect="non-scaling-stroke"
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
    #[prop(default = Signal::derive(move || Colour::Black), into, optional)] stroke: Signal<Colour>,
    #[prop(optional, into)] selected: Option<Signal<bool>>,
    #[prop(optional)] on_click: Option<Callback<u32>>,
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

    let on_click = move |ev: MouseEvent| {
        if let Some(cb) = on_click {
            let target = event_target::<web_sys::Element>(&ev);
            let pos: u32 = ev.client_x() as u32;
            cb.run(pos); // TODO: should be the genome region coord
        }
    };
    let path_id = next_label_id();
    let href_ref = format!("#{path_id}");

    let is_highlighted = move || selected.map(|s| s.get()).unwrap_or(false);

    view! {
        <g>
            <defs>
                <path
                    id=path_id.clone()
                    d=move || { layout.with(|l| l.draw_filled(start(), end(), style)) }
                    fill=move || color().to_string()
                    vector-effect="non-scaling-stroke"
                />
            // filter="url(#glow-effect)"
            </defs>

            {
                let href_ref = href_ref.clone();
                move || {
                    if is_highlighted() {
                        logging::log!("selected {href_ref}");
                        Some(
                            view! {
                                <use
                                    href=href_ref.clone()
                                    stroke="yellow"
                                    stroke-width="15"
                                    stroke-opacity="0.5"
                                    fill="none"
                                    stroke-linejoin="round"
                                />
                            },
                        )
                    } else {
                        None
                    }
                }
            }

            <use
                href=href_ref
                on:mouseenter=handle_enter
                stroke=move || stroke().to_string()
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
    #[prop(optional)] on_click: Option<Callback<u32>>,
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
            cb.run(0);
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
