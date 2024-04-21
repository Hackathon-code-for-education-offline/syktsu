use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub is_accent: bool,

    #[prop_or_default]
    pub with_icon: Option<AttrValue>,

    #[prop_or_default]
    pub value: Option<AttrValue>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        is_accent,
        with_icon,
        value,
    } = props;

    html! {
        <div class={classes!("button", "text-size-m16", if *is_accent {"accent"} else {"base"})}>
            if let Some(icon_name) = with_icon {
                <span class={classes!(icon_name)}></span>
            }

            { value.clone() }
        </div>
    }
}
