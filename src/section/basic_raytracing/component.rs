use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::web;
use winit::platform::web::EventLoopExtWebSys;
use winit::window::{Window, WindowId};
use yew::{Callback, Component, Context, Html, html, NodeRef};
use yew::platform::spawn_local;

use crate::section::basic_raytracing::graphics::WGPUState;

#[wasm_bindgen]
pub struct BasicRaytracing {
    wgpu_state: Option<WGPUState>,
    canvas: NodeRef,
}

pub struct Handler {
    canvas: HtmlCanvasElement,
    wgpu_callback: Callback<WGPUState>,
}

pub enum AppMsg {
    Initialize,
    Initialized(WGPUState),
    Redraw,
    Nothing,
}

impl Component for BasicRaytracing {
    type Message = AppMsg;
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
            AppMsg::Initialize => {
                info!("Initialize");

                let wgpu_cb = ctx.link().callback(AppMsg::Initialized);
                let handler = Handler {
                    canvas: self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                    wgpu_callback: wgpu_cb,
                };

                let event_loop = EventLoop::new().unwrap();
                event_loop.set_control_flow(ControlFlow::Wait);
                event_loop.spawn_app(handler);

            }
            AppMsg::Initialized(state) => {
                info!("Initialized");

                self.wgpu_state = Some(state);
                ctx.link().send_message(AppMsg::Redraw)
            }
            AppMsg::Redraw => {
                info!("Redrawing");
                self.wgpu_state.as_mut().unwrap().render().expect("TODO: panic message");
            }
            AppMsg::Nothing => {
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
            ctx.link().send_message(AppMsg::Initialize);
        }
    }
}

impl ApplicationHandler for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("App handler resumed");

        use web::WindowAttributesExtWebSys;

        let canvas = self.canvas.clone();
        let window_attributes = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(canvas.width(), canvas.height()))
            .with_canvas(Some(canvas));

        let window = event_loop.create_window(window_attributes).unwrap();
        let cb = self.wgpu_callback.clone();

        spawn_local(async move {
            cb.emit(WGPUState::new(window).await);
        });

    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {}

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        info!("App handler suspended");
    }
}