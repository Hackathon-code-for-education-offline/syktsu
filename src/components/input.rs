use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub with_icon: Option<AttrValue>,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub input_ref: NodeRef,

    #[prop_or_default]
    pub field_classes: Classes,

    #[prop_or_default]
    pub wrapper_classes: Classes,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        with_icon,
        placeholder,
        id,
        input_ref,
        wrapper_classes,
        field_classes,
    } = props;

    html! {
        <div class={classes!(wrapper_classes.clone(), "input")}>
            <input ref={input_ref.clone()} class={classes!(field_classes.clone(), "input__field")} {placeholder} {id} />

            if let Some(icon_name) = with_icon {
                <span class={classes!(icon_name)}></span>
            }
        </div>
    }
}
