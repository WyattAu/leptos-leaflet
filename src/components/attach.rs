//! Materialize declarative layer stubs into real Leaflet layers.
//!
//! Components like [`crate::components::CircleMarker`] and
//! [`crate::components::TileLayer`] render hidden `<div data-leaflet-*>`
//! stubs so the Leptos view tree stays declarative. This module scans the
//! map container for those stubs and creates the corresponding Leaflet
//! objects via the FFI layer. A MutationObserver keeps subsequent renders
//! (e.g. conditional layer groups) in sync without manual calls.

use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{Element, MutationObserver, MutationObserverInit, Node};

use crate::ffi;

const STASH_KEY: &str = "__wyatt_leaflet_layer";
const PARENT_KEY: &str = "__wyatt_leaflet_parent";

const STUB_SELECTOR: &str = "[data-leaflet-tile-layer],[data-leaflet-layer-group],\
[data-leaflet-circle-marker],[data-leaflet-geo-json]";

fn stash(el: &Element, key: &str, value: &JsValue) {
    let _ = Reflect::set(el, &key.into(), value);
}

fn stash_get(el: &Element, key: &str) -> Option<JsValue> {
    Reflect::get(el, &key.into())
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())
}

/// Attach every stub under `container` and watch for future additions.
pub(crate) fn attach_and_watch(map: &JsValue, container: &Element) {
    attach_tree(map, container);

    let map_clone = map.clone();
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |records: js_sys::Array| {
        let process = |nodes: &web_sys::NodeList, map: &JsValue, add: bool| {
            for i in 0..nodes.length() {
                let Some(node) = nodes.get(i) else { continue };
                if let Some(el) = node.dyn_ref::<Element>() {
                    if add {
                        attach_tree(map, el);
                    } else {
                        detach(map, el);
                    }
                }
            }
        };
        for record in records.iter() {
            let record: web_sys::MutationRecord = match record.dyn_into() {
                Ok(r) => r,
                Err(_) => continue,
            };
            process(&record.added_nodes(), &map_clone, true);
            process(&record.removed_nodes(), &map_clone, false);
        }
    });

    if let Ok(observer) = MutationObserver::new(callback.as_ref().unchecked_ref()) {
        let init = MutationObserverInit::new();
        init.set_child_list(true);
        init.set_subtree(true);
        let _ = observer.observe_with_options(container, &init);
        callback.forget();
    }
}

fn attach_tree(map: &JsValue, root: &Node) {
    let Some(root_el) = root.dyn_ref::<Element>() else {
        return;
    };

    if root_el.has_attribute("data-leaflet-tile-layer")
        || root_el.has_attribute("data-leaflet-layer-group")
        || root_el.has_attribute("data-leaflet-circle-marker")
        || root_el.has_attribute("data-leaflet-geo-json")
    {
        attach_node(map, root_el);
    }

    if let Ok(list) = root_el.query_selector_all(STUB_SELECTOR) {
        for i in 0..list.length() {
            if let Some(node) = list.get(i) {
                if let Some(el) = node.dyn_ref::<Element>() {
                    attach_node(map, el);
                }
            }
        }
    }
}

fn attach_node(map: &JsValue, el: &Element) {
    if stash_get(el, STASH_KEY).is_some() {
        return;
    }

    if el.has_attribute("data-leaflet-tile-layer") {
        let url = el
            .get_attribute("data-leaflet-tile-layer")
            .unwrap_or_default();
        if url.is_empty() {
            return;
        }
        let layer = ffi::create_tile_layer(&url, &js_sys::Object::new().into());
        ffi::add_layer_to_map(map, &layer);
        stash(el, STASH_KEY, &layer);
    } else if el.has_attribute("data-leaflet-layer-group") {
        let group = ffi::create_layer_group();
        ffi::add_layer_to_map(map, &group);
        stash(el, STASH_KEY, &group);
        attach_markers_in(map, el, &group);
    } else if el.has_attribute("data-leaflet-circle-marker") {
        let parent_group = el
            .closest("[data-leaflet-layer-group]")
            .ok()
            .flatten()
            .and_then(|group_el| stash_get(&group_el, STASH_KEY));
        match parent_group {
            Some(group) => attach_marker(map, el, &group),
            None => attach_marker(map, el, map),
        }
    } else if el.has_attribute("data-leaflet-geo-json") {
        let data = el.get_attribute("data-geo-data").unwrap_or_default();
        let Ok(parsed) = js_sys::JSON::parse(&data) else {
            return;
        };
        let layer = ffi::create_geo_json(&parsed, &js_sys::Object::new().into());
        ffi::add_layer_to_map(map, &layer);
        stash(el, STASH_KEY, &layer);
    }
}

fn attach_markers_in(map: &JsValue, group_el: &Element, group: &JsValue) {
    let Ok(list) = group_el.query_selector_all("[data-leaflet-circle-marker]") else {
        return;
    };
    for i in 0..list.length() {
        if let Some(node) = list.get(i) {
            if let Some(el) = node.dyn_ref::<Element>() {
                attach_marker(map, el, group);
            }
        }
    }
}

fn attach_marker(map: &JsValue, el: &Element, target: &JsValue) {
    if stash_get(el, STASH_KEY).is_some() {
        return;
    }

    let (Some(lat), Some(lng)) = (
        el.get_attribute("data-lat")
            .and_then(|v| v.parse::<f64>().ok()),
        el.get_attribute("data-lng")
            .and_then(|v| v.parse::<f64>().ok()),
    ) else {
        return;
    };

    let opts = js_sys::Object::new();
    let set = |key: &str, value: f64| {
        ffi::set_prop(&opts, key, JsValue::from_f64(value));
    };
    if let Some(v) = attr_f64(el, "data-radius") {
        set("radius", v);
    }
    if let Some(v) = el.get_attribute("data-color") {
        ffi::set_prop(&opts, "color", v.into());
    }
    if let Some(v) = el.get_attribute("data-fill-color") {
        ffi::set_prop(&opts, "fillColor", v.into());
    }
    if let Some(v) = attr_f64(el, "data-fill-opacity") {
        set("fillOpacity", v);
    }
    if let Some(v) = attr_f64(el, "data-weight") {
        set("weight", v);
    }
    if let Some(v) = attr_f64(el, "data-opacity") {
        set("opacity", v);
    }

    let latlng = ffi::create_lat_lng(lat, lng);
    let marker = ffi::create_circle_marker(&latlng, &opts.into());

    if std::ptr::eq(target, map) {
        ffi::add_layer_to_map(map, &marker);
    } else {
        ffi::add_layer_to_group(target, &marker);
        stash(el, PARENT_KEY, target);
    }
    stash(el, STASH_KEY, &marker);
}

fn detach(map: &JsValue, el: &Element) {
    let Some(layer) = stash_get(el, STASH_KEY) else {
        return;
    };
    match stash_get(el, PARENT_KEY) {
        Some(parent) => {
            let _ = ffi::call_method(&parent, "removeLayer", &[layer]);
        }
        None => ffi::remove_layer_from_map(map, &layer),
    }
}

fn attr_f64(el: &Element, name: &str) -> Option<f64> {
    el.get_attribute(name).and_then(|v| v.parse::<f64>().ok())
}
