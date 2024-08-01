mod utils;

use wasm_bindgen::prelude::*;

use image::EncodableLayout;

#[wasm_bindgen]
pub async fn extract_color(url: &str) -> Result<JsValue, JsValue> {

    let bytes = reqwest::get(url)
        .await
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?
        .bytes()
        .await
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?
        .to_vec();
    let image = image::load_from_memory(&bytes).unwrap();



    // if you are not sure
    let colors = dominant_color::get_colors(image.to_rgb8().as_bytes(), false);

    let js_array = js_sys::Array::new();
    for color in colors {
        js_array.push(&JsValue::from(color));
    }
    Ok(JsValue::from(js_array.slice(0, 3)))
}
