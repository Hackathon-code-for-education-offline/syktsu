use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub with_icon: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        with_icon,
        placeholder,
    } = props;

    html! {
        <div class={classes!("input")}>
            <input class={classes!("input__field")} {placeholder} />

            if let Some(icon_name) = with_icon {
                <span class={classes!(icon_name)}></span>
            }
        </div>
    }
}
