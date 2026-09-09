use wasm_bindgen::prelude::*;

use crate::ffi;
use crate::types::{CircleMarkerOptions, GeoJsonOptions, LatLng, LatLngBounds, TileLayerOptions};

/// Create a tile layer and return its JS object
pub fn create_tile_layer(url: &str, options: Option<TileLayerOptions>) -> JsValue {
    let opts = options.unwrap_or_default();
    let js_opts = js_sys::Object::new();
    if let Some(max_zoom) = opts.max_zoom {
        crate::ffi::set_prop(&js_opts, "maxZoom", JsValue::from_f64(max_zoom as f64));
    }
    crate::ffi::set_prop(
        &js_opts,
        "crossOrigin",
        JsValue::from_bool(opts.cross_origin),
    );
    if let Some(ref attribution) = opts.attribution {
        crate::ffi::set_prop(&js_opts, "attribution", JsValue::from_str(attribution));
    }

    ffi::create_tile_layer(url, &js_opts.into())
}

/// Create a circle marker and return its JS object
pub fn create_circle_marker(latlng: &LatLng, options: CircleMarkerOptions) -> JsValue {
    let ll = ffi::create_lat_lng(latlng.lat, latlng.lng);
    let js_opts = js_sys::Object::new();
    crate::ffi::set_prop(&js_opts, "radius", JsValue::from_f64(options.radius));
    crate::ffi::set_prop(&js_opts, "color", JsValue::from_str(&options.color));
    crate::ffi::set_prop(
        &js_opts,
        "fillColor",
        JsValue::from_str(&options.fill_color),
    );
    crate::ffi::set_prop(&js_opts, "weight", JsValue::from_f64(options.weight));
    crate::ffi::set_prop(&js_opts, "opacity", JsValue::from_f64(options.opacity));
    crate::ffi::set_prop(
        &js_opts,
        "fillOpacity",
        JsValue::from_f64(options.fill_opacity),
    );

    ffi::create_circle_marker(&ll, &js_opts.into())
}

/// Create a GeoJSON layer and return its JS object
pub fn create_geo_json_layer(
    data: &str,
    options: Option<GeoJsonOptions>,
) -> Result<JsValue, String> {
    let js_data = wasm_bindgen::JsValue::from_str(data);

    let opts = options.unwrap_or_default();
    let js_opts = js_sys::Object::new();
    if let Some(ref fill_color) = opts.fill_color {
        crate::ffi::set_prop(&js_opts, "fillColor", JsValue::from_str(fill_color));
    }
    if let Some(fill_opacity) = opts.fill_opacity {
        crate::ffi::set_prop(&js_opts, "fillOpacity", JsValue::from_f64(fill_opacity));
    }
    if let Some(ref color) = opts.color {
        crate::ffi::set_prop(&js_opts, "color", JsValue::from_str(color));
    }
    if let Some(weight) = opts.weight {
        crate::ffi::set_prop(&js_opts, "weight", JsValue::from_f64(weight));
    }
    crate::ffi::set_prop(
        &js_opts,
        "interactive",
        JsValue::from_bool(opts.interactive),
    );

    Ok(ffi::create_geo_json(&js_data, &js_opts.into()))
}

/// Create a feature group and return its JS object
pub fn create_feature_group() -> JsValue {
    ffi::create_feature_group()
}

/// Create a layer group and return its JS object
pub fn create_layer_group() -> JsValue {
    ffi::create_layer_group()
}

/// Bind a popup to a layer
pub fn bind_popup(layer: &JsValue, content: &str) {
    ffi::bind_popup(layer, content);
}

/// Bind a tooltip to a layer
pub fn bind_tooltip(layer: &JsValue, content: &str) {
    ffi::bind_tooltip(layer, content);
}

/// Set style on a layer
pub fn set_style(layer: &JsValue, options: &JsValue) {
    ffi::set_style(layer, options);
}

/// Reset style on a GeoJSON layer
pub fn reset_style(geo_layer: &JsValue, layer: &JsValue) {
    ffi::reset_style(geo_layer, layer);
}

/// Fire a click event on a layer
pub fn fire_click(layer: &JsValue) {
    ffi::fire_event(layer, "click");
}

/// Add a layer to a group
pub fn add_layer_to_group(group: &JsValue, layer: &JsValue) {
    ffi::add_layer_to_group(group, layer);
}

/// Clear all layers from a group
pub fn clear_layers(group: &JsValue) {
    ffi::clear_layers(group);
}

/// Get bounds from a layer group
pub fn get_bounds(group: &JsValue) -> JsValue {
    ffi::get_bounds(group)
}

/// Fit map to bounds
pub fn fit_bounds(map: &JsValue, bounds: &LatLngBounds, max_zoom: Option<i32>) {
    let sw = ffi::create_lat_lng(bounds.south_west.lat, bounds.south_west.lng);
    let ne = ffi::create_lat_lng(bounds.north_east.lat, bounds.north_east.lng);
    let js_bounds = ffi::create_lat_lng_bounds(&sw, &ne);

    let opts = js_sys::Object::new();
    if let Some(z) = max_zoom {
        crate::ffi::set_prop(&opts, "maxZoom", JsValue::from_f64(z as f64));
    }

    ffi::fit_bounds(map, &js_bounds, &opts.into());
}

/// Remove a map instance
pub fn remove_map(map: &JsValue) {
    ffi::remove_map(map);
}

/// Invalidate map size
pub fn invalidate_size(map: &JsValue) {
    ffi::invalidate_size(map);
}

/// Add a layer to a map
pub fn add_layer_to_map(map: &JsValue, layer: &JsValue) {
    ffi::add_layer_to_map(map, layer);
}

/// Remove a layer from a map
pub fn remove_layer_from_map(map: &JsValue, layer: &JsValue) {
    ffi::remove_layer_from_map(map, layer);
}
