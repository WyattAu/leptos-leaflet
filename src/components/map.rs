use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::ffi;
use crate::types::MapOptions;

/// The main Map component. Wraps a Leaflet map instance.
///
/// Provides reactive signals for map state and methods for interacting
/// with the map programmatically.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet_wyatt::{Map, MapOptions, TileLayer};
///
/// #[component]
/// fn MyMap() -> impl IntoView {
///     view! {
///         <Map id="my-map" options=MapOptions::default()>
///             <TileLayer url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"/>
///         </Map>
///     }
/// }
/// ```
#[component]
pub fn Map(
    /// Unique identifier for the map container element.
    #[prop(into)]
    id: String,
    /// Map initialization options.
    #[prop(optional)]
    options: Option<MapOptions>,
    /// Callback fired after map initialization.
    #[prop(optional)]
    _on_init: Option<Box<dyn Fn(web_sys::Element) + Send + Sync>>,
    /// Child layers to add to the map.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let options = options.unwrap_or_default();
    let id_for_clone = id.clone();

    // Create reactive signals for map state
    let (_map_ready, set_map_ready) = signal(false);
    let (_map_center, set_map_center) = signal(options.center);
    let (_map_zoom, set_map_zoom) = signal(options.zoom);

    // Store the map JS object in a RefCell for imperative access
    // Use Rc to allow sharing across closures
    let map_js = std::rc::Rc::new(std::cell::RefCell::<Option<JsValue>>::new(None));
    let map_js_for_async = map_js.clone();

    // Initialize map after mount
    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }

        let id_clone = id_for_clone.clone();
        let options_clone = options.clone();
        let map_js = map_js_for_async.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // Load Leaflet
            if let Err(e) = ffi::load_leaflet().await {
                leptos::logging::error!("Failed to load Leaflet: {:?}", e);
                return;
            }

            // Get container element
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                leptos::logging::error!("No browser document for map #{}", id_clone);
                return;
            };
            let _container = match document.get_element_by_id(&id_clone) {
                Some(el) => el,
                None => {
                    leptos::logging::error!("Map container #{} not found", id_clone);
                    return;
                }
            };

            // Create map options
            let opts = js_sys::Object::new();
            ffi::set_prop(&opts, "center", to_js_array(&options_clone.center));
            ffi::set_prop(&opts, "zoom", JsValue::from_f64(options_clone.zoom as f64));
            if let Some(min) = options_clone.min_zoom {
                ffi::set_prop(&opts, "minZoom", JsValue::from_f64(min as f64));
            }
            if let Some(max) = options_clone.max_zoom {
                ffi::set_prop(&opts, "maxZoom", JsValue::from_f64(max as f64));
            }
            ffi::set_prop(
                &opts,
                "zoomControl",
                JsValue::from_bool(options_clone.zoom_control),
            );
            ffi::set_prop(
                &opts,
                "attributionControl",
                JsValue::from_bool(options_clone.attribution_control),
            );
            ffi::set_prop(
                &opts,
                "continuousWorld",
                JsValue::from_bool(options_clone.continuous_world),
            );

            // Create map
            let map = ffi::create_map(&id_clone, &opts.into());

            // Store map reference
            map_js.borrow_mut().replace(map.clone());

            // Update reactive signals
            set_map_center.set(options_clone.center);
            set_map_zoom.set(options_clone.zoom);
            set_map_ready.set(true);

            // Invalidate size after layout settle
            let map_clone = map;
            let callback = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                ffi::invalidate_size(&map_clone);
            });
            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    200,
                );
            }
            callback.forget();
        });
    });

    let id_clone = id.clone();

    view! {
        <div id=id_clone class="world-map" role="region" aria-label="Interactive map">
            {children.map(|c| c())}
        </div>
    }
}

/// Helper to convert a Rust array to a JavaScript array
pub fn to_js_array(arr: &[f64]) -> JsValue {
    let js_arr = js_sys::Array::new();
    for &val in arr {
        js_arr.push(&JsValue::from_f64(val));
    }
    js_arr.into()
}
