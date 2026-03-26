//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub mod patina_bg;
pub mod patina_titlebar;
pub mod patina_container;
pub mod patina_view;

pub use patina_bg::PatinaBG;
pub use patina_titlebar::PatinaTitlebar;
pub use patina_container::PatinaContainer;
pub use patina_view::PatinaView;
