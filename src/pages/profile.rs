use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {
    pub id: AttrValue,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps { id } = props;

    html! {
        <div>
            { id }
        </div>
    }
}

#[function_component(MyProfile)]
pub fn my_profile() -> Html {
    html! {
        <div>
            { "My profile" }
        </div>
    }
}
