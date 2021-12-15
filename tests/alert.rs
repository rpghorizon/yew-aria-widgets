//! Based on <https://github.com/w3c/aria-practices/blob/apg-1.1r4/test/tests/alert.js>
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::{function_component, html, use_state, Callback};
use yew_aria_widgets::alert::*;

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(TestHost)]
fn test_host() -> Html {
    let alert_template = use_state(|| None);
    let onclick = {
        let alert_template = alert_template.clone();
        Callback::from(move |_| {
            alert_template.set(
                Some(html! {
                  <p><span lang="da">{"Hej"}</span>{", hello, "}<span lang="it">{"ciao"}</span>{", "}<span lang="ja">{"こんにちは"}</span>{", "}<span lang="ko">{"안녕"}</span></p>
                })
            );
        })
    };
    html! {
        <>
            <button id="alert-trigger" {onclick}>{"Trigger Alert"}</button>
            <AriaAlert id="example">{alert_template.as_ref().cloned().unwrap_or_default()}</AriaAlert>
        </>
    }
}

#[wasm_bindgen_test]
fn alert() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let block = document.create_element("div").unwrap();
    block.set_id("test-host");
    body.append_child(&block).unwrap();
    yew::start_app_in_element::<TestHost>(block.clone());
    // Begin test
    let alerts = document.get_element_by_id("example").unwrap();
    assert_eq!(alerts.children().length(), 0);
    let button = document.get_element_by_id("alert-trigger").unwrap();
    let button = button.dyn_into::<HtmlElement>().unwrap();
    button.click();
    let alerts = document.get_element_by_id("example").unwrap();
    assert_ne!(alerts.children().length(), 0);
    // End test
}
