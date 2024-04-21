use std::collections::HashMap;

use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Default, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PannellumOptions<'a> {
    pub default: PannellumDefault<'a>,
    pub scenes: HashMap<&'a str, PannellumScene<'a>>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PannellumDefault<'a> {
    pub first_scene: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_fade_duration: Option<usize>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PannellumScene<'a> {
    #[serde(rename = "type")]
    pub _type: &'a str,
    pub panorama: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_spots: Option<Vec<HotSpot<'a>>>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotSpot<'a> {
    pub pitch: i64,
    pub yaw: i64,
    #[serde(rename = "type")]
    pub _type: &'a str,
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<&'a str>,
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

    #[wasm_bindgen(method, structural, getter = pannellum, catch)]
    pub fn pannellum(this: &WasmWindow) -> Result<Pannellum, JsValue>;

    #[wasm_bindgen(method, structural, js_name = viewer, catch)]
    pub fn viewer(this: &Pannellum, id: &str, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, catch)]
    pub async fn invoke(
        this: &TauriInternals,
        cmd: &str,
        args: JsValue,
    ) -> Result<JsValue, JsValue>;
}
