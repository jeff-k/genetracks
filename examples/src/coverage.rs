#![warn(clippy::pedantic)]
#![allow(mixed_script_confusables)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

//use core::ops::Range;
use leptos::ev;
use leptos::logging;
use leptos::prelude::*;

use genetracks::components::{
    Bar, Figure, FigureTitle, Highlight, Label, Region, Ribbon, Tick, Ticks, Track,
};
use genetracks::elements::{Colour, ElemStyle};
//use genetracks::slider::RangeSlider;
//

fn screen_width() -> u32 {
    if let Ok(width) = window().outer_width() {
        width.as_f64().unwrap() as u32
    } else {
        800u32
    }
}

fn screen_height() -> u32 {
    if let Ok(height) = window().outer_height() {
        height.as_f64().unwrap() as u32
    } else {
        600u32
    }
}

fn restriction_sites() -> impl IntoView {
    view! {
        <Tick pos=255u32 />
        <Tick pos=865u32 />
    }
}

fn restriction_site_labels() -> impl IntoView {
    view! {
        <Label pos=255u32 curve=false>
            "AlwNI"
        </Label>
        <Label pos=865u32 curve=false>
            "BseYI"
        </Label>
        <Label pos=1265u32 curve=false>
            "PspFI"
        </Label>
    }
}

#[component]
pub fn CoverageDemo() -> impl IntoView {
    let length: u32 = 2686;
    let (_width, set_width) = signal::<u32>(screen_width() - 100);
    let (_height, set_height) = signal::<u32>(screen_height() - 200);

    let _width_handle = window_event_listener(ev::resize, move |_| {
        logging::log!("resized width to {}", screen_width());
        set_width(screen_width() - 100);
    });

    let _height_handle = window_event_listener(ev::resize, move |_| {
        set_height(screen_height() - 300);
    });

    let (view_range, _set_view_range) = signal((0, length));

    let (selected, set_selected) = signal(false);
    let handle_region_click = Callback::new(move |i: u32| {
        logging::log!("clicked on region {i}");
        if selected() {
            set_selected(false);
        } else {
            set_selected(true);
        }
    });

    let (hover_label_stroke, set_hover_label_stroke) = signal(Colour::Black);
    let handle_label_hover = Callback::new(move |hovering| {
        set_hover_label_stroke(if hovering { Colour::Red } else { Colour::Black });
    });
    let (hover_region_stroke, set_hover_region_stroke) = signal(Colour::Black);
    let handle_region_hover = Callback::new(move |hovering| {
        set_hover_region_stroke(if hovering { Colour::Red } else { Colour::Black });
    });
    view! {
        <div class="linear-figure">
            <Figure length=length tracks=12>

                <Track index=0u32>
                    <Ticks n=15u32 range=(45u32, length) text=true />
                    restricion_sites()
                </Track>

                <Track index=2u32>
                    <Ticks n=45u32 range=(0u32, length) />
                    <Bar start=1u32 end=length style=ElemStyle::Line color=Colour::Black />
                </Track>

                <Track index=3u32>
                    <Bar start=1u32 end=length style=ElemStyle::DoubleLine color=Colour::Black />
                </Track>

                <Track index=4u32>
                    <Bar start=500u32 end=1500u32 style=ElemStyle::Right />
                    {restriction_site_labels()}
                </Track>
                <Track index=6u32>
                    <Label pos=515u32>"CAP binding"</Label>
                    {restriction_sites()}
                </Track>
                <Track index=8u32>
                    <Region
                        start=315u32
                        end=417u32
                        style=ElemStyle::ArrowRight
                        color=Colour::Plum
                        stroke=hover_region_stroke
                        on_hover=handle_region_hover
                        on_click=handle_region_click
                        selected=selected
                    ></Region>

                    <Region
                        start=505u32
                        end=526u32
                        style=ElemStyle::None
                        color=Colour::MediumAquamarine
                    />

                    <Region
                        start=541u32
                        end=571u32
                        style=ElemStyle::ArrowRight
                        color=Colour::White
                    />
                </Track>

            </Figure>
        </div>
    }
}
