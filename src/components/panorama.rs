use yew::prelude::*;
use yew_hooks::use_event_with_window;

use crate::{
    shared::error::UiError,
    utils::{PannellumOptions, WINDOW},
};

#[derive(PartialEq, Properties, Clone)]
pub struct PanoramaProps {
    pub panorama_id: AttrValue,
    pub _type: AttrValue,
    pub panorama: Option<AttrValue>,
}

#[function_component(Panorama)]
pub fn panorama(props: &PanoramaProps) -> Html {
    let elem_id = props.panorama_id.clone();

    use_event_with_window("load", move |_: Event| {
        let options = serde_wasm_bindgen::to_value(&PannellumOptions {
            _type: "equirectangular".into(),
            panorama: "https://pannellum.org/images/alma.jpg".into(),
        })
        .unwrap_or_default();

        WINDOW
            .pannellum()
            .and_then(|pan| pan.viewer(elem_id.as_str(), options))
            .unwrap_or_else(|e| tracing::error!("{}", UiError::from(e)));
    });

    html! {
        <div id={props.panorama_id.clone()}></div>
    }
}
