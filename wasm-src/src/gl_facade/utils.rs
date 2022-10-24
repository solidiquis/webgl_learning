use js_sys::JsString;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, HtmlCanvasElement, window, WebGlRenderingContext as GL};

pub fn init_gl_rendering_context(canvas_id: &JsString) -> Result<GL, JsValue> {
    let canvas = get_canvas(canvas_id)?;

    let gl = canvas.get_context("webgl")?
        .ok_or(JsValue::from_str("Failed to get webgl context."))?
        .dyn_into::<GL>()?;

    Ok(gl)
}

fn get_canvas(canvas_id: &JsString) -> Result<HtmlCanvasElement, JsValue> {
    let id = canvas_id
        .as_string()
        .ok_or(JsValue::from_str("Invalid argument or invalid UTF-8."))?;

    let doc = document()?;

    let el = doc
        .get_element_by_id(&id)
        .ok_or(JsValue::from_str("Failed to query canvas element."))?;

    el.dyn_into::<HtmlCanvasElement>().map_err(|_e| {
        let err = format!("Unable to cast {canvas_id} to HtmlCanvasElement.");
        JsValue::from_str(&err)
    })
}

fn document() -> Result<Document, JsValue> {
    let win = window().ok_or(JsValue::from_str("Failed to get window object."))?;

    win.document().ok_or(JsValue::from_str("Failed to get document object."))
}
