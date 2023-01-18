mod components;
mod models;

use crate::components::task::tasklist::TaskList;
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "flex flex-col min-h-screen",

        div {
          class: "text-center text-2xl",
          "Todo app"
        }

        TaskList {}
      }
    })
}
