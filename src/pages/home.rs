use std::collections::HashMap;

use app_interface::{Code, Response, University};
use gloo_utils::window;
use reqwest::header::CONTENT_TYPE;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

use crate::shared::{
    config::{API_IP, API_PORT, API_UNIVERSITY_PATH},
    error::UiError,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <section class={classes!("home__title")}>
                <h1 class={classes!("home__title__prim")}>{ "Знакомьтесь с вузами ближе" }</h1>
                <p class={classes!("home__title__sub", "text-size-r24")}>{ "Виртуальные экскурсии, общение со студентами и преподавателями, честные отзывы" }</p>
            </section>

            <section class={"slider"}>
                <HomeCards />
            </section>
        </>
    }
}

#[function_component(HomeCards)]
fn home_cards() -> Html {
    // let res: Response<Vec<University>>;
    // let session = use_local_storage::<String>(LOCAL_SESSION.to_string());

    let university: UseAsyncHandle<Vec<University>, UiError> = use_async_with_options(
        async move {
            let client = reqwest::Client::new();

            let mut payload = HashMap::new();
            payload.insert("session", "rust");

            let location = window().location();

            let protocol = location.protocol().map_err(|e| UiError::from(e))?;
            let hostname = location.hostname().map_err(|e| UiError::from(e))?;

            let api_url = format!("{protocol}//{API_IP}:{API_PORT}/{API_UNIVERSITY_PATH}");

            let res_body = client
                .get(api_url)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .map_err(|e| UiError::from(e))?
                .json::<Vec<University>>()
                .await
                .map_err(|e| UiError::from(e))?;

            Ok(res_body)
        },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(data) = university.data.clone() {
        return data
            .into_iter()
            .map(|u| {
                html! {
                    <div class={classes!("slider__card")}>
                        <img src={u.link_pic} alt={u.title.clone()} class={classes!("slider__card__image")} />
                        <p class={classes!("text-size-r16", "slider__card__title")}>{ u.title }</p>
                        <span class={classes!("icon-star","slider__card__star")}></span>
                        <p class={classes!("text-size-r16", "slider__card__score")}>{ u.score }</p>
                        <p class={classes!("text-size-r16", "disabled", "slider__card__voters")}>{ u.voters }</p>
                    </div>
                }
            })
            .collect::<Html>();
        // use crate::utils::WINDOW;

        // if let Ok(tauri_internals) = WINDOW
        //     .tauri_internals()
        //     .map_err(|e| tracing::warn!("{:?}", e))
        // {
        //     return html! {};
        // }

        // spawn_local(async move {
        // let _ = tauri_internals.invoke("greet", serde_wasm_bindgen::to_value(&json!({"name": "sdad"})).unwrap_or_default()).await;
        // });
    };

    if let Some(error) = university.error.clone() {
        tracing::error!("{error}");
    }

    // fallback
    html! {}
}
