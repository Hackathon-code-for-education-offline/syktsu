use std::{collections::HashMap, vec};

use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::{
    components::panorama::Panorama,
    shared::error::UiError,
    utils::{HotSpot, PannellumDefault, PannellumOptions, PannellumScene, WINDOW},
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

    let mut scenes = HashMap::new();
    scenes.insert(
        "campus",
        PannellumScene {
            _type: "equirectangular",
            panorama: "https://pannellum.org/images/alma.jpg",
            hot_spots: vec![
                HotSpot {
                    pitch: -12,
                    yaw: 170,
                    _type: "info",
                    text: "This is Jordan Pond, located in Acadia National Park.",
                    ..Default::default()
                },
                HotSpot {
                    pitch: -24,
                    yaw: 170,
                    _type: "scene",
                    text: "This is Jordan Pond, located in Acadia National Park.",
                    scene_id: "kitchen".into(),
                    ..Default::default()
                },
            ]
            .into(),
            ..Default::default()
        },
    );
    scenes.insert(
        "kitchen",
        PannellumScene {
            _type: "equirectangular",
            panorama: "https://pannellum.org/images/alma.jpg",
            hot_spots: vec![HotSpot {
                pitch: -12,
                yaw: 100,
                _type: "info",
                text: "This is Jordan Pond, located in Acadia National Park.",
                ..Default::default()
            }]
            .into(),
            ..Default::default()
        },
    );

    let options = PannellumOptions {
        default: PannellumDefault {
            first_scene: "campus",
            ..Default::default()
        },
        scenes,
    };

    html! {
        <Panorama {options} />
    }
}
