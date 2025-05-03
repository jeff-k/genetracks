use leptos::prelude::*;

//use genetracks::circular as c;
//use console_error_panic_hook;
use leptos::logging;

use genetracks::components::{
    Bar, Circular, Figure, Highlight, Label, Region, Ribbon, Ticks, Track,
};
use genetracks::elements::{Colour, ElemStyle};

fn main() {
    //    console_error_panic_hook::set_once();
    mount_to_body(App);
}

fn myfig() -> impl IntoView {
    let (h, seth) = signal(Colour::Red);
    let hcb = Callback::new(move |s| {
        logging::log!("hovering {s:?}");
        if s {
            seth.set(Colour::BlueViolet);
        } else {
            seth.set(Colour::DarkOrange);
        }
    });

    let ccb = Callback::new(move |_: u32| {
        logging::log!("clicked");
        seth.set(Colour::DarkOliveGreen);
    });

    view! {
        <Track index=4u32>
            <Bar start=0 end=500 style=ElemStyle::Left color=h on_hover=hcb />
            <Bar start=500 end=750 style=ElemStyle::Right color=Colour::Red>
                <Label pos=u32::midpoint(500, 700)>"right bar"</Label>
            </Bar>
            <Bar start=4250 end=5000 color=Colour::Red style=ElemStyle::Left>
                <Label pos=u32::midpoint(4250, 5000)>"left bar"</Label>
            </Bar>
            <Bar start=1100 end=1101 color=Colour::RoyalBlue style=ElemStyle::Left />

            <Bar start=900 end=901 color=Colour::RoyalBlue style=ElemStyle::Right />
            <Bar start=3500 end=4250 style=ElemStyle::Right />
        </Track>

        <Track index=3u32>
            <Region start=0 end=500 style=ElemStyle::Left on_click=ccb />
            <Region start=500 end=750 style=ElemStyle::Right color=h on_click=ccb on_hover=hcb>
                <Label pos=u32::midpoint(500, 750)>"right bar"</Label>
            </Region>
            <Region start=4250 end=5000 color=Colour::Red style=ElemStyle::Left></Region>

            <Region start=1000 end=1001 color=Colour::RoyalBlue style=ElemStyle::Left />

            <Region start=950 end=951 color=Colour::RoyalBlue style=ElemStyle::Right />

            <Region start=3500 end=4250 style=ElemStyle::Right />

            <Label pos=u32::midpoint(4250, 5000)>"left bar"</Label>
        </Track>
    }
}

#[component]
fn App() -> impl IntoView {
    let (length, set_length) = signal::<u32>(5000);
    let (width, set_width) = signal::<u32>(1000);

    let (view_range, set_view_range) = signal::<(u32, u32)>((0, 5000));

    let set_start = move |start| {
        let (_, end) = view_range();
        set_view_range((start, end));
    };

    let set_end = move |end| {
        let (start, _) = view_range();
        set_view_range((start, end));
    };

    let onhov = move |h| {
        logging::log!("setting hover status {h:?}");
    };

    let (rib_clr, set_clr) = signal::<Colour>(Colour::Red);

    let rib_clk = Callback::new(move |_: u32| {
        set_view_range.set((500, 750));
    });

    let rib_hov = Callback::new(move |h| {
        if h {
            set_clr.set(Colour::GoldenRod);
        } else {
            set_clr.set(Colour::Red);
        }
    });

    view! {
        <div>
            <label>
                "width"
                <input
                    type="range"
                    min="20"
                    max="1000"
                    step="20"
                    prop:value=width
                    on:input=move |ev| {
                        set_width(event_target_value(&ev).parse::<u32>().unwrap_or(1000))
                    }
                /> {move || format!("{:.1}", width())}
            </label>
        </div>

        <div>
            <label>
                "length"
                <input
                    type="range"
                    min="5000"
                    max="50000"
                    step="20"
                    prop:value=length
                    on:input=move |ev| {
                        set_length(event_target_value(&ev).parse::<u32>().unwrap_or(5000))
                    }
                /> {move || format!("{:.1}", length())}
            </label>
        </div>

        <div>
            <label>
                "start"
                <input
                    type="range"
                    min="0"
                    max=move || view_range().1 - 20
                    step="20"
                    prop:value=move || view_range().0
                    on:input=move |ev| {
                        set_start(event_target_value(&ev).parse::<u32>().unwrap_or(0))
                    }
                /> {move || format!("{:.1}", view_range().0)}
            </label>
        </div>

        <div>
            <label>
                "end"
                <input
                    type="range"
                    min=move || view_range().0 + 20
                    max="5000"
                    step="20"
                    prop:value=move || view_range().1
                    on:input=move |ev| {
                        set_end(event_target_value(&ev).parse::<u32>().unwrap_or(5000))
                    }
                /> {move || format!("{:.1}", view_range().1)}
            </label>
        </div>
        <Figure length view=view_range on_scroll=set_view_range>

            <Highlight top=0u32 bottom=5u32 range=(1000, 2000) color=Colour::Yellow />
            <Track index=0u32>

                <Ribbon start=(2500, 4000) end=(1250, 3500) target=Some(4) />
                <Ticks n=10u32 range=view_range text=true />

            </Track>

            <Track index=1u32>
                <Ticks n=20u32 range=view_range />

            </Track>
            <Track index=1u32>
                <Bar start=0 end=5000 />
            </Track>
            {myfig()}

        </Figure>

        <p />
        <Circular length>

            <Highlight top=0u32 bottom=7u32 range=view_range color=Colour::LemonChiffon />
            <Track index=4u32>
                <Ribbon
                    start=(500, 750)
                    end=(4250, 5000)
                    on_hover=rib_hov
                    on_click=rib_clk
                    color=rib_clr
                />
                <Ribbon start=(2000, 2050) end=(4000, 4500) color=Colour::RoyalBlue />
            </Track>

            <Track index=0u32>
                <Ticks n=10u32 range=(0, 5000) text=true />

                <Ticks n=20u32 range=(0, 5000) />
            </Track>

            <Track index=1u32>
                <Bar start=0 end=5000 on_hover=Callback::new(onhov) />
            </Track>
            {myfig()}

        </Circular>

        <Figure length />

        <Circular length=length>
            <Track index=0u32>
                <Region start=0 end=500 style=ElemStyle::Left></Region>
            </Track>
            <Track index=1u32 />
        </Circular>

        <Circular length />
    }
}
