use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use winit::window::{Window};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::app::AppCallbackContext;
use crate::graphics::WGPUState;

#[wasm_bindgen]
pub struct PrepareEnvironment {
    wgpu_state: Option<WGPUState>,
    canvas: NodeRef,
}

pub enum PrepareEnvironmentMsg {
    Initialize,
    WindowCreated(Window),
    WGPUInitialized(WGPUState),
    Redraw,
    Nothing,
}

impl Component for PrepareEnvironment {
    type Message = PrepareEnvironmentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        info!("Creating App");
        PrepareEnvironment {
            wgpu_state: None,
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PrepareEnvironmentMsg::Initialize => {
                info!("Initialize");
            }
            PrepareEnvironmentMsg::WindowCreated(window) => {
                info!("WindowCreated");
                let cb = ctx.link().callback(PrepareEnvironmentMsg::WGPUInitialized);
                spawn_local(async move {
                    cb.emit(WGPUState::new(window,include_str!("shader.wgsl")).await);
                });
            }
            PrepareEnvironmentMsg::WGPUInitialized(state) => {
                info!("WGPUInitialized");

                self.wgpu_state = Some(state);
                ctx.link().send_message(PrepareEnvironmentMsg::Redraw)
            }
            PrepareEnvironmentMsg::Redraw => {
                info!("Redraw");
                self.wgpu_state.as_mut().unwrap().render().expect("TODO: panic message");
            }
            PrepareEnvironmentMsg::Nothing => {
                info!("Nothing")
            }
        };

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="w-full">
                <h2>{"阶段1：准备实践环境"}</h2>
                <p class="my-4">{"搭建基于Rust/WebGPU/Yew/Wgpu的实践环境。"}</p>
                <canvas ref={self.canvas.clone()} id="canvas" width="600" height="600"></canvas>
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
                ctx.link().callback(PrepareEnvironmentMsg::WindowCreated)
            ));

        }
    }
}
