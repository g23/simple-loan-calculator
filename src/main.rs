mod components;
mod fin_math;
mod utils;

use leptos::*;

use crate::components::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
