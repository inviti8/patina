use dioxus::prelude::*;
use crate::components::PatinaBG;

#[derive(Props, Clone, PartialEq)]
pub struct PatinaContainerProps {
    pub svg_content: String,
}

#[component]
pub fn PatinaContainer(props: PatinaContainerProps) -> Element {
    rsx! {
        div {
            class: "patina-container",
            PatinaBG { svg_content: props.svg_content.clone() }
        }
    }
}