pub mod circular;
pub mod elements;

pub use elements::{ElemStyle, ElementData, TrackData};
//pub use circular;
use leptos::*;

#[component]
pub fn Element(#[prop(into)] height: f32, #[prop(into)] data: ElementData) -> impl IntoView {
    let scale = use_context::<ReadSignal<f32>>().unwrap();

    // these are repeated, but probably shouldn't be memoised
    let start = move || data.start / scale();
    let end = move || (data.end - data.start) / scale();
    let midpoint = move || ((data.end - data.start) / 2.0) / scale();

    let text = data.label.map(|label| {
        view! {
            <text
                x=move || format!("{}", midpoint())
                y=format!("{}", height / 2.0)
                font-size="12"
                font-family="monospace"
                text-anchor="middle"
                dominant-baseline="central"
            >
                {label.to_string()}
            </text>
        }
    });

    let elem = match data.style {
        ElemStyle::Left => view! {
            <path
                d=move || {
                    format!(
                        "M0,0 L{},0 L{},{} L0,{} L{},{} L0,0",
                        end(),
                        end(),
                        height,
                        height,
                        -10.0,
                        height / 2.0,
                    )
                }

                fill=data.colour.to_string()
                stroke=data.colour.to_string()
                stroke-opacity="0.0"
            ></path>
        }
        .into_view(),

        ElemStyle::Right => view! {
            <path
                d=move || {
                    format!(
                        "M0,0 L{},0 L{},{} L{},{} L0,{} L0,0",
                        end(),
                        end() + 10.0,
                        height / 2.0,
                        end(),
                        height,
                        height,
                    )
                }

                fill=data.colour.to_string()
                stroke=data.colour.to_string()
                stroke-opacity="0.0"
            ></path>
        }
        .into_view(),
        ElemStyle::Line => view! {
            <path
                d=move || format!("M0.0,{} L{},{}", height / 2.0, end(), height / 2.0)
                stroke=data.colour.to_string()
            ></path>
        }
        .into_view(),
        ElemStyle::Spacer => view! {}
        .into_view(),
        ElemStyle::Bar => view! {
            <path d=format!("M0.0,0.0 L0.0,{}", height) stroke=data.colour.to_string()></path>
            <path
                d=move || format!("M0.0,{} L{},{}", height / 2.0, end(), height / 2.0)
                stroke=data.colour.to_string()
            ></path>
            <path
                d=move || format!("M{},0.0 L{},{}", end(), end(), height)
                stroke=data.colour.to_string()
            ></path>
        }
        .into_view(),
        ElemStyle::Rect => view! {
            <rect
                x="0"
                y="0"
                width=move || format!("{}", end())
                height=format!("{}", height)
                fill=data.colour.to_string()
                stroke=data.colour.to_string()
                stroke-opacity="0.0"
            ></rect>
        }
        .into_view(),
        ElemStyle::Tick => {
            view! { <path d=format!("M0.0,0.0 L0.0,{}", height) stroke=data.colour.to_string()></path> }
                .into_view()
        }
    };

    view! { <g transform=move || { format!("translate({} 0)", start()) }>{elem} {text}</g> }
}

#[component]
pub fn Track(
    #[prop(into)] y: f32,
    #[prop(into)] elements: Vec<ElementData>,
    #[prop(default = 18.0)] height: f32,
) -> impl IntoView {
    view! {
        <g transform=format!(
            "translate(0 {})",
            y,
        )>{elements.into_iter().map(|e| view! { <Element height data=e /> }).collect_view()}</g>
    }
}

pub fn from_json(json: &str) -> Vec<TrackData> {
    serde_json::from_str::<Vec<TrackData>>(json).unwrap_or_default()
}

#[component]
pub fn Figure(
    #[prop(default = 0.0)] x: f32,
    #[prop(default = 0.0)] y: f32,
    #[prop(into)] width: ReadSignal<f32>,
    #[prop(into)] tracks: ReadSignal<Vec<(usize, TrackData)>>,
) -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    set_scale(1.0);
    provide_context(scale);
    set_scale(1.0);
    let (height, set_height) = create_signal(0.0);

    create_effect(move |_| {
        let mut max_end: f32 = 0.0;
        let mut h: f32 = 0.0;
        for (_, track) in tracks() {
            for elem in track.elements {
                max_end = f32::max(max_end, elem.end);
            }
            h += track.height;
        }

        set_height(h);
        if max_end > 0.0 {
            set_scale(max_end / width());
        }
    });

    view! {
        <svg
            width=width
            height=height
            viewBox=move || { format!("{} {} {} {}", x, y, width(), height()) }
            preserveAspectRatio="none"
        >
            <For
                each=move || tracks().into_iter().enumerate()
                key=move |(_, (id, _))| *id
                children=move |(index, (_, track))| {
                    let t = &track.clone();
                    view! { <Track y=index as f32 * t.height elements=t.elements.clone() /> }
                }
            />

        </svg>
    }
}
