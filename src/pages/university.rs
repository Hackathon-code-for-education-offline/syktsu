use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::{
    components::panorama::Panorama,
    shared::error::UiError,
    utils::{PannellumOptions, WINDOW},
};

#[derive(PartialEq, Properties)]
pub struct UniversityProps {
    #[prop_or_default]
    pub id: Option<usize>,
}

#[function_component(University)]
pub fn university(props: &UniversityProps) -> Html {
    let UniversityProps { id } = props;

    html! {
        <div>
            { id }
        </div>
    }
}

#[function_component(Universities)]
pub fn universities() -> Html {
    html! {
        <div>
            { "List here" }
        </div>
    }
}

#[function_component(UniversityMap)]
pub fn university_map(props: &UniversityProps) -> Html {
    let UniversityProps { id } = props;
    // let university_id = id;
    // let elem_ref = use_node_ref();

    // use_effect_with(elem_ref.clone(), move |elem_ref| {
    //     if let Some(pan_id) = elem_ref.cast::<HtmlElement>().map(|e| e.id()) {
    //         if let Ok(pan) = WINDOW.pannellum() {
    //             let options = serde_wasm_bindgen::to_value(&PannellumOptions {
    //                 _type: "equirectangular".to_string().into(),
    //                 panorama: "https://pannellum.org/images/alma.jpg".to_string().into(),
    //             })
    //             .unwrap_or_default();
    //             // let options = serde_wasm_bindgen::to_value(&json!({
    //             //     "type": "equirectangular",
    //             //     "panorama": "https://pannellum.org/images/alma.jpg"
    //             // }))
    //             // .unwrap_or_default();

    //             tracing::warn!("{:?}", options);

    //             let _ = pan
    //                 .viewer(pan_id.as_str(), options)
    //                 .map_err(|e| tracing::error!("{}", UiError::from(e)));
    //         };
    //     };

    //     tracing::error!("unable to get panorama element");

    //     || ()
    // });

    // pannellum.viewer('panorama', {
    //     "type": "equirectangular",
    //     "panorama": "https://pannellum.org/images/alma.jpg"
    // });

    html! {
        <Panorama _type={"equirectangular"} panorama={"https://pannellum.org/images/alma.jpg"} panorama_id={"panorama"} />
        // <div ref={elem_ref} id={"panorama"}></div>
    }
}
