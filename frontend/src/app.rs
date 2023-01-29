use yew::prelude::*;
use crate::modules::requests;
use crate::BACKEND;

#[function_component]
pub fn App() -> Html {

    let text = use_state(|| String::from("Hello"));
    let phrase = use_state(|| String::new());

    let onclick = {
        let phrase = phrase.clone();
        let text = text.clone();

        move |_event: MouseEvent| {
        let phrase = phrase.clone();
        let text = text.clone();
            wasm_bindgen_futures::spawn_local(async move {
            let tmp = requests::post_request::<String>(
                &format!("{BACKEND}/echo"),
                String::from(&*text))
                .await
                .expect("Error with the request!!");
            phrase.set(tmp.to_string())
            })
        }
    };

    html! {
        <div>
            <button {onclick}>{ "Send!" }</button>
            <p>{"Answer: "}{ &*phrase }</p>
        </div>
    }
}
