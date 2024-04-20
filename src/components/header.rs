use crate::components::{button::Button, input::Input};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub is_auth: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    // let HeaderProps { is_auth } = props;

    html! {
        <header>
            <span class={classes!("header__logo", "icon-logo")}></span>

            <Button with_icon={"icon-menu"} value={"Все вузы"} />
            <Input placeholder={"Поиск"} />

            <Button value={"Стать партнёром"} />
            <Button value={"Войти"} is_accent=true />
        </header>
    }
}
