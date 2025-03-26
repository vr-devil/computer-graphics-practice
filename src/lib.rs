use webapp::app::App;

pub mod graphics;

pub mod webapp;

pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

