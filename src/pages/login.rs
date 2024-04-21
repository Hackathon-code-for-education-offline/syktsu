use std::borrow::Cow;

use crate::components::{button::Button, input::Input};
use app_interface::LoginRequest;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Login)]
pub fn login() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let is_auth: UseAsyncHandle<Response<Vec<University>>, UiError> = use_async_with_options(
        async move {
            let client = reqwest::Client::new();

            let mut payload = HashMap::new();
            payload.insert("session", "rust");

            let location = window().location();

            let protocol = location.protocol().map_err(|e| UiError::from(e))?;
            let hostname = location.hostname().map_err(|e| UiError::from(e))?;

            let api_url = format!("{protocol}//{hostname}:{API_PORT}/{API_UNIVERSITY_PATH}");

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

    let Some(data) = university.data else {
        if let Some(e) = university.error {
            tracing::error!("{e}");
        }

        return html!();
    };

    let go_back = use_callback((), move |_e: MouseEvent, _| navigator.back());

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

        use_callback((), move |e: SubmitEvent, _| {
            e.prevent_default();

            // unsafe for now
            let username = username_ref
                .cast::<HtmlInputElement>()
                .map(|e| Cow::Owned(e.value()))
                .unwrap_or_default();
            let password = password_ref
                .cast::<HtmlInputElement>()
                .map(|e| Cow::Owned(e.value()))
                .unwrap_or_default();

            let req = LoginRequest { username, password };
            tracing::warn!("{:?}", req)
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
