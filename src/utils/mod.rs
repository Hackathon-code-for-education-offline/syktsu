use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type WasmWindow;

    #[wasm_bindgen(extends = Object)]
    pub type TauriInternals;

    #[wasm_bindgen(js_name = window)]
    pub static WINDOW: WasmWindow;

    #[wasm_bindgen(method, structural, getter = __TAURI_INTERNALS__, catch)]
    pub fn tauri_internals(this: &WasmWindow) -> Result<TauriInternals, JsValue>;

    #[wasm_bindgen(method, structural, catch)]
    pub async fn invoke(this: &TauriInternals, cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

