use dioxus::prelude::*;
use serde_json::json;

#[allow(non_snake_case)]
pub fn Form(cx: Scope) -> Element {
    let task_name = use_state(cx, || String::new());

    cx.render(rsx! {
    div {
      class: "flex flex-col justify-center gap-4 border bg-gray-800",

      div {
        label {
          r#for: "task_name",
          class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
          "New Task Name"
        }
        input {
          id: "task_name",
          r#type: "text",
          class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",placeholder: "ex. go shopping",
          required: true,
          value: "{ task_name }",
          oninput: move |e| task_name.set(e.value.clone()),
        }
      }

      button {
        r#type: "button",
        class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
        onclick: move |_| cx.spawn({
          let task_name = task_name.clone();
          async move {
            let post_body = json!({
              "task": *task_name
            });
            let client = reqwest::Client::new();
            let _res = client.post("http://127.0.0.1:3000/task")
              .json(&post_body)
              .send()
              .await;

            task_name.set("".to_string());
        }}),
        "Submit"
      }
    }
  })
}
