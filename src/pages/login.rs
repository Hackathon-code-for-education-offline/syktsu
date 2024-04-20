use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class={classes!("login__form")}>
            <span class={classes!("login__form__logo")}></span>

            <div class={classes!("login__form__title")}>
            </div>

        </div>
    }
}
