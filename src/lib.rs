use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("No global `window` exists");
    let document: Document = window.document().expect("Should have a document on window");

    let button = document.create_element("button")?.dyn_into::<web_sys::HtmlElement>()?;
    button.set_inner_text("Click Me");

    button.style().set_property("margin", "10px")?;
    button.style().set_property("padding", "10px 20px")?;
    button.style().set_property("font-size", "16px")?;

    let message = document.create_element("div")?.dyn_into::<web_sys::HtmlElement>()?;
    message.set_inner_text("Hello, World!");

    message.style().set_property("color", "blue")?;
    message.style().set_property("font-weight", "bold")?;
    message.style().set_property("margin-top", "10px")?;

    let body = document.body().expect("Document should have a body");
    body.append_child(&button)?;
    body.append_child(&message)?;

    let message_clone = message.clone();

    let click_closure = Closure::wrap(Box::new(move || {
        message_clone.set_inner_text("Button Clicked!");
    }) as Box<dyn FnMut()>);

    button.set_onclick(Some(click_closure.as_ref().unchecked_ref()));

    click_closure.forget();

    let message_hover = message.clone();

    let hover_closure = Closure::wrap(Box::new(move || {
        message_hover.set_inner_text("Mouse Over!");
    }) as Box<dyn FnMut()>);

    message.set_onmouseover(Some(hover_closure.as_ref().unchecked_ref()));

    hover_closure.forget();

    Ok(())
}