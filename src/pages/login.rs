use std::borrow::Cow;

use crate::shared::config::{API_IP, API_LOGIN_PATH, API_PORT, API_UNIVERSITY_PATH};
use crate::utils::WINDOW;
use crate::{
    components::{button::Button, input::Input},
    pages::university::University,
    shared::{error::UiError, response::Response},
};
use app_interface::LoginRequest;
use gloo_utils::format::JsValueSerdeExt;
use gloo_utils::window;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Request};
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, RequestMode};
use yew::prelude::*;
use yew_hooks::{use_local_storage, UseLocalStorageHandle};
use yew_router::hooks::use_navigator;

#[derive(Deserialize)]
pub struct Token {
    value: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let local_storage: UseLocalStorageHandle<String> = use_local_storage("token".to_string());

    let go_back = use_callback((), move |_e: MouseEvent, _| navigator.back());

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

        use_callback((), move |e: SubmitEvent, _| {
            e.prevent_default();
            let local_storage = local_storage.clone();

            // unsafe for now
            let username = username_ref
                .cast::<HtmlInputElement>()
                .map(|e| e.value())
                .unwrap_or_default();
            let password = password_ref
                .cast::<HtmlInputElement>()
                .map(|e| e.value())
                .unwrap_or_default();

            let body = LoginRequest { username, password };

            spawn_local(async move {
                let location = window().location();
                let client = Client::new();

                let protocol = location
                    .protocol()
                    .map_err(|e| tracing::error!("{}", UiError::from(e)))
                    .unwrap_or_default();
                // let hostname = location
                //     .hostname()
                //     .map_err(|e| tracing::error!("{}", UiError::from(e)))
                //     .unwrap_or_default();

                let api_url = format!("{protocol}//{API_IP}:{API_PORT}/{API_LOGIN_PATH}");

                let res = WINDOW
                    .fetch(
                        api_url,
                        JsValue::from_serde(&json!({
                            "body": json!(body).to_string(),
                            "method": "POST",
                            "mode": "no-cors"
                        }))
                        .unwrap_or_default(),
                    )
                    .await;
                // tracing::warn!("{:?}");

                // .post(api_url)
                // .fetch_mode_no_cors()
                // .json(&req)
                // .build()
                // .unwrap()
                // .ge
                // .await
                // .unwrap()
                // .text()
                // .await;
                // tracing::error!("{res:?}");
                // let body = reqwest::get("http://192.168.17.129:8000/university/1")
                //     .await
                //     .unwrap()
                //     .text()
                //     .await
                //     .unwrap();

                // println!("body = {body:?}");
                // let Ok(req) = client
                //     .post(api_url)
                //     .header(CONTENT_TYPE, "application/json")
                //     .fetch_mode_no_cors()
                //     .json(&req)
                //     .build()
                //     .map_err(|e| tracing::error!("{}", UiError::from(e)))
                // else {
                //     return tracing::error!("unable to create request");
                // };
                // let Ok(req) = client
                //     .post(api_url)
                //     .header(CONTENT_TYPE, "application/json")
                //     // .mode(RequestMode::NoCors)
                //     .json(&req)
                //     .map_err(|e| tracing::error!("{}", e.to_string()))
                // else {
                //     return tracing::error!("unable to create request");
                // };

                // let req = client
                //     .post(api_url)
                //     .header(CONTENT_TYPE, "application/json")
                //     .json(&req);

                // let Ok(req) = client
                //     .get("http://192.168.17.129:8000/university/1")
                //     .fetch_mode_no_cors()
                //     .header(CONTENT_TYPE, "application/json")
                //     .build()
                // else {
                //     return tracing::error!("unable to create request");
                // };

                // if let Ok(res) = client
                //     .execute(req)
                //     .await
                //     .map_err(|e| tracing::error!("{}", UiError::from(e)))
                // {
                //     let _ = res
                //         .text()
                //         .await
                //         .map(|token| local_storage.clone().set(token))
                //         .map_err(|e| tracing::error!("{}", UiError::from(e)));
                // };

                // tracing::warn!("{:?}", req.text().await)
                // if let Ok(res) = req.send().await {
                //     let _ = res
                //         .body()
                //         // .await
                //         .map(|token| String::from(token))
                //         .map_err(|e| tracing::error!("{}", e.to_string()));
                // };
                // if let Ok(res) = client
                //     .execute(req)
                //     .await
                //     .map_err(|e| tracing::error!("{}", UiError::from(e)))
                // {
                //     let _ = res
                //         .text()
                //         .await
                //         .map(|token| local_storage.clone().set(token))
                //         .map_err(|e| tracing::error!("{}", UiError::from(e)));
                // };
            })
        })
    };

    html! {
        <div class={classes!("login__wrapper")}>
            <Button with_border=true classes={classes!("login__back-button")} with_icon={"icon-arrowBack"} onclick={go_back} />

            <p class={classes!("login__wrapper__title", "text-size-m24")}>{"Вход"}</p>

            <form class={classes!("login__form")} {onsubmit}>
                <Input input_ref={username_ref} placeholder={"Логин"} id={"username"} wrapper_classes={classes!("login__form__input")} field_classes={classes!("text-size-r18")}  />
                <Input input_ref={password_ref} placeholder={"Пароль"} id={"password"} wrapper_classes={classes!("login__form__input")} field_classes={classes!("text-size-r18")}  />

                <Button is_accent=true value="Войти" classes={classes!("login__form__button")} _type={"submit"} />
            </form>
        </div>
    }
}
