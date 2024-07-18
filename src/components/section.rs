use winit::window::Window;
use yew::{AttrValue, Callback, classes, Component, Context, Html, html, NodeRef, Properties};
use web_sys::HtmlCanvasElement;
use crate::app::AppCallbackContext;
use crate::graphics::WGPUState;

pub struct Section {
    canvas: NodeRef,
}

pub enum SectionMsg {
    WindowCreated(Window),
    WGPUInitialized(WGPUState),
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: AttrValue,
    pub subtitle: AttrValue,
    pub shader: AttrValue,

    #[prop_or_default]
    pub cursor_enabled: bool,
}

impl Component for Section {
    type Message = SectionMsg;
    type Properties = SectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SectionMsg::WindowCreated(window) => {
                let cb = ctx.link().callback(SectionMsg::WGPUInitialized);
                let shader = ctx.props().shader.clone();
                yew::platform::spawn_local(async move {
                    cb.emit(WGPUState::new(window, &shader).await)
                })
            }
            SectionMsg::WGPUInitialized(mut state) => {
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
            <section class="w-full flex flex-row">
                <div class="grow">
                    <h2>{props.title.as_str()}</h2>
                    <p class="my-4">{props.subtitle.as_str()}</p>
                </div>
                <div>
                    <canvas
                        ref={self.canvas.clone()}
                        class={classes}
                        width="600"
                        height="600"
                    />
                </div>
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let (cb, _) = ctx.link()
                .context::<AppCallbackContext>(Callback::noop())
                .unwrap();

            cb.emit((
                self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                ctx.link().callback(SectionMsg::WindowCreated)
            ));
        }
    }
}