use std::collections::HashMap;

use app_interface::{Response, University};
use gloo_utils::window;
use reqwest::header::CONTENT_TYPE;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

use crate::shared::{config::API_PORT, error::UiError};

#[function_component(Home)]
pub fn home() -> Html {
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

            let api_url = format!("{protocol}://{hostname}:{API_PORT}");

            let res_body = client
                .post(api_url)
                .header(CONTENT_TYPE, "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| UiError::from(e))?
                .json::<Response<Vec<University>>>()
                .await
                .map_err(|e| UiError::from(e))?;

            Ok(res_body)
        },
        UseAsyncOptions::enable_auto(),
    );

    // fallback
    // if is_auth.loading {
    //     return html!();
    // };

    // if let Some(data) = is_auth.data.clone() {}

    // if let Some(error) = is_auth.error.clone() {
    //     tracing::error!("{error}");
    // }
    use crate::utils::WINDOW;

    if let Ok(tauri_internals) = WINDOW
        .tauri_internals()
        .map_err(|e| tracing::warn!("{:?}", e))
    {
        // spawn_local(async move {
        // let _ = tauri_internals.invoke("greet", serde_wasm_bindgen::to_value(&json!({"name": "sdad"})).unwrap_or_default()).await;
        // });
    };

    html! {
        <>
            <section class={classes!("home__title")}>
                <h1 class={classes!("home__title__prim")}>{ "Знакомьтесь с вузами ближе" }</h1>
                <p class={classes!("home__title__sub", "text-size-r24")}>{ "Виртуальные экскурсии, общение со студентами и преподавателями, честные отзывы" }</p>
            </section>

            <section class={"slider"}>
                // {
                //     universities.into_iter()
                //         .map(|u| )
                //         .collect::<Html>()
                // }
            </section>
        </>
    }
}
