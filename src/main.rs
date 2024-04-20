pub mod components;
pub mod pages;
pub mod shared;
pub mod utils;

mod app;

use app::router::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    yew::Renderer::<App>::new().render();
}
