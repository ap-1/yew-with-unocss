use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <p class="text-5xl">{ "Hello World!" }</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
