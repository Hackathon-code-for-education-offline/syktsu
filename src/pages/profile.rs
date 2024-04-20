use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {
    id: AttrValue
}

#[function_component]
pub fn ComponentName(props: &ProfileProps) -> Html {
    let ProfileProps { id } = props;

    html! {
        <div>
            { id }
        </div>
    }
}