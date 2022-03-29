use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class="title">{"RustyMinitel"}</h2>
            <div class="box">
                <div class="contain">
                    <p>{"Home"}</p>
                    <button>{"Information"}</button>
                    <button>{"Network"}</button>
                    <button>{"Process"}</button>
                </div>
            </div>

        </div>
    }
}

