use core::ops::Range;
use leptos::ev;
use leptos::prelude::*;

use genetracks::components::{
    Bar, Circular, Figure, Highlight, Label, Region, Ribbon, Ticks, Track,
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

fn ribbons() -> impl IntoView {
    view! {
        <Ribbon start=(2400u32, 3924u32) end=(12400u32, 12924u32) color=Colour::RoyalBlue />

        <Ribbon
            start=(5474935u32, 5487569u32)
            end=(7109290u32, 7121924u32)
            color=Colour::RoyalBlue
        />
    }
}
/*
#[component]
fn GeneIntervals(
    #[prop(into)] intervals: StoredValue<IntervalTree<u32, String>>,
    #[prop(into, optional)] range: Option<Signal<(u32, u32)>>,
) -> impl IntoView {
    let genes = match range {
        Some(range) => {
            Memo::new(move |_| {
                let range = range();
                if range.1 - range.0 > 90000 {
                    return vec![];
                }

                let q = Range {
                    start: range.0,
                    end: range.1,
                };

                let ovls: Vec<_> = intervals
                    .get_value()
                    .iter_overlaps(&q)
                    .map(|(interval, gene)| (gene.clone(), interval.start, interval.end))
                    .collect();
                //logging::log!("interval: {}", ovls.len());
                ovls
            })
        }
        None => Memo::new(move |_| {
            intervals
                .get_value()
                .iter()
                .map(|(interval, gene)| (gene.clone(), interval.start, interval.end))
                .collect()
        }),
    };

    view! {
        <For
            each=genes
            key=|(gene, start, end)| format!("{gene}-{start}-{end}")
            children=move |(gene, start, end)| {
                let pos = start + ((end - start) / 2);
                logging::log!("gene: {pos}");
                view! {
                    <Bar start=start end=end color=Colour::LightGrey />
                    <Label pos=pos>{gene}</Label>
                }
            }
        />
    }
}
*/
#[component]
fn App() -> impl IntoView {
    let length: u32 = 54321;
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
                <h1>"Genome Browser Demo"</h1>
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
                            start=0u32
                            end=length
                            style=ElemStyle::Left
                            color=Colour::LightBlue
                        />
                    </Track>

                    <Track index=5u32></Track>

                    <Track index=3u32></Track>

                    <Track index=2u32>ribbons()</Track>

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

                        <Highlight range=view_range top=0u32 bottom=6u32 color=Colour::Yellow />

                        <Track index=0u32>
                            <Ticks n=15u32 range=(0u32, length) text=true />
                        </Track>

                        <Track index=1u32>
                            <Ticks n=45u32 range=(0u32, length) />
                        </Track>

                        <Track index=3u32>// edge_3a,279160,LightGrey,-,7071469,7350629
                        </Track>
                        <Track index=2u32></Track>

                        <Track index=4u32></Track>

                        <Track index=6u32></Track>

                    </Circular>
                </div>

            </div>
        </div>
    }
}
