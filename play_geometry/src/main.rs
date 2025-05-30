use dioxus::prelude::*;
use web_sys::{wasm_bindgen::JsCast, window, HtmlCanvasElement};

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
    dioxus::launch(App);
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
    let mut canvas = use_signal(|| None);
    use_effect(move || canvas.set(find_canvas("line-canvas")));

    rsx! {
        div {
            canvas { id: "line-cavans" }
        }
    }
}

fn find_canvas(id: &str) -> Option<HtmlCanvasElement> {
    let document = window().unwrap().document().unwrap();
    let elment = document.get_element_by_id(id).unwrap();
    elment
        .dyn_into::<HtmlCanvasElement>()
        .map_or(None, |e| Some(e))
}
