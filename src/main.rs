mod app;
mod components;
mod data;
mod util;

use app::App;

fn main() {
    // Panics become readable console errors instead of "unreachable executed".
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
