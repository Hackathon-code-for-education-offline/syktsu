use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub _type: Option<AttrValue>,

    #[prop_or_default]
    pub is_accent: bool,

    #[prop_or_default]
    pub with_icon: Option<AttrValue>,

    #[prop_or_default]
    pub with_border: bool,

    #[prop_or_default]
    pub value: Option<AttrValue>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        _type,
        is_accent,
        with_icon,
        value,
        onclick,
        with_border,
        classes,
    } = props;

    html! {
        <button type={_type.clone().unwrap_or("button".into())} class={classes!(classes.clone(), "button", "text-size-m16", if *is_accent {"accent"} else {"base"},  if *with_border {""} else {"no-border"})} {onclick}>
            if let Some(icon_name) = with_icon {
                <span class={classes!(icon_name)}></span>
            }

            { value.clone() }
        </button>
    }
}
