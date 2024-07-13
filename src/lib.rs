use crate::app::App;

mod app;
pub mod graphics;

pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

