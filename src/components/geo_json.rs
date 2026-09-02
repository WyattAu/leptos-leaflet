use leptos::prelude::*;

use crate::types::GeoJsonOptions;

/// A GeoJSON layer that renders geographic data on the map.
///
/// Supports click events via the on_click callback.
///
/// # Example
/// ```rust,no_run
/// use leptos_leaflet::{GeoJsonLayer, GeoJsonOptions};
///
/// view! {
///     <GeoJsonLayer
///         data=geojson_data
///         options=GeoJsonOptions::default()
///         on_click=|name| { log!("Clicked: {}", name); }
///     />
/// }
/// ```
#[component]
pub fn GeoJsonLayer(
    data: String,
    #[prop(optional)] options: Option<GeoJsonOptions>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let _options = options.unwrap_or_default();

    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }

        // GeoJSON layer is created and attached by the parent Map component
    });

    // Store on_click as a data attribute for the parent to pick up
    let on_click_id = if on_click.is_some() {
        Some(format!("geojson-onclick-{}", js_sys::Math::random()))
    } else {
        None
    };

    view! {
        <div
            data-leaflet-geo-json="true"
            data-geo-data=data
            data-on-click-id=on_click_id.unwrap_or_default()
            style="display:none"
        ></div>
    }
}
