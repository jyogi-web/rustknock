use rustknock_frontend::welcome::Welcome;
use yew::prelude::*;

use {
    futures::prelude::*,
    log::*,
    pharos::{Observable, ObserveConfig},
    // web_sys               :: { console::log_1 as dbg } ,
    wasm_bindgen::prelude::*,
    wasm_bindgen_test::*,
    ws_stream_wasm::*,
};

use wasm_bindgen_futures::spawn_local;

const URL: &str = "ws://127.0.0.1:3000/ws/";

async fn close_events() {
    // let _ = console_log::init_with_level(Level::Trace);

    // info!はwasm-loggerのマクロだよ！
    info!("starting test: close_events");

    let (mut ws, _wsio) = WsMeta::connect(URL, None)
        .await
        .expect_throw("Could not create websocket");

    let mut evts = ws.observe(ObserveConfig::default()).await.expect("observe");

    info!("Update: {:?}", ws.ready_state());
    ws.close().await.expect_throw("close ws");
    info!("Update: {:?}", ws.ready_state());

    assert!(evts.next().await.unwrap_throw().is_closing());
    assert!(evts.next().await.unwrap_throw().is_closed());
}

#[function_component(App)]
pub fn app() -> Html {
    let onclick = { Callback::from(move |_| spawn_local(async { close_events().await })) };

    html!(
        <div>
            <Welcome />
            <button {onclick}>{"enter"}</button>
        </div>
    )
}
