use dioxus::prelude::*;

#[component]
pub fn PatinaContainer() -> Element {
    // Parse SVG as XML and extract key elements
    let svg_content = include_str!("../../assets/container.svg");
    
    rsx! {
        div {
            class: "patina-container",
            div {
                class: "svg-fill-maintain-aspect",
                dangerous_inner_html: svg_content
            }
        }
    }
}