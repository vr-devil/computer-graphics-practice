use web_sys::HtmlCanvasElement;
use winit::window::Window;
use yew::{Callback, Component, Context, Html, html, NodeRef};
use yew::platform::spawn_local;

use crate::app::AppCallbackContext;
use crate::graphics::WGPUState;

pub struct Light {
    canvas: NodeRef,
}

pub enum LightMsg {
    WindowCreated(Window),
    WGPUInitialized(WGPUState)
}

impl Component for Light {
    type Message = LightMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LightMsg::WindowCreated(window) => {
                let cb = ctx.link().callback(LightMsg::WGPUInitialized);
                spawn_local(async move {
                    cb.emit(WGPUState::new(window, include_str!("shader.wgsl")).await)
                })
            }
            LightMsg::WGPUInitialized(mut state) => {
                state.render().expect("failed to render.");
                // self.wgpu_state = Some(state);
            }
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="w-full">
                <h2>{"阶段3：光"}</h2>
                <p class="my-4">{"实现光照效果。"}</p>
                <canvas
                    ref={self.canvas.clone()}
                    id="light_canvas"
                    width="600"
                    height="600"
                />
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let (cb, _) =ctx.link()
                .context::<AppCallbackContext>(Callback::noop())
                .unwrap();

            cb.emit((
                self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                ctx.link().callback(LightMsg::WindowCreated)
            ));
        }
    }
}