use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PatinaBGProps {
    pub svg_content: String,
}

#[component]
pub fn PatinaBG(props: PatinaBGProps) -> Element {
    rsx! {
        div {
            class: "patina-bg",
            div {
                class: "svg-fill-maintain-aspect",
                dangerous_inner_html: "{props.svg_content}"
            }
        }
    }
}