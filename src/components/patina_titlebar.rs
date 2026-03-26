use dioxus::prelude::*;
use dioxus::desktop::use_window;

// Import SVG icons
static MINIMIZE_ICON: &str = include_str!("../../assets/icon_minimize.svg");
static MAXIMIZE_ICON: &str = include_str!("../../assets/icon_maximize.svg");
static CLOSE_ICON: &str = include_str!("../../assets/icon_close.svg");

#[component]
pub fn PatinaTitlebar() -> Element {
    let window = use_window();
    let svg_content = include_str!("../../assets/titlebar.svg");

    rsx! {
        // Custom titlebar
        div {
            class: "patina-titlebar",
            // Interactive Elements Layer
            div {
                class: "right",
                button {
                    class: "patina-titlebar-button",
                    id: "minimize",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.set_minimized(true);
                        }
                    },
                    dangerous_inner_html: MINIMIZE_ICON
                }
                button {
                    class: "patina-titlebar-button",
                    id: "maximize",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.toggle_maximized();
                        }
                    },
                    dangerous_inner_html: MAXIMIZE_ICON
                }
                button {
                    class: "patina-titlebar-button",
                    id: "close",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.close();
                        }
                    },
                    dangerous_inner_html: CLOSE_ICON
                }
            }
            div {
                class: "drag-region left title svg-titlebar",
                dangerous_inner_html: svg_content,
                onmousedown: {
                    let window = window.clone();
                    move |_| { 
                        window.drag(); 
                    }
                },
                span { "Patina UI Framework" }
            }
        }
    }
}
