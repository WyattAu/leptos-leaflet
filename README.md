# leptos-leaflet

Leptos components for [Leaflet.js](https://leafletjs.com/) maps. Provides a declarative, reactive API for building interactive maps in Rust/WASM with Leptos.

## Features

- **Declarative API** -- Build maps with Leptos components instead of imperative JS
- **Type-safe** -- Rust types for all Leaflet options and configuration
- **Reactive** -- Map state updates automatically when Leptos signals change
- **SSR-compatible** -- Renders map container during SSR, initializes Leaflet on client
- **Islands-ready** -- Works with Leptos islands for selective hydration
- **Full Leaflet feature support** -- Tile layers, markers, GeoJSON, layer groups, popups, tooltips

## Quick Start

### Cargo.toml

```toml
[dependencies]
leptos = "0.8"
leptos-leaflet = "0.1"
```

### Basic Map

```rust
use leptos::prelude::*;
use leptos_leaflet::{Map, MapOptions, TileLayer};

#[component]
fn App() -> impl IntoView {
    view! {
        <Map id="my-map" options=MapOptions {
            center: [51.5, -0.1],
            zoom: 13,
            ..Default::default()
        }>
            <TileLayer
                url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
            />
        </Map>
    }
}
```

## Components

### Map

The main map container. Wraps a Leaflet `L.map` instance.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `id` | `String` | required | HTML ID for the map container |
| `options` | `MapOptions` | `MapOptions::default()` | Map initialization options |
| `on_init` | `Option<Box<dyn Fn(Element)>>` | `None` | Callback when map is ready |
| `children` | `Option<Children>` | `None` | Child components (TileLayer, etc.) |

### MapOptions

```rust
MapOptions {
    center: [30.0, 0.0],      // [lat, lng]
    zoom: 2,                   // Initial zoom
    min_zoom: Some(2),         // Minimum zoom
    max_zoom: Some(10),        // Maximum zoom
    zoom_control: true,        // Show +/- buttons
    attribution_control: false, // Hide attribution
    continuous_world: true,    // Allow world wrapping
}
```

### TileLayer

Loads map tiles from a URL template.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `url` | `String` | required | Tile URL template with `{s}`, `{z}`, `{x}`, `{y}` placeholders |
| `options` | `TileLayerOptions` | `TileLayerOptions::default()` | Tile layer options |

```rust
<TileLayer
    url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
    options=TileLayerOptions {
        max_zoom: Some(18),
        cross_origin: true,
        ..Default::default()
    }
/>
```

### CircleMarker

A circle marker placed at a specific coordinate.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `latlng` | `LatLng` | required | Position [lat, lng] |
| `options` | `CircleMarkerOptions` | `CircleMarkerOptions::default()` | Marker options |

```rust
<CircleMarker
    latlng=LatLng::new(51.5, -0.1)
    options=CircleMarkerOptions {
        radius: 10.0,
        color: "#ff0000".to_string(),
        fill_color: "#ff0000".to_string(),
        fill_opacity: 0.5,
        ..Default::default()
    }
/>
```

### GeoJsonLayer

Renders GeoJSON data on the map.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `data` | `String` | required | GeoJSON string |
| `options` | `GeoJsonOptions` | `GeoJsonOptions::default()` | GeoJSON layer options |

### LayerGroup

A container for multiple markers or layers.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `id` | `String` | required | Unique identifier for the group |
| `children` | `Option<Children>` | `None` | Child markers |

### DivIcon

A custom HTML icon for markers.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `html` | `String` | required | HTML content |
| `class_name` | `String` | `""` | CSS class |
| `icon_size` | `[i32; 2]` | `[30, 14]` | Size [w, h] |
| `icon_anchor` | `[i32; 2]` | `[15, 7]` | Anchor point [x, y] |

## Types

### LatLng

```rust
LatLng::new(51.5074, -0.1278) // London
```

### LatLngBounds

```rust
LatLngBounds::new(
    LatLng::new(51.0, -0.5),
    LatLng::new(52.0, 0.5),
)
```

## Advanced Usage

### Dynamic Markers with Signals

```rust
use leptos::prelude::*;
use leptos_leaflet::{Map, CircleMarker, CircleMarkerOptions, LatLng};

#[component]
fn DynamicMap() -> impl IntoView {
    let (markers, set_markers) = signal(vec![
        LatLng::new(51.5, -0.1),
        LatLng::new(48.8, 2.3),
        LatLng::new(52.5, 13.4),
    ]);

    view! {
        <Map id="dynamic-map">
            <For
                each=move || markers.get()
                key=|m| format!("{},{}", m.lat, m.lng)
                children=move |latlng| view! {
                    <CircleMarker
                        latlng=latlng
                        options=CircleMarkerOptions {
                            radius: 8.0,
                            color: "#00e5ff".to_string(),
                            fill_color: "#00e5ff".to_string(),
                            ..Default::default()
                        }
                    />
                }
            />
        </Map>
    }
}
```

### Layer Toggle

```rust
use leptos::prelude::*;
use leptos_leaflet::{Map, TileLayer, LayerGroup};

#[component]
fn MapWithToggles() -> impl IntoView {
    let (show_markers, set_show_markers) = signal(true);

    view! {
        <Map id="toggle-map">
            <TileLayer url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"/>
            {move || if show_markers.get() {
                view! {
                    <LayerGroup id="markers">
                        // Markers here
                    </LayerGroup>
                }.into_any()
            } else {
                ().into_any()
            }}
        </Map>
        <button on:click=move |_| set_show_markers.update(|v| *v = !*v)>
            {move || if show_markers.get() { "Hide Markers" } else { "Show Markers" }}
        </button>
    }
}
```

### GeoJSON with Event Handling

```rust
use leptos::prelude::*;
use leptos_leaflet::{Map, GeoJsonLayer, GeoJsonOptions};

#[component]
fn GeoMap() -> impl IntoView {
    let geojson_data = r#"{
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "geometry": {
                "type": "Polygon",
                "coordinates": [[[0,0],[1,0],[1,1],[0,1],[0,0]]]
            },
            "properties": {"name": "Test"}
        }]
    }"#.to_string();

    view! {
        <Map id="geo-map">
            <TileLayer url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"/>
            <GeoJsonLayer data=geojson_data options=GeoJsonOptions::default()/>
        </Map>
    }
}
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `csr` | Yes | Client-side rendering support |
| `ssr` | No | Server-side rendering support |
| `hydrate` | No | Hydration support |
| `islands` | No | Islands mode support |

## Requirements

- Rust 1.70+
- Leptos 0.8+
- WASM target (wasm32-unknown-unknown)

## License

MIT
