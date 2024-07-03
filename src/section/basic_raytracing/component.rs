use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use winit::window::{Window};
use yew::{Callback, Component, Context, Html, html, NodeRef};
use yew::platform::spawn_local;

use crate::app::AppCallbackContext;
use crate::section::basic_raytracing::graphics::WGPUState;

#[wasm_bindgen]
pub struct BasicRaytracing {
    wgpu_state: Option<WGPUState>,
    canvas: NodeRef,
}

pub enum BasicRaytracingMsg {
    Initialize,
    WindowCreated(Window),
    WGPUInitialized(WGPUState),
    Redraw,
    Nothing,
}

impl Component for BasicRaytracing {
    type Message = BasicRaytracingMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        info!("Creating App");
        BasicRaytracing {
            wgpu_state: None,
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BasicRaytracingMsg::Initialize => {
                info!("Initialize");
            }
            BasicRaytracingMsg::WindowCreated(window) => {
                info!("WindowCreated");
                let cb = ctx.link().callback(BasicRaytracingMsg::WGPUInitialized);

                spawn_local(async move {
                    cb.emit(WGPUState::new(window).await);
                });
            }
            BasicRaytracingMsg::WGPUInitialized(state) => {
                info!("Initialized");

                self.wgpu_state = Some(state);
                ctx.link().send_message(BasicRaytracingMsg::Redraw)
            }
            BasicRaytracingMsg::Redraw => {
                info!("Redrawing");
                self.wgpu_state.as_mut().unwrap().render().expect("TODO: panic message");
            }
            BasicRaytracingMsg::Nothing => {
                info!("Nothing")
            }
        };

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section>
                <h2>{"阶段2: 基本光线追踪"}</h2>
                <p>{"实现基本光线追踪功能。"}</p>
                <canvas ref={self.canvas.clone()} id="basic_raytracing_canvas" width="600" height="600"></canvas>
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        info!("App rendered");
        if first_render {
            info!("App first render");
            let (app_cb, _) = ctx.link().context::<AppCallbackContext>(Callback::noop()).unwrap();

            app_cb.emit((
                self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                ctx.link().callback(BasicRaytracingMsg::WindowCreated)
            ));
        }
    }
}