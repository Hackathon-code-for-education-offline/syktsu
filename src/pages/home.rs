use std::{collections::HashMap, ops::Div};

use app_interface::{Code, Response, University};
use gloo_utils::window;
use reqwest::header::CONTENT_TYPE;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

use crate::shared::{
    config::{API_PORT, API_UNIVERSITY_PATH},
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

                // {
                //     universities.into_iter()
                //         .map(|u| )
                //         .collect::<Html>()
                // }
            </section>
        </>
    }
}

#[function_component(HomeCards)]
fn home_cards() -> Html {
    // let res: Response<Vec<University>>;
    // let session = use_local_storage::<String>(LOCAL_SESSION.to_string());

    let university: UseAsyncHandle<Response<Vec<University>>, UiError> = use_async_with_options(
        async move {
            let client = reqwest::Client::new();

            let mut payload = HashMap::new();
            payload.insert("session", "rust");

            let location = window().location();

            let protocol = location.protocol().map_err(|e| UiError::from(e))?;
            let hostname = location.hostname().map_err(|e| UiError::from(e))?;

            let api_url = format!("{protocol}//{hostname}:{API_PORT}/{API_UNIVERSITY_PATH}");

            let res_body = client
                .get(api_url)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .map_err(|e| UiError::from(e))?
                .json::<Response<Vec<University>>>()
                .await
                .map_err(|e| UiError::from(e))?;

            // let res_body = client
            //     .post(api_url)
            //     .header(CONTENT_TYPE, "application/json")
            //     .json(&payload)
            //     .send()
            //     .await
            //     .map_err(|e| UiError::from(e))?
            //     .json::<Response<Vec<University>>>()
            //     .await
            //     .map_err(|e| UiError::from(e))?;

            Ok(res_body)
        },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(data) = university.data.clone() {
        if let Code::Success = data.code {
            return data
                .payload
                .into_iter()
                .map(|u| {
                    html! {
                       <div class={classes!("home__university-card")}>
                            <img src={u.link_profile} alt={u.title.clone()} />
                            <p class={classes!("text-size-r16")}>{ u.title }</p>
                            <span class={classes!("icon-star")}></span>
                            <p class={classes!("text-size-r16")}>{ u.score }</p>
                            <p class={classes!("text-size-r16", "disabled")}>{ u.voters }</p>
                       </div>
                    }
                })
                .collect::<Html>();
        } else {
            tracing::error!("unable to get universities")
        }
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
