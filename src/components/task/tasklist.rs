use dioxus::prelude::*;

use crate::{
  components::task::{
    task::SingleTask,
    form::Form,
  }, models::task::Task};

#[allow(non_snake_case)]
pub fn TaskList(cx: Scope) -> Element {
    let content = use_future(&cx, (), |_| async {
        reqwest::get("http://127.0.0.1:3000/tasks")
            .await
            .unwrap()
            .json::<Vec<Task>>()
            .await
    });

    cx.render(match content.value() {
        Some(Ok(data)) => rsx!(
          Form {}
          
          data.iter().map(|task| rsx! {
            SingleTask { task: task }
          })
        ),
        Some(Err(e)) => rsx! { format!("An error loading the resource occured. {:?}", e)},
        None => rsx! { pre { "Loading..." } },
    })
}
