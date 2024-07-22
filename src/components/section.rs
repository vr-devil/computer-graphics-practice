use yew::{AttrValue, Component, Context, Html, html, Properties};

pub struct Section;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Html,

    pub title: AttrValue,
    pub subtitle: AttrValue,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <section class="w-full flex flex-row">
                <div class="grow">
                    <h2>{props.title.as_str()}</h2>
                    <p class="my-4">{props.subtitle.as_str()}</p>
                </div>
                <div>
                    { props.children.clone() }
                </div>
            </section>
        }
    }
}