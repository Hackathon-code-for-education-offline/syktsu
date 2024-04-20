use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub is_auth: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps { is_auth } = props;

    html! {
        <header>

        </header>
    }
}
