use dioxus::prelude::*;
use crate::components::{PatinaBG, PatinaContainer};

#[derive(Props, Clone, PartialEq)]
pub struct PatinaViewProps {
    pub rows: u32,
    pub columns: u32,
}

#[component]
pub fn PatinaView(props: PatinaViewProps) -> Element {
    let mut containers = Vec::new();
    
    // Generate grid of containers
    for row in 0..props.rows {
        for col in 0..props.columns {
            containers.push(rsx! {
                PatinaContainer { key: "{row}-{col}" }
            });
        }
    }
    
    rsx! {
        div {
            class: "patina-view",
            // Background layer
            PatinaBG {}
            
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
                            PatinaContainer { key: "{row}-{col}" }
                        }
                    }
                }
            }
        }
    }
}