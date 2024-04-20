use serde_json::json;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::{
    shared::error::UiError,
    utils::{viewer, PannellumOptions, WINDOW},
};

#[derive(PartialEq, Properties, Clone)]
pub struct PanoramaProps {
    pub panorama_id: AttrValue,
    pub _type: AttrValue,
    pub panorama: Option<AttrValue>,
}

#[function_component(Panorama)]
pub fn panorama(props: &PanoramaProps) -> Html {
    html! {
        <div id={props.panorama_id.clone()}>
            <PanoramaInit ..props.clone() />
        </div>
    }
}

#[function_component(PanoramaInit)]
pub fn panorama_init(props: &PanoramaProps) -> Html {
    let elem_id = props.panorama_id.clone();

    use_effect_once(move || {
        if let Ok(pan) = WINDOW.pannellum() {
            // let options = serde_wasm_bindgen::to_value(&PannellumOptions {
            //     _type: "equirectangular".to_string().into(),
            //     panorama: "https://pannellum.org/images/alma.jpg".to_string().into(),
            // })
            // .unwrap_or_default();
            // let options = serde_wasm_bindgen::to_value(
            //     &format!(""json!({
            //         "type": "equirectangular",
            //         "panorama": "https://pannellum.org/images/alma.jpg"
            //     })
            //     .to_string()),
            // )
            // .unwrap_or_default();

            // tracing::warn!("{:?}", options);

            let _ = viewer();
            // let _ = pan
            //     .viewer(elem_id.to_string(), options)
            //     .map_err(|e| tracing::error!("{}", UiError::from(e)));
        };

        || ()
    });

    html! {}
}
