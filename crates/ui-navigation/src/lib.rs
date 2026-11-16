//! Controlled native navigation. Retain one [`NavigationState`] entity and both
//! its event subscription and an observer in the application view constructor.
//! Setters are application operations outside render; interactions only request
//! changes. Before hiding a region or dropping the entity, call
//! [`NavigationState::restore_focus`] with a mounted application fallback.
mod model;
mod state;
mod view;
pub use model::*;
pub use state::*;
pub use view::Navigation;
#[cfg(test)]
mod tests;
