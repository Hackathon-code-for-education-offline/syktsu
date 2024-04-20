pub mod utils;
pub mod shared;
pub mod pages;

mod app;

use app::router::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
