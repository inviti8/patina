use dioxus::prelude::*;
use crate::components::{PatinaBG, PatinaContainer};

#[derive(Props, Clone, PartialEq)]
pub struct PatinaViewProps {
    pub rows: u32,
    pub columns: u32,
    pub bg_svg: String,
    pub container_svg: String,
}

#[component]
pub fn PatinaView(props: PatinaViewProps) -> Element {
    rsx! {
        div {
            class: "patina-view",
            // Background layer
            PatinaBG { svg_content: props.bg_svg.clone() }
            
            // Grid overlay
            div {
                class: "patina-view-grid",
                // Generate rows
                for row in 0..props.rows {
                    div {
                        class: "patina-view-row",
                        key: "{row}",
                        // Generate columns for this row
                        for col in 0..props.columns {
                            PatinaContainer { 
                                key: "{row}-{col}",
                                svg_content: props.container_svg.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}