use leptos_leaflet_wyatt::types::*;

#[test]
fn test_map_options_default() {
    let opts = MapOptions::default();
    assert_eq!(opts.center, [30.0, 0.0]);
    assert_eq!(opts.zoom, 2);
    assert_eq!(opts.min_zoom, Some(2));
    assert_eq!(opts.max_zoom, Some(10));
    assert!(opts.zoom_control);
    assert!(!opts.attribution_control);
    assert!(opts.continuous_world);
}

#[test]
fn test_tile_layer_options_default() {
    let opts = TileLayerOptions::default();
    assert_eq!(opts.max_zoom, Some(18));
    assert!(opts.cross_origin);
    assert!(opts.subdomains.is_none());
    assert!(opts.attribution.is_none());
}

#[test]
fn test_circle_marker_options_default() {
    let opts = CircleMarkerOptions::default();
    assert_eq!(opts.radius, 10.0);
    assert_eq!(opts.color, "#ff0000");
    assert_eq!(opts.fill_color, "#ff0000");
    assert_eq!(opts.weight, 1.0);
    assert_eq!(opts.opacity, 0.8);
    assert_eq!(opts.fill_opacity, 0.5);
}

#[test]
fn test_geo_json_options_default() {
    let opts = GeoJsonOptions::default();
    assert_eq!(opts.fill_color, Some("transparent".to_string()));
    assert_eq!(opts.fill_opacity, Some(0.0));
    assert!(opts.interactive);
}

#[test]
fn test_lat_lng() {
    let ll = LatLng::new(51.5, -0.1);
    assert_eq!(ll.lat, 51.5);
    assert_eq!(ll.lng, -0.1);
}

#[test]
fn test_lat_lng_bounds() {
    let sw = LatLng::new(51.0, -0.5);
    let ne = LatLng::new(52.0, 0.5);
    let bounds = LatLngBounds::new(sw, ne);
    assert_eq!(bounds.south_west.lat, 51.0);
    assert_eq!(bounds.north_east.lat, 52.0);
}

#[test]
fn test_layer_state() {
    let state = LayerState::default();
    assert!(state.quakes);
    assert!(state.borders);
    assert!(state.capitals);
}
