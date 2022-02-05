use rustknock_frontend::welcome::Welcome;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div>
            <Welcome />
        </div>
    )
}
