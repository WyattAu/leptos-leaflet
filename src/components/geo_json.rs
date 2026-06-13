use leptos::prelude::*;

use crate::types::GeoJsonOptions;

/// A GeoJSON layer that renders geographic data on the map.
///
/// # Example
/// ```rust,no_run
/// use leptos_leaflet::{GeoJsonLayer, GeoJsonOptions};
///
/// view! {
///     <GeoJsonLayer
///         data=geojson_data
///         options=GeoJsonOptions::default()
///     />
/// }
/// ```
#[component]
pub fn GeoJsonLayer(
    data: String,
    #[prop(optional)] _options: GeoJsonOptions,
) -> impl IntoView {
    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }

        // GeoJSON layer is created and attached by the parent Map component
    });

    view! {
        <div
            data-leaflet-geo-json="true"
            data-geo-data=data
            style="display:none"
        ></div>
    }
}
