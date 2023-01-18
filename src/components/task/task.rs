use dioxus::prelude::*;

use crate::models::task::Task;

#[derive(Props, PartialEq)]
pub struct TaskProps<'a> {
    task: &'a Task,
}

#[allow(non_snake_case)]
pub fn SingleTask<'a>(cx: Scope<'a, TaskProps<'a>>) -> Element {
    cx.render(rsx! {
      div {
        class: "flex justify-start p-2 ",

        p {
          class: "text-lg font-bold",
          "{ cx.props.task.id }"
        }
        p {
          class: "text-lg font-bold",
          "{ cx.props.task.task }"
        }
      }
    })
}
