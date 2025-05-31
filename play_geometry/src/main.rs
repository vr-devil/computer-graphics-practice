use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use web_sys::{wasm_bindgen::JsCast, window, HtmlCanvasElement};
use wgpu::SurfaceTarget;
use play_geometry::WGPUInstance;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        h1 { "VR-DEVIL's Graphics" }
        Line {}
    }
}

#[component]
pub fn Line() -> Element {
    // let mut canvas = use_signal(|| None);
    use_effect(move || {
        info!("wgpu_instance created.1");

        // canvas.set(get_canvas("line-canvas"));
        spawn(async {
            let el = get_canvas("line-canvas").unwrap();
            let wgpu_instance = WGPUInstance::new(SurfaceTarget::Canvas(el)).await;
            info!("wgpu_instance created.2");
        });

    });

    rsx! {
        div {
            canvas { id: "line-canvas" }
        }
    }
}

fn get_canvas(id: &str) -> Option<HtmlCanvasElement> {
    window().and_then(|win| win.document()).and_then(|doc| {
        doc.get_element_by_id(id)
            .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok())
    })
}
