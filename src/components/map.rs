use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::ffi;
use crate::types::MapOptions;

/// The main Map component. Wraps a Leaflet map instance.
///
/// # Example
/// ```rust,no_run
/// use leptos_leaflet::{Map, MapOptions, TileLayer};
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
    #[prop(into)] id: String,
    #[prop(optional)] options: Option<MapOptions>,
    #[prop(optional)] _on_init: Option<Box<dyn Fn(web_sys::Element) + Send + Sync>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let options = options.unwrap_or_default();
    let id_for_closure = id.clone();

    // Initialize map after mount
    Effect::new(move |_| {
        let window = web_sys::window();
        if window.is_none() {
            return; // SSR, skip
        }

        let id_clone = id_for_closure.clone();
        let options_clone = options.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // Load Leaflet
            if let Err(e) = ffi::load_leaflet().await {
                leptos::logging::error!("Failed to load Leaflet: {:?}", e);
                return;
            }

            // Get container element
            let document = web_sys::window().unwrap().document().unwrap();
            let _container = match document.get_element_by_id(&id_clone) {
                Some(el) => el,
                None => {
                    leptos::logging::error!("Map container #{} not found", id_clone);
                    return;
                }
            };

            // Create map options
            let opts = js_sys::Object::new();
            js_sys::Reflect::set(&opts, &"center".into(), &to_js_array(&options_clone.center)).unwrap();
            js_sys::Reflect::set(&opts, &"zoom".into(), &JsValue::from_f64(options_clone.zoom as f64)).unwrap();
            if let Some(min) = options_clone.min_zoom {
                js_sys::Reflect::set(&opts, &"minZoom".into(), &JsValue::from_f64(min as f64)).unwrap();
            }
            if let Some(max) = options_clone.max_zoom {
                js_sys::Reflect::set(&opts, &"maxZoom".into(), &JsValue::from_f64(max as f64)).unwrap();
            }
            js_sys::Reflect::set(&opts, &"zoomControl".into(), &JsValue::from_bool(options_clone.zoom_control)).unwrap();
            js_sys::Reflect::set(&opts, &"attributionControl".into(), &JsValue::from_bool(options_clone.attribution_control)).unwrap();
            js_sys::Reflect::set(&opts, &"continuousWorld".into(), &JsValue::from_bool(options_clone.continuous_world)).unwrap();

            // Create map
            let map = ffi::create_map(&id_clone, &opts.into());

            // Invalidate size after layout settle
            let map_clone = map.clone();
            let callback = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                ffi::invalidate_size(&map_clone);
            });
            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    200,
                );
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
fn to_js_array(arr: &[f64]) -> JsValue {
    let js_arr = js_sys::Array::new();
    for &val in arr {
        js_arr.push(&JsValue::from_f64(val));
    }
    js_arr.into()
}
