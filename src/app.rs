use log::info;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys};
use winit::window::{Window, WindowId};
use yew::prelude::*;
use crate::components::raytracer::{RaytracerCanvas};
use crate::components::section::Section;


pub struct App {
    event_loop_proxy: EventLoopProxy<AppMsg>,
}

pub struct Handler;

pub enum AppMsg {
    CreateWindow(HtmlCanvasElement, Callback<Window>)
}

pub type RequestWindowMessage = (HtmlCanvasElement, Callback<Window>);
pub type AppCallbackContext = Callback<RequestWindowMessage>;

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let event_loop = EventLoop::<AppMsg>::with_user_event().build().unwrap();
        let event_loop_proxy = event_loop.create_proxy();
        event_loop.spawn_app(Handler);

        Self {
            event_loop_proxy
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let _ = self.event_loop_proxy.send_event(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link()
            .callback(|(canvas, cb)| AppMsg::CreateWindow(canvas, cb));

        html! {
        <ContextProvider<AppCallbackContext> context={cb}>
            <div class="container mx-auto py-8 justify-center">
                <header class="py-6">
                    <h1>{"你好, 计算机图形"}</h1>
                    <p class="my-4">{"这是学习《"}<a href="https://gabrielgambetta.com/computer-graphics-from-scratch/" target="_blank">{"Computer Graphics from Scratch"}</a>{"》的课后实践。"}</p>
                </header>
                <main class="grid gap-8">
                    <Section title="部分1: 光线追踪器(Raytracer)" subtitle="基于CPU实现的光线追踪器，包括基本光线追踪逻辑、光照效果、阴影与反射光。"><RaytracerCanvas /></Section>

                    // <Section title="阶段1：准备实践环境" subtitle="搭建基于Rust/WebGPU/Yew/Wgpu的实践环境。" shader={include_str!("shaders/prepare_environment.wgsl")}/>
                    // <Section title="阶段2：基本光线追踪" subtitle="实现基本光线追踪功能。" shader={include_str!("shaders/basic_raytracing.wgsl")}/>
                    // <Section title="阶段3：光" subtitle="实现光照效果。" shader={include_str!("shaders/light.wgsl")}/>
                    // <Section title="阶段4：阴影与反射" subtitle="实现物体的阴影和反射光。" shader={include_str!("shaders/shadows_and_reflections.wgsl")}/>
                </main>
            </div>
        </ContextProvider<AppCallbackContext>>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {}
    }
}


impl ApplicationHandler<AppMsg> for Handler {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppMsg) {
        match event {
            AppMsg::CreateWindow(canvas, cb) => {
                // create window and callback to children

                let window_attributes = Window::default_attributes()
                    .with_inner_size(PhysicalSize::new(canvas.width(), canvas.height()))
                    .with_canvas(Some(canvas));

                let window = event_loop.create_window(window_attributes).unwrap();

                spawn_local(async move {
                    cb.emit(window);
                })
            }
        };
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        info!("{:?}", event);
        match event {
            _ => {}
        }
    }
}

