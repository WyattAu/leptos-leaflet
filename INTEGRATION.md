# Integration Guide: leptos-leaflet with SSR Personal Site

This guide explains how to integrate the `leptos-leaflet` library with the existing SSR personal site world map.

## Current State

The SSR personal site currently uses vanilla JavaScript with Leaflet.js for the world map. The implementation is in `site/pkg/world.js` (~63KB) and includes:

- Dynamic Leaflet loading from CDN
- Map initialization with CartoDB dark basemap
- Earthquake markers (circle markers with magnitude-based styling)
- Country boundaries (GeoJSON from TopoJSON)
- Capital weather markers (custom div icons)
- Layer toggles (quakes, borders, capitals)
- Country click handling with slide-out panel
- Map search with datalist
- Chart crosshair and price history

## Integration Strategy

### Phase 1: Core Map Component (Week 1)

Replace the vanilla JS map initialization with a Leptos `Map` component.

**Current code (world.js):**
```javascript
async function initMap() {
  await loadLeaflet();
  mapInstance = L.map("world-map", {
    center: [30, 0],
    zoom: 2,
    zoomControl: true,
    attributionControl: false,
    continuousWorld: true,
    minZoom: 2,
    maxZoom: 10
  });
  L.tileLayer("https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png", {
    maxZoom: 18,
    crossOrigin: true
  }).addTo(mapInstance);
}
```

**New code (Leptos):**
```rust
use leptos_leaflet::{Map, MapOptions, TileLayer, TileLayerOptions};

view! {
    <Map
        id="world-map"
        options=MapOptions {
            center: [30.0, 0.0],
            zoom: 2,
            min_zoom: Some(2),
            max_zoom: Some(10),
            zoom_control: true,
            attribution_control: false,
            continuous_world: true,
        }
    >
        <TileLayer
            url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
            options=TileLayerOptions {
                max_zoom: Some(18),
                cross_origin: true,
                ..Default::default()
            }
        />
    </Map>
}
```

### Phase 2: Layer Management (Week 2)

Replace the vanilla JS layer toggles with Leptos signals.

**Current code (world.js):**
```javascript
const _layerState = { quakes: true, borders: true, capitals: true };

function initLayerToggles() {
  document.querySelectorAll(".map-layer-btn").forEach(btn => {
    btn.addEventListener("click", () => {
      const layer = btn.dataset.layer;
      _layerState[layer] = !_layerState[layer];
      btn.classList.toggle("is-active", _layerState[layer]);
      if (layer === "quakes" && quakeLayer) {
        _layerState.quakes ? mapInstance.addLayer(quakeLayer) : mapInstance.removeLayer(quakeLayer);
      }
      // ... similar for borders and capitals
    });
  });
}
```

**New code (Leptos):**
```rust
use leptos_leaflet::{LayerToggle, LayerToggleGroup};

let (show_quakes, set_show_quakes) = signal(true);
let (show_borders, set_show_borders) = signal(true);
let (show_capitals, set_show_capitals) = signal(true);

view! {
    <LayerToggleGroup>
        <LayerToggle
            label="QUAKES"
            active=show_quakes.get()
            on_toggle=move |active| set_show_quakes.set(active)
        />
        <LayerToggle
            label="BORDERS"
            active=show_borders.get()
            on_toggle=move |active| set_show_borders.set(active)
        />
        <LayerToggle
            label="CAPITALS"
            active=show_capitals.get()
            on_toggle=move |active| set_show_capitals.set(active)
        />
    </LayerToggleGroup>
}
```

### Phase 3: Data Layers (Week 3)

Replace the vanilla JS data fetching with Leptos resources.

**Current code (world.js):**
```javascript
async function fetchEarthquakes() {
  const data = await fetch("https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/4.5_day.geojson");
  const geojson = await data.json();
  initQuakeMap(geojson);
}

function initQuakeMap(data) {
  quakeLayer.clearLayers();
  data.features.forEach(feature => {
    const [lng, lat] = feature.geometry.coordinates;
    const mag = feature.properties.mag;
    const color = mag >= 6 ? "#ff1744" : mag >= 5 ? "#ff6d00" : "#ffab00";
    L.circleMarker([lat, lng], {
      radius: Math.max(4, mag * 3),
      fillColor: color,
      color: color,
      weight: 1,
      opacity: 0.8,
      fillOpacity: 0.5
    }).bindPopup(popupHtml).addTo(quakeLayer);
  });
  mapInstance.fitBounds(quakeLayer.getBounds().pad(0.1), { maxZoom: 5 });
}
```

**New code (Leptos):**
```rust
use leptos_leaflet::{CircleMarker, CircleMarkerOptions, LatLng, LayerGroup};

let (earthquakes, set_earthquakes) = signal(Vec::<Earthquake>::new());

// Fetch earthquakes using Leptos resource
let earthquake_resource = Resource::new(
    move || (),
    |_| async move {
        fetch_earthquakes().await.unwrap_or_default()
    },
);

view! {
    <LayerGroup id="earthquakes">
        {move || earthquake_resource.get().map(|data| {
            data.into_iter().map(|eq| {
                let color = if eq.magnitude >= 6.0 { "#ff1744" }
                    else if eq.magnitude >= 5.0 { "#ff6d00" }
                    else { "#ffab00" };
                view! {
                    <CircleMarker
                        latlng=LatLng::new(eq.lat, eq.lng)
                        options=CircleMarkerOptions {
                            radius: (eq.magnitude * 3.0).max(4.0),
                            color: color.to_string(),
                            fill_color: color.to_string(),
                            fill_opacity: 0.5,
                            weight: 1.0,
                            opacity: 0.8,
                        }
                    />
                }
            }).collect::<Vec<_>>()
        })}
    </LayerGroup>
}
```

### Phase 4: Event Handling (Week 4)

Replace the vanilla JS event handlers with Leptos events.

**Current code (world.js):**
```javascript
function loadCountryBoundaries() {
  // ... load topojson ...
  countryLayer = L.geoJSON(geojson, {
    style: { fillColor: "transparent", fillOpacity: 0, color: "rgba(255,255,255,0.12)", weight: 0.5 },
    onEachFeature: (feature, layer) => {
      layer.on({
        click: () => {
          const iso = ISO_N2A[feature.id];
          onCountryClick(iso, feature.properties.name);
        },
        mouseover: () => {
          layer.setStyle({ fillOpacity: 0.08, color: "rgba(0,229,255,0.35)", weight: 1 });
        },
        mouseout: () => {
          countryLayer.resetStyle(layer);
        }
      });
    }
  }).addTo(mapInstance);
}
```

**New code (Leptos):**
```rust
use leptos_leaflet::{GeoJsonLayer, GeoJsonOptions};

let (selected_country, set_selected_country) = signal(Option::<String>::None);

view! {
    <GeoJsonLayer
        data=country_geojson
        options=GeoJsonOptions {
            fill_color: Some("transparent".to_string()),
            fill_opacity: Some(0.0),
            color: Some("rgba(255,255,255,0.12)".to_string()),
            weight: Some(0.5),
            interactive: true,
        }
        on_click=move |iso, name| {
            set_selected_country.set(Some(name));
        }
    />
}
```

## Migration Checklist

- [ ] Phase 1: Replace map initialization with `Map` component
- [ ] Phase 2: Replace layer toggles with `LayerToggle` components
- [ ] Phase 3: Replace earthquake/capital data with Leptos resources
- [ ] Phase 4: Replace event handlers with Leptos events
- [ ] Phase 5: Remove vanilla JS code from `world.js`
- [ ] Phase 6: Test all functionality
- [ ] Phase 7: Update documentation

## Estimated Effort

| Phase | Effort | Risk |
|-------|--------|------|
| Phase 1: Core Map | 2-3 days | Low |
| Phase 2: Layer Management | 1-2 days | Low |
| Phase 3: Data Layers | 3-4 days | Medium |
| Phase 4: Event Handling | 2-3 days | Medium |
| Phase 5: JS Removal | 1 day | Low |
| Phase 6: Testing | 2-3 days | Low |
| Phase 7: Documentation | 1 day | Low |
| **Total** | **12-16 days** | |

## Benefits of Migration

1. **Type Safety** -- Rust compiler catches errors at build time
2. **Reactive State** -- Leptos signals for automatic UI updates
3. **Reduced Bundle Size** -- Replace 63KB JS with typed Rust components
4. **Better Developer Experience** -- Declarative API instead of imperative JS
5. **Easier Maintenance** -- Single language (Rust) for all interactive logic

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Leaflet API compatibility | Test thoroughly with Leaflet 1.9.4 |
| WASM bundle size increase | Monitor bundle size, use code splitting if needed |
| Performance regression | Benchmark before/after, optimize critical paths |
| Browser compatibility | Test on Chrome, Firefox, Safari, Edge |

## Next Steps

1. Start with Phase 1 (Core Map Component)
2. Create a feature branch for the migration
3. Implement incrementally, testing each phase
4. Deploy to staging environment for testing
5. Merge to main after thorough testing
