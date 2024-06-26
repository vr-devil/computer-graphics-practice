use yew::prelude::*;
use crate::section::prepare_environment::component::App as PrepareEnvironmentApp;

#[function_component]
pub fn App() -> Html {
    return html!(
        <>
            <h1>{"你好, 计算机图形"}</h1>
            <p>{"《"}<a href="https://gabrielgambetta.com/computer-graphics-from-scratch/" target="_blank">{"Computer Graphics from Scratch"}</a>{"》实践。"}</p>
            <section>
                <PrepareEnvironmentApp />
            </section>
        </>
    );
}