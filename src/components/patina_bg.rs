use dioxus::prelude::*;

#[component]
pub fn PatinaBG() -> Element {
    // Parse SVG as XML and extract key elements
    let svg_content = include_str!("../../assets/bg.svg");
    
    rsx! {
        div {
            class: "patina-bg",
            dangerous_inner_html: svg_content
        }
    }
}