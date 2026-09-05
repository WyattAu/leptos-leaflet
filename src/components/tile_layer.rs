use leptos::prelude::*;

use crate::types::TileLayerOptions;

/// A tile layer that loads map tiles from a URL template.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::{TileLayer, TileLayerOptions};
///
/// let _ = view! {
///     <TileLayer
///         url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
///         options=TileLayerOptions::default()
///     />
/// };
/// ```
#[component]
pub fn TileLayer(
    /// URL template for tile images (e.g., `https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png`).
    #[prop(into)] url: String,
    /// Tile layer options.
    #[prop(optional)] options: Option<TileLayerOptions>,
) -> impl IntoView {
    let _options = options.unwrap_or_default();

    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }
        // TileLayer is created and attached by the parent Map component
    });

    let url_clone = url.clone();
    view! {
        <div
            data-leaflet-tile-layer=url_clone
            style="display:none"
        ></div>
    }
}
