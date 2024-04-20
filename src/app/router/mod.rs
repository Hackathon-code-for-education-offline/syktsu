use crate::pages::{
    home::Home,
    profile::{MyProfile, Profile},
    university::{Universities, University},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    ProfileRoot,
    #[at("/profile/:id")]
    Profile { id: String },
    #[at("/university")]
    UniversityRoot,
    #[at("/university/:id")]
    University { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub(crate) fn app() -> Html {
    // use crate::utils::WINDOW;

    tracing_wasm::set_as_global_default();

    // if let Ok(tauri_internals) = WINDOW.tauri_internals().map_err(|e| tracing::warn!("{:?}", e)) {
    //     // spawn_local(async move {
    //         // let _ = tauri_internals.invoke("greet", serde_wasm_bindgen::to_value(&json!({"name": "sdad"})).unwrap_or_default()).await;
    //     // });
    // };

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::UniversityRoot => html! { <Universities /> },
        Route::University { id } => html! { <University {id} /> },
        Route::ProfileRoot => html! { <MyProfile /> },
        Route::Profile { id } => html! { <Profile {id} /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
