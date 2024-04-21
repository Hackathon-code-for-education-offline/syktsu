use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BreadCrumbsProps {
    pub values: Vec<AttrValue>,
}

#[function_component(BreadCrumbs)]
pub fn bread_crumbs(props: &BreadCrumbsProps) -> Html {
    let BreadCrumbsProps { values } = props;
    let length = values.len();

    values
        .into_iter()
        .enumerate()
        .map(|(key, name)| {
            html! {
                <div {key} class={classes!("bread-crumbs")}>
                    <p class={classes!("text-size-r16")}>{ name }</p>

                    if length != key {
                        <span class={classes!("text-size-r16", "icon-caretRight")}></span>
                    }
                </div>
            }
        })
        .collect::<Html>()
}
