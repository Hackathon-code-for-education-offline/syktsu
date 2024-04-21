use std::collections::HashMap;

use crate::{
    app::config::api::API_PATH,
    components::header::Header,
    pages::{
        home::Home,
        login::Login,
        profile::{MyProfile, Profile},
        university::{Universities, University, UniversityMap, UniversitySettings},
    },
    shared::{error::UiError, response::Response},
};
use gloo_utils::window;
use reqwest::header::CONTENT_TYPE;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, use_local_storage, UseAsyncHandle, UseAsyncOptions};
use yew_router::{navigator, prelude::*};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/profile")]
    ProfileRoot,
    #[at("/profile/:id")]
    Profile { id: String },
    #[at("/university")]
    UniversityRoot,
    #[at("/university/:id")]
    University { id: usize },
    #[at("/university/:id/settings")]
    UniversitySettings { id: usize },
    #[at("/university/:id/map")]
    UniversityMap { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub(crate) fn app() -> Html {
    // let session = use_local_storage::<String>(LOCAL_SESSION.to_string());

    // let is_auth: UseAsyncHandle<Response, UiError> = use_async_with_options(
    //     async move {
    //         let client = reqwest::Client::new();

    //         let mut payload = HashMap::new();
    //         payload.insert("session", "rust");

    //         let location = window().location();

    //         let protocol = location.protocol().map_err(|e| UiError::from(e))?;
    //         let hostname = location.hostname().map_err(|e| UiError::from(e))?;

    //         let api_url = format!("{protocol}://{hostname}:4444");

    //         // let api_url = window()
    //         //     .location()
    //         //     .origin()
    //         //     .map(|o| format!("{o}/{API_PATH}"))
    //         //     .map_err(|e| UiError::from(e))?;

    //         let res_body = client
    //             .post(api_url)
    //             .header(CONTENT_TYPE, "application/json")
    //             .json(&payload)
    //             .send()
    //             .await
    //             .map_err(|e| UiError::from(e))?
    //             .json::<Response>()
    //             .await
    //             .map_err(|e| UiError::from(e))?;

    //         Ok(res_body)
    //     },
    //     UseAsyncOptions::enable_auto(),
    // );

    // // fallback
    // if is_auth.loading {
    //     return html!();
    // };

    // if let Some(data) = is_auth.data.clone() {

    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    };
    // }

    // if let Some(error) = is_auth.error.clone() {
    //     tracing::error!("{error}");
    // }
    // use crate::utils::WINDOW;

    // if let Ok(tauri_internals) = WINDOW.tauri_internals().map_err(|e| tracing::warn!("{:?}", e)) {
    //     // spawn_local(async move {
    //         // let _ = tauri_internals.invoke("greet", serde_wasm_bindgen::to_value(&json!({"name": "sdad"})).unwrap_or_default()).await;
    //     // });
    // };

    // html!()
}

fn switch(routes: Route) -> Html {
    html! {
        <>
            if Route::Login != routes {
                <AppHeader />
            }

            <main class={classes!(if Route::Login == routes {"login"} else {""})}>
                // if Route::Login != routes {
                //     <BreadCrumbs values={} />
                // }

                {
                    match routes {
                        Route::Home => html! { <Home /> },
                        Route::Login => html! { <Login /> },
                        Route::UniversityRoot => html! { <Universities /> },
                        Route::UniversitySettings { id } => html! { <UniversitySettings {id} /> },
                        Route::University { id } => html! { <University {id} /> },
                        Route::UniversityMap { id } => html! { <UniversityMap {id} /> },
                        Route::ProfileRoot => html! { <MyProfile /> },
                        Route::Profile { id } => html! { <Profile {id} /> },

                        Route::NotFound  => html! { <h1>{ "404" }</h1> },
                    }
                }
            </main>
        </>
    }
}

#[function_component(AppHeader)]
fn app_header() -> Html {
    let navigator = use_navigator().unwrap();
    let to_login_page = Callback::from(move |_e: MouseEvent| navigator.push(&Route::Login));
    let is_auth = true;

    html! {
        <Header {is_auth} {to_login_page} />
    }
}
