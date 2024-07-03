use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys};
use winit::window::{Window, WindowId};
use yew::prelude::*;
use crate::section::basic_raytracing::component::BasicRaytracing;
use crate::section::prepare_environment::component::PrepareEnvironment;

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
            <div class="container mx-auto">
                <header class="py-6">
                    <h1>{"你好, 计算机图形"}</h1>
                    <p class="my-4">{"这是学习《"}<a href="https://gabrielgambetta.com/computer-graphics-from-scratch/" target="_blank">{"Computer Graphics from Scratch"}</a>{"》的课后实践。"}</p>
                </header>
                <main class="grid grid-flow-col auto-cols-max gap-8">
                    <PrepareEnvironment />
                    <BasicRaytracing />
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

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {}
}

