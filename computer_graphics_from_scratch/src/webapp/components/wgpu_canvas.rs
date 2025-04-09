use winit::window::Window;
use yew::{AttrValue, Callback, classes, Component, Context, Html, html, NodeRef, Properties};
use web_sys::HtmlCanvasElement;
use crate::webapp::app::AppCallbackContext;
use crate::graphics::WGPUState;

pub struct WGPUCanvas {
    canvas: NodeRef,
}

pub enum WGPUCanvasMsg {
    WindowCreated(Window),
    WGPUInitialized(WGPUState),
}

#[derive(Properties, PartialEq)]
pub struct WGPUCanvasProps {
    pub shader: AttrValue,

    #[prop_or_default]
    pub cursor_enabled: bool,
}

impl Component for WGPUCanvas {
    type Message = WGPUCanvasMsg;
    type Properties = WGPUCanvasProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WGPUCanvasMsg::WindowCreated(window) => {
                let cb = ctx.link().callback(WGPUCanvasMsg::WGPUInitialized);
                let shader = ctx.props().shader.clone();
                yew::platform::spawn_local(async move {
                    cb.emit(WGPUState::new(window, &shader).await)
                })
            }
            WGPUCanvasMsg::WGPUInitialized(mut state) => {
                state.render().expect("failed to render.");
                // self.wgpu_state = Some(state);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let mut classes = classes!();
        if !props.cursor_enabled {
            classes.push("pointer-events-none");
        }

        html! {
            <canvas
                ref={self.canvas.clone()}
                class={classes}
                width="600"
                height="600"
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let (cb, _) = ctx.link()
                .context::<AppCallbackContext>(Callback::noop())
                .unwrap();

            cb.emit((
                self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                ctx.link().callback(WGPUCanvasMsg::WindowCreated)
            ));
        }
    }
}