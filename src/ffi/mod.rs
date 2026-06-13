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
    let document = window.document().ok_or_else(|| JsValue::from_str("no document"))?;

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
        let document = web_sys::window().unwrap().document().unwrap();
        let script = document.create_element("script").unwrap();
        script
            .set_attribute("src", "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js")
            .unwrap();

        let r1 = resolve.clone();
        let _onload = move || {
            r1.call1(&JsValue::NULL, &JsValue::NULL).ok();
        };
        let onload_cb = js_sys::Function::new_no_args("");
        script
            .add_event_listener_with_callback("load", &onload_cb)
            .unwrap();

        let r2 = reject.clone();
        let _onerror = move || {
            r2.call1(&JsValue::NULL, &JsValue::NULL).ok();
        };
        let onerror_cb = js_sys::Function::new_no_args("");
        script
            .add_event_listener_with_callback("error", &onerror_cb)
            .unwrap();

        document
            .head()
            .unwrap()
            .append_child(&script)
            .unwrap();
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

/// Create a map instance
pub fn create_map(id: &str, options: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let map_fn = js_sys::Reflect::get(&l, &"map".into()).unwrap();
    let map_fn: js_sys::Function = map_fn.into();
    map_fn.call2(&l, &JsValue::from_str(id), options).unwrap()
}

/// Create a tile layer
pub fn create_tile_layer(url: &str, options: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let tile_layer_fn = js_sys::Reflect::get(&l, &"tileLayer".into()).unwrap();
    let tile_layer_fn: js_sys::Function = tile_layer_fn.into();
    tile_layer_fn.call2(&l, &JsValue::from_str(url), options).unwrap()
}

/// Create a circle marker
pub fn create_circle_marker(latlng: &JsValue, options: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let circle_fn = js_sys::Reflect::get(&l, &"circleMarker".into()).unwrap();
    let circle_fn: js_sys::Function = circle_fn.into();
    circle_fn.call2(&l, latlng, options).unwrap()
}

/// Create a feature group
pub fn create_feature_group() -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let fg_fn = js_sys::Reflect::get(&l, &"featureGroup".into()).unwrap();
    let fg_fn: js_sys::Function = fg_fn.into();
    fg_fn.call0(&l).unwrap()
}

/// Create a layer group
pub fn create_layer_group() -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let lg_fn = js_sys::Reflect::get(&l, &"layerGroup".into()).unwrap();
    let lg_fn: js_sys::Function = lg_fn.into();
    lg_fn.call0(&l).unwrap()
}

/// Create a GeoJSON layer
pub fn create_geo_json(data: &JsValue, options: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let geo_fn = js_sys::Reflect::get(&l, &"geoJSON".into()).unwrap();
    let geo_fn: js_sys::Function = geo_fn.into();
    geo_fn.call2(&l, data, options).unwrap()
}

/// Create a div icon
pub fn create_div_icon(options: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let div_icon_fn = js_sys::Reflect::get(&l, &"divIcon".into()).unwrap();
    let div_icon_fn: js_sys::Function = div_icon_fn.into();
    div_icon_fn.call1(&l, options).unwrap()
}

/// Create a LatLng
pub fn create_lat_lng(lat: f64, lng: f64) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let lat_lng_fn = js_sys::Reflect::get(&l, &"latLng".into()).unwrap();
    let lat_lng_fn: js_sys::Function = lat_lng_fn.into();
    lat_lng_fn.call2(&l, &JsValue::from_f64(lat), &JsValue::from_f64(lng)).unwrap()
}

/// Create LatLngBounds
pub fn create_lat_lng_bounds(sw: &JsValue, ne: &JsValue) -> JsValue {
    let l = js_sys::Reflect::get(&js_sys::global(), &"L".into()).unwrap();
    let bounds_fn = js_sys::Reflect::get(&l, &"latLngBounds".into()).unwrap();
    let bounds_fn: js_sys::Function = bounds_fn.into();
    bounds_fn.call2(&l, sw, ne).unwrap()
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
    let _ = call_method(map, "addLayer", &[layer.clone()]);
}

/// Remove a layer from a map
pub fn remove_layer_from_map(map: &JsValue, layer: &JsValue) {
    let _ = call_method(map, "removeLayer", &[layer.clone()]);
}

/// Add a layer to a layer group
pub fn add_layer_to_group(group: &JsValue, layer: &JsValue) {
    let _ = call_method(group, "addLayer", &[layer.clone()]);
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
    let _ = call_method(layer, "setStyle", &[options.clone()]);
}

/// Reset style on a layer
pub fn reset_style(geo_layer: &JsValue, layer: &JsValue) {
    let _ = call_method(geo_layer, "resetStyle", &[layer.clone()]);
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
    let _ = call_method(layer, "on", &[JsValue::from_str("mouseover"), callback.into()]);
}

/// Add a mouseout handler to a layer
pub fn on_mouseout(layer: &JsValue, callback: js_sys::Function) {
    let _ = call_method(layer, "on", &[JsValue::from_str("mouseout"), callback.into()]);
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
