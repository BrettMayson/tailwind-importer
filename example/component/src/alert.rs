use yew::prelude::*;

use crate::Level;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AlertProps {
    #[prop_or_default]
    pub level: Level,
    pub label: Option<String>,
    pub text: String,
    #[prop_or(false)]
    pub border: bool,
    #[prop_or(false)]
    pub rounded: bool,

    pub children: Option<Children>,
}

#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let label = props
        .label
        .clone()
        .map(|label| {
            html! {
                <span class="font-medium mr-2">{ label }</span>
            }
        })
        .unwrap_or_default();
    let children = props.children.clone().unwrap_or_default();
    let mut class = classes!("p-4", "mb-4", "text-sm");
    if props.border {
        class.push("border-t-4");
        class.push(props.level.border());
    }
    if props.rounded {
        class.push("rounded-lg")
    }
    class.push(props.level.text());
    class.push(props.level.bg());
    html! {
        <div { class } role="alert">
            { label } { &props.text }
            { children }
        </div>
    }
}
