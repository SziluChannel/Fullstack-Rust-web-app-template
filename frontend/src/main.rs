pub mod app;
use app::App;
pub mod modules;
pub const BACKEND: &str = "http://localhost:8888";

fn main() {
    yew::Renderer::<App>::new().render();
}
