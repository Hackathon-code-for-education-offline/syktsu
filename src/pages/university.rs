use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UniversityProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(University)]
pub fn university(props: &UniversityProps) -> Html {
    let UniversityProps { id } = props;

    html! {
        <div>
            { id }
        </div>
    }
}

#[function_component(Universities)]
pub fn universities() -> Html {
    html! {
        <div>
            { "List here" }
        </div>
    }
}
