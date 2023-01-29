use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::modules::requests;
use web_sys::HtmlInputElement;
use crate::BACKEND;

#[function_component]
pub fn App() -> Html {

    let text = use_state(|| String::new());
    let answer = use_state(|| String::new());

    let onclick = {
        let answer = answer.clone();
        let text = text.clone();
        move |_: MouseEvent| {
            let answer = answer.clone();
            let text = text.clone();
            wasm_bindgen_futures::spawn_local(async move {
            let tmp = requests::post_request::<String>(
                &format!("{BACKEND}/echo"),
                String::from(&*text))
                .await
                .expect("Error with the request!!");
                answer.set(tmp.to_string())
            })
        }
    };

    let on_text_input = Callback::from(move |event: Event| {
            text.set(event.target().unwrap().unchecked_into::<HtmlInputElement>().value());
        });

    html! {
        <div>
            <h1>{"Rust web app demo..."}</h1>
            <h3>{"Demonstration of a frontend-backend communication."}</h3>
            <h3>{"Add some text to send:"}<br/></h3>
            <input type={"text"} onchange={on_text_input} />
            <button {onclick}>{ "Send!" }</button>
            <p>{"Answer from the backend: "}{ &*answer }</p>
        </div>
    }
}
