//use core::ops::Range;
use leptos::ev;
use leptos::logging;
use leptos::prelude::*;

use genetracks::components::{
    Bar, Circular, Figure, FigureTitle, Highlight, Label, Region, Ribbon, Tick, Ticks, Track,
};
use genetracks::elements::{Colour, ElemStyle};
//use genetracks::slider::RangeSlider;

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
        <Tick pos=2550u32 />
        <Tick pos=2655u32 />
        <Tick pos=265_932 />
        <Tick pos=273u32 />
        <Tick pos=390u32 />
        <Tick pos=632u32 />
        <Tick pos=637u32 />
        <Tick pos=683u32 />
        <Tick pos=844u32 />
        <Tick pos=848u32 />

        // other sites
        <Tick pos=1032u32 />
        <Tick pos=1091u32 />
        <Tick pos=1150u32 />
        <Tick pos=1266u32 />
        <Tick pos=1471u32 />
        <Tick pos=1590u32 />
        <Tick pos=1673u32 />
        <Tick pos=1924u32 />
        <Tick pos=1986u32 />
        <Tick pos=2002u32 />
        <Tick pos=2071u32 />
    }
}

fn restriction_site_labels() -> impl IntoView {
    view! {
        <Label pos=2550u32 curve=false>
            "AlwNI"
        </Label>
        <Label pos=2655u32 curve=false>
            "BseYI"
        </Label>
        <Label pos=2659u32 curve=false>
            "PspFI"
        </Label>
        <Label pos=273u32 curve=false>
            "AflIII"
        </Label>
        <Label pos=390u32 curve=false>
            "BspQI"
        </Label>
        <Label pos=632u32 curve=false>
            "HindIII"
        </Label>
        <Label pos=637u32 curve=false>
            "BfuAI"
        </Label>
        <Label pos=683u32 curve=false>
            "ApoI"
        </Label>
        <Label pos=844u32 curve=false>
            "KasI"
        </Label>
        <Label pos=848u32 curve=false>
            "PluTI"
        </Label>

        // other sites
        <Label pos=1032u32 curve=false>
            "PfoI"
        </Label>
        <Label pos=1091u32 curve=false>
            "EcoO109I"
        </Label>
        <Label pos=1150u32 curve=false>
            "ZraI"
        </Label>
        <Label pos=1266u32 curve=false>
            "SspI"
        </Label>
        <Label pos=1471u32 curve=false>
            "XmnI"
        </Label>
        <Label pos=1590u32 curve=false>
            "ScaI"
        </Label>
        <Label pos=1673u32 curve=false>
            "TsoI"
        </Label>
        <Label pos=1924u32 curve=false>
            "NmeAIII"
        </Label>
        <Label pos=1986u32 curve=false>
            "BsrFI"
        </Label>
        <Label pos=2002u32 curve=false>
            "BpmI"
        </Label>
        <Label pos=2071u32 curve=false>
            "AhdI"
        </Label>
    }
}

fn _ribbons() -> impl IntoView {
    view! {
        <Ribbon start=(2400u32, 3924u32) end=(12400u32, 12924u32) color=Colour::RoyalBlue />

        <Ribbon
            start=(5_474_935u32, 5_487_569u32)
            end=(7_109_290u32, 7_121_924u32)
            color=Colour::RoyalBlue
        />
    }
}

#[component]
pub fn CircularDemo() -> impl IntoView {
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
        <div class="app-container">
            <div class="header">
                <h1>"Genome Browser Demo: "<i>"pUC19"</i></h1>
            </div>

            <div class="circular-view">
                <div class="circular-figure">
                    <Circular length=length>

                        <FigureTitle>
                            <tspan font-weight="bold">pUC19</tspan>
                            " (β-gal fragment)"
                        </FigureTitle>
                        <Highlight range=view_range top=0u32 bottom=6u32 color=Colour::Yellow />

                        <Track index=4u32>
                            <Ticks n=15u32 range=(45u32, length) text=true />
                        </Track>

                        <Track index=5u32>
                            <Ticks n=45u32 range=(0u32, length) />
                            <Bar
                                start=1u32
                                end=length
                                style=ElemStyle::DoubleLine
                                color=Colour::Black
                            />
                        </Track>

                        <Track index=7u32>
                            <Bar start=1285u32 end=2144u32 style=ElemStyle::Right />
                        </Track>
                        <Track index=8u32>
                            <Label pos=515u32>"CAP binding"</Label>
                        </Track>
                        <Track index=10u32>
                            <Region
                                start=2315u32
                                end=217u32
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

                            <Region
                                start=579u32
                                end=595u32
                                style=ElemStyle::None
                                color=Colour::MediumAquamarine
                                stroke=Colour::Red
                            />

                            <Region
                                start=615u32
                                end=938u32
                                style=ElemStyle::ArrowRight
                                color=Colour::GoldenRod
                            />

                            <Region
                                start=1179u32
                                end=1283u32
                                style=ElemStyle::ArrowRight
                                color=Colour::White
                            />

                            <Region
                                start=1284u32
                                end=2144u32
                                style=ElemStyle::ArrowRight
                                color=Colour::LightGreen
                            />

                        </Track>

                        <Track index=12u32>
                            <Label pos=750u32>
                                <tspan font-weight="bold">lacZα</tspan>
                                " (β-gal frag.)"
                            </Label>

                            <Label pos=1700u32>
                                <tspan font-weight="bold">AmpR</tspan>
                            </Label>
                            <Label pos=2650u32>"ori (pMB1 origin)"</Label>

                            <Label pos=550u32 color=hover_label_stroke on_hover=handle_label_hover>
                                "lac promoter"
                            </Label>
                        </Track>

                        <Track index=1u32>{restriction_site_labels()}</Track>
                        <Track index=3u32>{restriction_sites()}</Track>
                    </Circular>
                </div>

            </div>
        </div>
    }
}
