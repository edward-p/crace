#![allow(non_snake_case)]
use crace::app::App;

fn main() {
    // Init logger
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    dioxus_web::launch(App);
}

