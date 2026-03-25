// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use dioxus::desktop::tao;
use dioxus::desktop::use_window;

use components::PatinaBG;

/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    let window = tao::window::WindowBuilder::new()
        .with_resizable(true)
        .with_decorations(false)  // Frameless - no title bar, borders
        .with_transparent(true);  // Optional: transparent background
    dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::new().with_window(window)).launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
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
        
        // Main content with SVG background
        PatinaBG {}
    }
}
