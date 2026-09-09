use wasm_bindgen::prelude::*;

// ============================================================
// Leaflet FFI Bindings
// ============================================================
// These bindings wrap the Leaflet JavaScript API for use from Rust/WASM.
// They use JsValue for most operations since Leaflet objects are opaque JS objects.

/// Check if Leaflet is loaded
pub fn is_loaded() -> bool {
    js_sys::Reflect::get(&js_sys::global(), &"L".into())
        .map(|v| !v.is_undefined() && !v.is_null())
        .unwrap_or(false)
}

/// Load Leaflet from CDN
pub async fn load_leaflet() -> Result<(), JsValue> {
    if is_loaded() {
        return Ok(());
    }

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;

    // Inject CSS
    let link = document
        .create_element("link")
        .map_err(|e| format!("create link: {:?}", e))?;
    link.set_attribute("rel", "stylesheet")
        .map_err(|e| format!("set rel: {:?}", e))?;
    link.set_attribute("href", "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css")
        .map_err(|e| format!("set href: {:?}", e))?;
    document
        .head()
        .ok_or_else(|| JsValue::from_str("no head"))?
        .append_child(&link)
        .map_err(|e| format!("append link: {:?}", e))?;

    // Inject JS and wait for load
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("no document"));
            return;
        };
        let Ok(script) = document.create_element("script") else {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("create script"));
            return;
        };
        if script
            .set_attribute("src", "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js")
            .is_err()
        {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("set script src"));
            return;
        }

        // The load/error callbacks must be real closures attached to the
        // script element; otherwise the promise never settles and callers
        // hang forever waiting for the future.
        let r1 = resolve.clone();
        let onload = Closure::<dyn Fn()>::new(move || {
            r1.call1(&JsValue::NULL, &JsValue::NULL).ok();
        });
        let onload_cb = onload.into_js_value();
        if script
            .add_event_listener_with_callback("load", onload_cb.unchecked_ref())
            .is_err()
        {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("listen load"));
            return;
        }

        let r2 = reject.clone();
        let onerror = Closure::<dyn Fn()>::new(move || {
            r2.call1(&JsValue::NULL, &JsValue::NULL).ok();
        });
        let onerror_cb = onerror.into_js_value();
        if script
            .add_event_listener_with_callback("error", onerror_cb.unchecked_ref())
            .is_err()
        {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("listen error"));
            return;
        }

        let Some(head) = document.head() else {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("no head"));
            return;
        };
        if head.append_child(&script).is_err() {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("append script"));
        }
    });

    wasm_bindgen_futures::JsFuture::from(promise).await?;

    // Wait for L to be defined
    let mut attempts = 0;
    while !is_loaded() && attempts < 50 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await?;
        attempts += 1;
    }

    if !is_loaded() {
        return Err(JsValue::from_str("Leaflet failed to load"));
    }

    Ok(())
}

/// Set a property on a freshly created JS options object.
///
/// INVARIANT: `obj` is a plain `Object` created by this crate, so
/// `Reflect::set` cannot fail (it has no non-writable properties); the
/// result is ignored so a broken JS bridge cannot panic the WASM app.
pub(crate) fn set_prop(obj: &JsValue, key: &str, value: JsValue) {
    let _ = js_sys::Reflect::set(obj, &JsValue::from_str(key), &value);
}

/// Resolve `L.<name>` as a JS function paired with the global `L` object.
///
/// Returns `None` when Leaflet is not loaded on the page; callers degrade
/// to `JsValue::NULL` instead of panicking the WASM app.
fn leaflet_fn(name: &str) -> Option<(js_sys::Function, JsValue)> {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).ok()?;
    let f = js_sys::Reflect::get(&l, &JsValue::from_str(name))
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    Some((f, l))
}

/// Create a map instance. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_map(id: &str, options: &JsValue) -> JsValue {
    match leaflet_fn("map") {
        Some((map_fn, l)) => map_fn
            .call2(&l, &JsValue::from_str(id), options)
            .unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a tile layer. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_tile_layer(url: &str, options: &JsValue) -> JsValue {
    match leaflet_fn("tileLayer") {
        Some((tile_layer_fn, l)) => tile_layer_fn
            .call2(&l, &JsValue::from_str(url), options)
            .unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a circle marker. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_circle_marker(latlng: &JsValue, options: &JsValue) -> JsValue {
    match leaflet_fn("circleMarker") {
        Some((circle_fn, l)) => circle_fn
            .call2(&l, latlng, options)
            .unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a feature group. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_feature_group() -> JsValue {
    match leaflet_fn("featureGroup") {
        Some((fg_fn, l)) => fg_fn.call0(&l).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a layer group. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_layer_group() -> JsValue {
    match leaflet_fn("layerGroup") {
        Some((lg_fn, l)) => lg_fn.call0(&l).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a GeoJSON layer. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_geo_json(data: &JsValue, options: &JsValue) -> JsValue {
    match leaflet_fn("geoJSON") {
        Some((geo_fn, l)) => geo_fn.call2(&l, data, options).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a div icon. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_div_icon(options: &JsValue) -> JsValue {
    match leaflet_fn("divIcon") {
        Some((div_icon_fn, l)) => div_icon_fn.call1(&l, options).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create a LatLng. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_lat_lng(lat: f64, lng: f64) -> JsValue {
    match leaflet_fn("latLng") {
        Some((lat_lng_fn, l)) => lat_lng_fn
            .call2(&l, &JsValue::from_f64(lat), &JsValue::from_f64(lng))
            .unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Create LatLngBounds. Returns `JsValue::NULL` if Leaflet is not loaded.
pub fn create_lat_lng_bounds(sw: &JsValue, ne: &JsValue) -> JsValue {
    match leaflet_fn("latLngBounds") {
        Some((bounds_fn, l)) => bounds_fn.call2(&l, sw, ne).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Call a method on a JS object
pub fn call_method(obj: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let method_fn = js_sys::Reflect::get(obj, &method.into())?;
    let method_fn: js_sys::Function = method_fn.into();
    let args_arr = js_sys::Array::new();
    for arg in args {
        args_arr.push(arg);
    }
    method_fn.apply(obj, &args_arr)
}

/// Add a layer to a map
pub fn add_layer_to_map(map: &JsValue, layer: &JsValue) {
    let _ = call_method(map, "addLayer", std::slice::from_ref(layer));
}

/// Remove a layer from a map
pub fn remove_layer_from_map(map: &JsValue, layer: &JsValue) {
    let _ = call_method(map, "removeLayer", std::slice::from_ref(layer));
}

/// Add a layer to a layer group
pub fn add_layer_to_group(group: &JsValue, layer: &JsValue) {
    let _ = call_method(group, "addLayer", std::slice::from_ref(layer));
}

/// Clear all layers from a group
pub fn clear_layers(group: &JsValue) {
    let _ = call_method(group, "clearLayers", &[]);
}

/// Get bounds from a layer group
pub fn get_bounds(group: &JsValue) -> JsValue {
    call_method(group, "getBounds", &[]).unwrap_or(JsValue::NULL)
}

/// Fit map to bounds
pub fn fit_bounds(map: &JsValue, bounds: &JsValue, options: &JsValue) {
    let _ = call_method(map, "fitBounds", &[bounds.clone(), options.clone()]);
}

/// Invalidate map size
pub fn invalidate_size(map: &JsValue) {
    let _ = call_method(map, "invalidateSize", &[JsValue::NULL]);
}

/// Bind a popup to a layer
pub fn bind_popup(layer: &JsValue, content: &str) {
    let _ = call_method(layer, "bindPopup", &[JsValue::from_str(content)]);
}

/// Bind a tooltip to a layer
pub fn bind_tooltip(layer: &JsValue, content: &str) {
    let _ = call_method(layer, "bindTooltip", &[JsValue::from_str(content)]);
}

/// Set style on a layer
pub fn set_style(layer: &JsValue, options: &JsValue) {
    let _ = call_method(layer, "setStyle", std::slice::from_ref(options));
}

/// Reset style on a layer
pub fn reset_style(geo_layer: &JsValue, layer: &JsValue) {
    let _ = call_method(geo_layer, "resetStyle", std::slice::from_ref(layer));
}

/// Fire an event on a layer
pub fn fire_event(layer: &JsValue, event_type: &str) {
    let _ = call_method(layer, "fire", &[JsValue::from_str(event_type)]);
}

/// Add a click handler to a layer
pub fn on_click(layer: &JsValue, callback: js_sys::Function) {
    let _ = call_method(layer, "on", &[JsValue::from_str("click"), callback.into()]);
}

/// Add a mouseover handler to a layer
pub fn on_mouseover(layer: &JsValue, callback: js_sys::Function) {
    let _ = call_method(
        layer,
        "on",
        &[JsValue::from_str("mouseover"), callback.into()],
    );
}

/// Add a mouseout handler to a layer
pub fn on_mouseout(layer: &JsValue, callback: js_sys::Function) {
    let _ = call_method(
        layer,
        "on",
        &[JsValue::from_str("mouseout"), callback.into()],
    );
}

/// Iterate layers in a group
pub fn each_layer(group: &JsValue, callback: js_sys::Function) {
    let _ = call_method(group, "eachLayer", &[callback.into()]);
}

/// Get layers from a group
pub fn get_layers(group: &JsValue) -> JsValue {
    call_method(group, "getLayers", &[]).unwrap_or(JsValue::NULL)
}

/// Remove a map instance
pub fn remove_map(map: &JsValue) {
    let _ = call_method(map, "remove", &[]);
}
