use rustknock_frontend::hello::Hello;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div>
            <Hello />
        </div>
    )
}
