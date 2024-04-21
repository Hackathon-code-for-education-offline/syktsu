use std::{collections::HashMap, vec};

use app_interface::{HotSpot, PannellumDefault, PannellumOptions, PannellumScene};
use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::{components::panorama::Panorama, shared::error::UiError, utils::WINDOW};

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

#[function_component(UniversitySettings)]
pub fn university_settings(props: &UniversityProps) -> Html {
    let UniversityProps { id } = props;

    html! {
        <>
            <h3>{"Настройки"}</h3>
        </>
    }
}

#[function_component(UniversityMap)]
pub fn university_map(props: &UniversityProps) -> Html {
    let UniversityProps { id } = props;

    let mut scenes = HashMap::new();
    scenes.insert(
        "campus".to_string(),
        PannellumScene {
            _type: "equirectangular".to_string(),
            panorama: "https://pannellum.org/images/alma.jpg".to_string(),
            hot_spots: vec![
                HotSpot {
                    pitch: -1.02,
                    yaw: 170.0,
                    _type: "info".to_string(),
                    text: "This is Jordan Pond, located in Acadia National Park.".to_string(),
                    ..Default::default()
                },
                HotSpot {
                    pitch: -24.0,
                    yaw: 170.0,
                    _type: "scene".to_string(),
                    text: "This is Jordan Pond, located in Acadia National Park.".to_string(),
                    scene_id: "kitchen".to_string().into(),
                    ..Default::default()
                },
            ]
            .into(),
            ..Default::default()
        },
    );
    scenes.insert(
        "kitchen".to_string(),
        PannellumScene {
            _type: "equirectangular".to_string(),
            panorama: "https://pannellum.org/images/alma.jpg".to_string(),
            hot_spots: vec![HotSpot {
                pitch: -12.0,
                yaw: 100.0,
                _type: "info".to_string(),
                text: "This is Jordan Pond, located in Acadia National Park.".to_string(),
                ..Default::default()
            }]
            .into(),
            ..Default::default()
        },
    );

    let options = PannellumOptions {
        default: PannellumDefault {
            first_scene: "campus".to_string(),
            ..Default::default()
        },
        scenes,
    };

    html! {
        <Panorama {options} />
    }
}
