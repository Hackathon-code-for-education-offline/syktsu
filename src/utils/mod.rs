use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct PannellumOptions {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub panorama: Option<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type WasmWindow;

    #[wasm_bindgen(extends = Object)]
    pub type TauriInternals;

    #[wasm_bindgen(extends = Object)]
    pub type Pannellum;

    #[wasm_bindgen(js_name = window)]
    pub static WINDOW: WasmWindow;

    #[wasm_bindgen(method, structural, getter = __TAURI_INTERNALS__, catch)]
    pub fn tauri_internals(this: &WasmWindow) -> Result<TauriInternals, JsValue>;

    #[wasm_bindgen(method, structural, getter = panellum, catch)]
    pub fn pannellum(this: &WasmWindow) -> Result<Pannellum, JsValue>;

    #[wasm_bindgen(catch)]
    pub fn viewer() -> Result<(), JsValue>;
    // #[wasm_bindgen(method, structural, js_name = viewer, catch)]
    // pub fn viewer(this: &Pannellum, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, catch)]
    pub async fn invoke(
        this: &TauriInternals,
        cmd: &str,
        args: JsValue,
    ) -> Result<JsValue, JsValue>;
}
