use app_interface::PannellumOptions;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::use_event_with_window;

use crate::{shared::error::UiError, utils::WINDOW};

#[derive(PartialEq, Properties, Clone, Serialize)]
pub struct PanoramaProps {
    pub options: PannellumOptions,
}

#[function_component(Panorama)]
pub fn panorama(props: &PanoramaProps) -> Html {
    let options = props.options.clone();
    let panorama_id = "panorama";

    use_event_with_window("load", move |_: Event| {
        WINDOW
            .pannellum()
            .and_then(|pan| {
                pan.viewer(
                    panorama_id,
                    JsValue::from_serde(&options).unwrap_or_default(),
                )
            })
            .unwrap_or_else(|e| tracing::error!("{}", UiError::from(e)));
    });

    html! {
        <div id={panorama_id}></div>
    }
}
