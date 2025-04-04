use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>{ "Bienvenue sur SentinelRisk" }</h1>
            <p>{ "L'interface Yew est opérationnelle 🚀" }</p>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
