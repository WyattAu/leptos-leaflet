use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::types::CircleMarkerOptions;

/// A circle marker placed on the map at a specific lat/lng.
///
/// # Example
/// ```rust,no_run
/// use leptos_leaflet::{CircleMarker, CircleMarkerOptions, LatLng};
///
/// view! {
///     <CircleMarker
///         latlng=LatLng::new(51.5, -0.1)
///         options=CircleMarkerOptions {
///             radius: 10.0,
///             color: "#ff0000".to_string(),
///             fill_color: "#ff0000".to_string(),
///             ..Default::default()
///         }
///     />
/// }
/// ```
#[component]
pub fn CircleMarker(
    latlng: crate::types::LatLng,
    #[prop(optional)] options: CircleMarkerOptions,
) -> impl IntoView {
    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }

        // CircleMarker is created and attached by the parent LayerGroup
        // This component provides the configuration
    });

    let lat = latlng.lat;
    let lng = latlng.lng;
    let radius = options.radius;
    let color = options.color.clone();
    let fill_color = options.fill_color.clone();
    let weight = options.weight;
    let opacity = options.opacity;
    let fill_opacity = options.fill_opacity;

    view! {
        <div
            data-leaflet-circle-marker="true"
            data-lat=lat.to_string()
            data-lng=lng.to_string()
            data-radius=radius.to_string()
            data-color=color
            data-fill-color=fill_color
            data-weight=weight.to_string()
            data-opacity=opacity.to_string()
            data-fill-opacity=fill_opacity.to_string()
            style="display:none"
        ></div>
    }
}
