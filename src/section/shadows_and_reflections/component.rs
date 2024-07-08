use web_sys::HtmlCanvasElement;
use winit::window::Window;
use yew::{Callback, Component, Context, Html, html, NodeRef};
use yew::platform::spawn_local;

use crate::app::AppCallbackContext;
use crate::section::shadows_and_reflections::graphics::WGPUState;

pub struct ShadowsAndReflections {
    canvas: NodeRef,
    wgpu_state: Option<WGPUState>
}

pub enum ShadowsAndReflectionsMsg {
    WindowCreated(Window),
    WGPUInitialized(WGPUState)
}

impl Component for ShadowsAndReflections {
    type Message = ShadowsAndReflectionsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
            wgpu_state: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ShadowsAndReflectionsMsg::WindowCreated(window) => {
                let cb = ctx.link().callback(ShadowsAndReflectionsMsg::WGPUInitialized);
                spawn_local(async move {
                    cb.emit(WGPUState::new(window).await)
                })
            }
            ShadowsAndReflectionsMsg::WGPUInitialized(mut state) => {
                state.render().expect("failed to render.");
                // self.wgpu_state = Some(state);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class="w-full">
                <h2>{"阶段4：阴影与反射"}</h2>
                <p class="my-4">{"实现物体的阴影和反射光。"}</p>
                <canvas
                    ref={self.canvas.clone()}
                    id="shadows_and_reflections_canvas"
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
                ctx.link().callback(ShadowsAndReflectionsMsg::WindowCreated)
            ));
        }
    }
}