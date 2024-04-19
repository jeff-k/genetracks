use leptos::*;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use std::fmt;
//use std::rc::Rc;

pub mod elements;
use elements::{Dims, ElemStyle, ElementData, TrackData};

#[component]
fn Text(pos: elements::Dims, text: String) -> impl IntoView {
    view! {
        <text
            x=pos.x.to_string()
            y=pos.y.to_string()
            font-size="10"
            font-family="monospace"
            text-anchor="middle"
            dominant-baseline="central"
        >
            {text.to_string()}
        </text>
    }
}

#[component]
fn Rect(pos: Dims, text: String) -> impl IntoView {
    view! {
        <text
            x=pos.x.to_string()
            y=pos.y.to_string()
            font-size="10"
            font-family="monospace"
            text-anchor="middle"
            dominant-baseline="central"
        >
            {text.to_string()}
        </text>
    }
}

#[component]
pub fn Element(#[prop(into)] height: f32, #[prop(into)] data: ElementData) -> impl IntoView {
    let style = data.style;
    let label = data.label;
    let start = data.start / data.scale;
    let colour = data.colour;
    let length = data.end - data.start;

    let end: f32 = length / data.scale;
    let midpoint = (length / 2.0) / data.scale;
    let start = (start) / data.scale;

    let text = label.map(|label| {
        view! {
            <text
                x=format!("{}", midpoint)
                y=format!("{}", height / 2.0)
                font-size="12"
                font-family="monospace"
                text-anchor="middle"
                dominant-baseline="middle"
            >
                {label.to_string()}
            </text>
        }
    });

    let elem = match style {
        ElemStyle::Left => view! {
            <path
                d=format!(
                    "M0,0 L{},0 L{},{} L0,{} L{},{} L0,0",
                    end,
                    end,
                    height,
                    height,
                    -10.0,
                    height / 2.0,
                )

                fill=colour.to_string()
                stroke=colour.to_string()
                stroke-opacity="0.0"
            ></path>
        }
        .into_view(),

        ElemStyle::Right => view! {
            <path
                d=format!(
                    "M0,0 L{},0 L{},{} L{},{} L0,{} L0,0",
                    end,
                    end + 10.0,
                    height / 2.0,
                    end,
                    height,
                    height,
                )

                fill=colour.to_string()
                stroke=colour.to_string()
                stroke-opacity="0.0"
            ></path>
        }
        .into_view(),
        ElemStyle::Line => view! {
            <path
                d=format!("M0.0,{} L{},{}", height / 2.0, end, height / 2.0)
                stroke=colour.to_string()
            ></path>
        }
        .into_view(),
        ElemStyle::Bar => view! {
            <path d=format!("M0.0,0.0 L0.0,{}", height) stroke=colour.to_string()></path>
            <path
                d=format!("M0.0,{} L{},{}", height / 2.0, end, height / 2.0)
                stroke=colour.to_string()
            ></path>
            <path d=format!("M{},0.0 L{},{}", end, end, height) stroke=colour.to_string()></path>
        }
        .into_view(),
        ElemStyle::Rect => view! {
            <rect
                x="0"
                y="0"
                width=format!("{}", end)
                height=format!("{}", height)
                fill=colour.to_string()
                stroke=colour.to_string()
                stroke-opacity="0.0"
            ></rect>
        }
        .into_view(),
        ElemStyle::Tick => {
            view! { <path d=format!("M0.0,0.0 L0.0,{}", height) stroke=colour.to_string()></path> }
                .into_view()
        }
    };

    view! { <g transform=format!("translate({} 0)", start)>{elem} {text}</g> }
}

#[component]
pub fn Track(
    #[prop(into)] y: f32,
    #[prop(into)] elements: Vec<ElementData>,
    #[prop(default = 20.0)] height: f32,
) -> impl IntoView {
    view! {
        <g transform=format!(
            "translate(0 {})",
            y,
        )>{elements.into_iter().map(|e| view! { <Element height data=e/> }).collect_view()}</g>
    }
}

#[component]
pub fn Figure(#[prop(into)] dims: Dims, #[prop(into)] tracks: Vec<TrackData>) -> impl IntoView {
    let v = view! {
        <svg
            width=dims.width
            height=dims.height
            viewBox=format!("0.0 0.0 {} {}", dims.width, dims.height)
            preserveAspectRatio="none"
        >

            {
                let mut y: f32 = 0.0;
                tracks
                    .into_iter()
                    .map(|t| {
                        let v = view! { <Track y elements=t.elements/> };
                        y += &t.height;
                        logging::log!("tracking track at {:?}", & y);
                        v
                    })
                    .collect_view()
            }

        </svg>
    };

    logging::log!("{:?}", &v.clone().into_view());
    v
}
