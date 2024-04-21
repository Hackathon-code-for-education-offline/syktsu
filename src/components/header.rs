use crate::components::{button::Button, input::Input};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub is_auth: bool,
    pub to_login_page: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    // let HeaderProps { is_auth } = props;
    let shadow_toggle = use_node_ref();

    let toggle_menu = {
        let shadow_toggle = shadow_toggle.clone();

        use_callback((), move |_e: MouseEvent, _| {
            shadow_toggle.cast::<HtmlInputElement>().map_or((), |e| {
                if e.checked() {
                    e.set_checked(false)
                } else {
                    e.set_checked(true)
                }
            })
        })
    };

    html! {
        <header>
            <input type={"checkbox"} ref={shadow_toggle} class={classes!("shadow-toggle")} id={"header-toggle"} />
            <span class={classes!("header__logo", "icon-logo")}></span>

            <Button with_icon={"icon-menu"} value={"Все вузы"} onclick={toggle_menu} />
            <Input placeholder={"Поиск"} />

            <Button value={"Стать партнёром"} onclick={props.to_login_page.clone()} />
            <Button value={"Войти"} is_accent=true onclick={props.to_login_page.clone()} />
        </header>
    }
}
