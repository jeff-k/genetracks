use leptos::prelude::*;

#[component]
pub fn RangeSlider(
    #[prop(into)] width: Signal<u32>,
    #[prop(into)] min: Signal<u32>,
    #[prop(into)] max: Signal<u32>,
    #[prop(into)] range: Signal<(u32, u32)>,
    #[prop(into)] set_range: WriteSignal<(u32, u32)>,
) -> impl IntoView {
    let start = move || range().0;
    let end = move || range().1;

    let on_start = move |start| {
        set_range((start, end()));
    };

    let on_end = move |end| {
        set_range((start(), end));
    };

    let handle_start_change = move |ev| {
        let value = event_target_value(&ev).parse::<u32>().unwrap();
        let new_start = value.min(end().saturating_sub(1000));
        on_start(new_start);
    };

    let handle_end_change = move |ev| {
        if let Ok(value) = event_target_value(&ev).parse::<u32>() {
            if value <= max() && value >= min() {
                let new_end = value.max(start().saturating_add(1000));
                on_end(new_end);
            }
        };
    };

    let range_style = move || {
        let total_width = width() as f64;
        let scale = max() as f64;
        let left_pos = (start() as f64 / scale) * total_width;
        let right_pos = (end() as f64 / scale) * total_width;

        format!("left: {}px; width: {}px", left_pos, right_pos - left_pos)
    };

    view! {
        <div class="range-slider" style=move || format!("width: {}px", width())>
            <div class="range-slider__container">
                <div class="range-slider__track"></div>
                <div class="range-slider__range" style=range_style></div>

                <div class="range-slider__inputs">
                    <input
                        type="range"
                        class="range-slider__input range-slider__input--start"
                        prop:min=min
                        prop:max=max
                        prop:value=start
                        on:input=handle_start_change
                    />

                    <input
                        type="range"
                        class="range-slider__input range-slider__input--end"
                        prop:min=min
                        prop:max=max
                        prop:value=end
                        on:input=handle_end_change
                    />
                </div>
            </div>

            <div class="range-slider__values">
                <input
                    type="number"
                    class="range-slider__value-input"
                    step=1000
                    prop:value=start
                    on:input=handle_start_change
                />
                <input
                    type="number"
                    class="range-slider__value-input"
                    step=1000
                    prop:value=end
                    on:input=handle_end_change
                />

            </div>
        </div>
    }
}
