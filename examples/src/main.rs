//use core::ops::Range;
use leptos::ev;
use leptos::prelude::*;

use genetracks::components::{
    Bar, Circular, Figure, FigureTitle, Highlight, Label, Region, Ribbon, Tick, Ticks, Track,
};
use genetracks::elements::{Colour, ElemStyle};
use genetracks::slider::RangeSlider;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

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
        <Track index=2u32>
            <Tick pos=396u32 label="EcoRI".to_string() />
            <Tick pos=402u32 label="SacI".to_string() />
            <Tick pos=408u32 label="KpnI".to_string() />
            <Tick pos=412u32 label="SmaI".to_string() />
            <Tick pos=417u32 label="BamHII".to_string() />
            <Tick pos=423u32 label="XbaI".to_string() />
            <Tick pos=429u32 label="SalI".to_string() />
            <Tick pos=435u32 label="PstI".to_string() />
            <Tick pos=441u32 label="SphI".to_string() />
            <Tick pos=447u32 label="HindIII".to_string() />

            // other sites
            <Tick pos=183u32 label="NdeI".to_string() />
            <Tick pos=806u32 label="PciI".to_string() />
            <Tick pos=1766u32 label="BsaI".to_string() />
            <Tick pos=2177u32 label="ScaI".to_string() />
            <Tick pos=2501u32 label="SspI".to_string() />

            <Bar start=500u32 end=1000u32 />
        </Track>
    }
}

fn _ribbons() -> impl IntoView {
    view! {
        <Ribbon start=(2400u32, 3924u32) end=(12400u32, 12924u32) color=Colour::RoyalBlue />

        <Ribbon
            start=(5474935u32, 5487569u32)
            end=(7109290u32, 7121924u32)
            color=Colour::RoyalBlue
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let length: u32 = 2686;
    let (width, set_width) = signal::<u32>(screen_width() - 100);
    let (height, set_height) = signal::<u32>(screen_height() - 200);

    let _width_handle = window_event_listener(ev::resize, move |_| {
        set_width(screen_width() - 100);
    });

    let _height_handle = window_event_listener(ev::resize, move |_| {
        set_height(screen_height() - 300);
    });

    let (view_range, set_view_range) = signal((0, length));

    view! {
        <div class="app-container">
            <div class="header">
                <h1>"Genome Browser Demo: "<i>"pUC19"</i></h1>
            </div>

            <div class="linear-view">
                <Figure
                    length=length
                    width=width
                    view=view_range
                    tracks=8u32
                    on_scroll=set_view_range
                >
                    <Track index=0u32>
                        <Ticks n=15u32 range=view_range text=true />
                    </Track>

                    <Track index=1u32>
                        <Ticks n=45u32 range=view_range />
                    </Track>

                    <Track index=4u32>
                        <Region
                            start=146u32
                            end=507u32
                            style=ElemStyle::Right
                            color=Colour::LightBlue
                        />

                        <Region
                            start=1629u32
                            end=2489u32
                            style=ElemStyle::Left
                            color=Colour::Salmon
                        />

                        <Region
                            start=1158u32
                            end=1625u32
                            style=ElemStyle::Right
                            color=Colour::LightGreen
                        />
                    </Track>

                    <Track index=5u32>
                        <Region
                            start=396u32
                            end=454u32
                            style=ElemStyle::Right
                            color=Colour::Yellow
                        />
                        <Region
                            start=507u32
                            end=568u32
                            style=ElemStyle::Left
                            color=Colour::Purple
                        />

                        <Region
                            start=1543u32
                            end=2431u32
                            style=ElemStyle::Right
                            color=Colour::Orange
                        />

                    </Track>
                    <Track index=3u32></Track>

                    <Track index=2u32>{restriction_sites()}</Track>

                    <Track index=6u32></Track>

                </Figure>
                <RangeSlider
                    width=width
                    min=0
                    max=length
                    range=view_range
                    set_range=set_view_range
                />
            </div>
            <div class="circular-view">
                <div class="circular-figure">
                    <Circular length=length width=height>

                        <FigureTitle>
                            <tspan font-weight="bold">pUC19</tspan>
                            " (β-gal fragment)"
                        </FigureTitle>
                        <Highlight range=view_range top=0u32 bottom=6u32 color=Colour::Yellow />

                        <Track index=0u32>
                            <Ticks n=15u32 range=(0u32, length) text=true />
                        </Track>

                        <Track index=1u32>
                            <Ticks n=45u32 range=(0u32, length) />
                        </Track>

                        <Track index=3u32>
                            <Region
                                start=146u32
                                end=507u32
                                style=ElemStyle::Right
                                color=Colour::LightBlue
                            ></Region>

                            <Label pos=325u32>
                                <tspan font-weight="bold">lacZα</tspan>
                                "(β-gal fragment)"
                            </Label>

                            <Region
                                start=1629u32
                                end=2489u32
                                style=ElemStyle::Left
                                color=Colour::Salmon
                            />

                            <Label pos=2000u32>"AmpR (ampicillin resistance)"</Label>
                            <Region
                                start=1158u32
                                end=1625u32
                                style=ElemStyle::Right
                                color=Colour::LightGreen
                            />

                            <Label pos=1300u32>"ori (pMB1 origin)"</Label>
                        </Track>

                        <Track index=5u32>
                            <Region
                                start=396u32
                                end=454u32
                                style=ElemStyle::Right
                                color=Colour::Yellow
                            />
                            <Label pos=425u32>"MCS (Multiple Cloning Site)"</Label>

                            <Region
                                start=507u32
                                end=568u32
                                style=ElemStyle::Left
                                color=Colour::Purple
                            />

                            <Label pos=534u32>"lac promotoer"</Label>
                        </Track>

                        <Track index=2u32>{restriction_sites()}</Track>
                    </Circular>
                </div>

            </div>
        </div>
    }
}
