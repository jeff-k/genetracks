#![warn(clippy::pedantic)]
#![allow(mixed_script_confusables)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

//use core::ops::Range;
use leptos::prelude::*;

mod coverage;
mod puc19;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <div class="header">
                <h1>"Figure demos"</h1>
            </div>

            <coverage::CoverageDemo></coverage::CoverageDemo>
            <puc19::CircularDemo></puc19::CircularDemo>
        </div>
    }
}
