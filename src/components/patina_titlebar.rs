use dioxus::prelude::*;
use dioxus::desktop::use_window;

#[component]
pub fn PatinaTitlebar() -> Element {
    let window = use_window();

    rsx! {
        // Custom titlebar
        div {
            class: "titlebar",
            div {
                class: "drag-region left title",
                onmousedown: {
                    let window = window.clone();
                    move |_| { 
                        window.drag(); 
                    }
                },
                span { "Patina UI Framework" }
            }
            div {
                class: "right",
                button {
                    class: "titlebar-button",
                    id: "minimize",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.set_minimized(true);
                        }
                    },
                    "─"
                }
                button {
                    class: "titlebar-button",
                    id: "maximize",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.toggle_maximized();
                        }
                    },
                    "□"
                }
                button {
                    class: "titlebar-button",
                    id: "close",
                    onclick: {
                        let window = window.clone();
                        move |_| {
                            window.close();
                        }
                    },
                    "✕"
                }
            }
        }
    }
}
