use leptos_leaflet::types::*;
use leptos_leaflet::state::*;

#[test]
fn test_map_options_custom() {
    let opts = MapOptions {
        center: [51.5, -0.1],
        zoom: 13,
        min_zoom: Some(1),
        max_zoom: Some(18),
        zoom_control: false,
        attribution_control: true,
        continuous_world: false,
    };
    assert_eq!(opts.center, [51.5, -0.1]);
    assert_eq!(opts.zoom, 13);
    assert_eq!(opts.min_zoom, Some(1));
    assert_eq!(opts.max_zoom, Some(18));
    assert!(!opts.zoom_control);
    assert!(opts.attribution_control);
    assert!(!opts.continuous_world);
}

#[test]
fn test_tile_layer_options_custom() {
    let opts = TileLayerOptions {
        max_zoom: Some(19),
        cross_origin: false,
        subdomains: Some(vec!["a".to_string(), "b".to_string()]),
        attribution: Some("Test attribution".to_string()),
    };
    assert_eq!(opts.max_zoom, Some(19));
    assert!(!opts.cross_origin);
    assert_eq!(opts.subdomains, Some(vec!["a".to_string(), "b".to_string()]));
    assert_eq!(opts.attribution, Some("Test attribution".to_string()));
}

#[test]
fn test_circle_marker_options_custom() {
    let opts = CircleMarkerOptions {
        radius: 15.0,
        color: "#00ff00".to_string(),
        fill_color: "#00ff00".to_string(),
        weight: 2.0,
        opacity: 0.9,
        fill_opacity: 0.7,
    };
    assert_eq!(opts.radius, 15.0);
    assert_eq!(opts.color, "#00ff00");
    assert_eq!(opts.weight, 2.0);
    assert_eq!(opts.opacity, 0.9);
    assert_eq!(opts.fill_opacity, 0.7);
}

#[test]
fn test_geo_json_options_custom() {
    let opts = GeoJsonOptions {
        fill_color: Some("#ff0000".to_string()),
        fill_opacity: Some(0.3),
        color: Some("#000000".to_string()),
        weight: Some(2.0),
        interactive: false,
    };
    assert_eq!(opts.fill_color, Some("#ff0000".to_string()));
    assert_eq!(opts.fill_opacity, Some(0.3));
    assert_eq!(opts.color, Some("#000000".to_string()));
    assert_eq!(opts.weight, Some(2.0));
    assert!(!opts.interactive);
}

#[test]
fn test_lat_lng_custom() {
    let ll = LatLng::new(-33.8688, 151.2093);
    assert_eq!(ll.lat, -33.8688);
    assert_eq!(ll.lng, 151.2093);
}

#[test]
fn test_lat_lng_bounds_custom() {
    let sw = LatLng::new(-34.0, 151.0);
    let ne = LatLng::new(-33.0, 152.0);
    let bounds = LatLngBounds::new(sw, ne);
    assert_eq!(bounds.south_west.lat, -34.0);
    assert_eq!(bounds.north_east.lng, 152.0);
}

#[test]
fn test_layer_state_custom() {
    let mut state = LayerState::default();
    assert!(state.quakes);
    assert!(state.borders);
    assert!(state.capitals);

    state.quakes = false;
    state.borders = false;
    assert!(!state.quakes);
    assert!(!state.borders);
    assert!(state.capitals);
}

#[test]
fn test_fit_bounds_options_default() {
    let opts = FitBoundsOptions::default();
    assert!(opts.padding.is_none());
    assert!(opts.max_zoom.is_none());
    assert!(opts.animate);
}

#[test]
fn test_popup_options_default() {
    let opts = PopupOptions::default();
    assert!(opts.max_width.is_none());
    assert!(opts.min_width.is_none());
    assert!(opts.offset.is_none());
    assert!(opts.auto_pan);
}

#[test]
fn test_div_icon_options_default() {
    let opts = DivIconOptions::default();
    assert!(opts.class_name.is_empty());
    assert!(opts.html.is_empty());
    assert_eq!(opts.icon_size, [30, 14]);
    assert_eq!(opts.icon_anchor, [15, 7]);
}
