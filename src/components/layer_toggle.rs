use leptos::prelude::*;
use std::rc::Rc;

/// A toggle button for controlling layer visibility.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos::logging::log;
/// use leptos_leaflet::LayerToggle;
///
/// let _ = view! {
///     <LayerToggle
///         label="Earthquakes"
///         active=true
///         on_toggle=std::rc::Rc::new(|active| { log!("Layer toggled: {}", active); })
///     />
/// };
/// ```
#[component]
pub fn LayerToggle(
    /// Label text for the toggle button.
    #[prop(into)] label: String,
    /// Whether the layer is currently active.
    #[prop(optional)] active: Option<bool>,
    /// Callback fired when the toggle state changes.
    #[prop(optional)] on_toggle: Option<Rc<dyn Fn(bool)>>,
) -> impl IntoView {
    let is_active = active.unwrap_or(true);
    let (current_active, set_current_active) = signal(is_active);

    let label_clone = label.clone();

    view! {
        <button
            class=move || {
                if current_active.get() {
                    "layer-toggle active"
                } else {
                    "layer-toggle"
                }
            }
            aria-pressed=move || current_active.get().to_string()
            on:click=move |_| {
                let new_state = !current_active.get();
                set_current_active.set(new_state);
                if let Some(ref callback) = on_toggle {
                    callback(new_state);
                }
            }
        >
            {label_clone}
        </button>
    }
}

/// A group of layer toggles for controlling multiple layers.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos::logging::log;
/// use leptos_leaflet::{LayerToggle, LayerToggleGroup};
///
/// let _ = view! {
///     <LayerToggleGroup>
///         <LayerToggle label="Quakes" active=true />
///         <LayerToggle label="Borders" active=true />
///         <LayerToggle label="Capitals" active=false />
///     </LayerToggleGroup>
/// };
/// ```
#[component]
pub fn LayerToggleGroup(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="layer-toggle-group" role="group" aria-label="Layer controls">
            {children.map(|c| c())}
        </div>
    }
}
