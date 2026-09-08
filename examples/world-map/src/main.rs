use leptos::prelude::*;
use leptos_leaflet_wyatt::*;

/// Example: World Map with Earthquakes and Country Boundaries
///
/// This example demonstrates:
/// - Map initialization with custom options
/// - Tile layer loading
/// - Circle markers for earthquakes
/// - GeoJSON layer for country boundaries
/// - Layer groups for organizing markers
/// - Popups and tooltips
/// - Interactive layer toggles
/// - Country click handling
///
/// Run with: cargo run --example world-map
#[component]
fn App() -> impl IntoView {
    let (show_quakes, set_show_quakes) = signal(true);
    let (show_borders, set_show_borders) = signal(true);
    let (show_capitals, set_show_capitals) = signal(true);
    let (selected_country, set_selected_country) = signal(Option::<String>::None);

    // Sample earthquake data
    let earthquakes = vec![
        (51.5074, -0.1278, 4.5, "London, UK"),
        (40.7128, -74.0060, 3.2, "New York, US"),
        (35.6762, 139.6503, 5.1, "Tokyo, JP"),
        (-33.8688, 151.2093, 2.8, "Sydney, AU"),
        (48.8566, 2.3522, 3.8, "Paris, FR"),
    ];

    // Sample country boundaries (simplified)
    let country_geojson = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[[-10, 50], [2, 50], [2, 60], [-10, 60], [-10, 50]]]
                },
                "properties": {"name": "United Kingdom", "iso": "GB"}
            },
            {
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[[-5, 42], [10, 42], [10, 52], [-5, 52], [-5, 42]]]
                },
                "properties": {"name": "France", "iso": "FR"}
            }
        ]
    }"#.to_string();

    view! {
        <div class="app">
            <h1>"World Map Example"</h1>

            // Map controls
            <div class="controls">
                <button on:click=move |_| set_show_quakes.update(|v| *v = !*v)>
                    {move || if show_quakes.get() { "Hide Earthquakes" } else { "Show Earthquakes" }}
                </button>
                <button on:click=move |_| set_show_borders.update(|v| *v = !*v)>
                    {move || if show_borders.get() { "Hide Borders" } else { "Show Borders" }}
                </button>
                <button on:click=move |_| set_show_capitals.update(|v| *v = !*v)>
                    {move || if show_capitals.get() { "Hide Capitals" } else { "Show Capitals" }}
                </button>
            </div>

            // Selected country display
            {move || selected_country.get().map(|country| {
                view! {
                    <div class="selected-country">
                        "Selected: " {country}
                    </div>
                }
            })}

            // The map
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
                // Tile layer
                <TileLayer
                    url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
                    options=TileLayerOptions {
                        max_zoom: Some(18),
                        cross_origin: true,
                        ..Default::default()
                    }
                />

                // Earthquake markers (conditionally shown)
                {move || if show_quakes.get() {
                    view! {
                        <LayerGroup id="earthquakes">
                            {earthquakes.iter().map(|(lat, lng, mag, name)| {
                                let color = if *mag >= 5.0 {
                                    "#ff1744"
                                } else if *mag >= 4.0 {
                                    "#ff6d00"
                                } else {
                                    "#ffab00"
                                };
                                let radius: f64 = (*mag * 3.0f64).max(4.0);
                                view! {
                                    <CircleMarker
                                        latlng=LatLng::new(*lat, *lng)
                                        options=CircleMarkerOptions {
                                            radius,
                                            color: color.to_string(),
                                            fill_color: color.to_string(),
                                            fill_opacity: 0.5,
                                            weight: 1.0,
                                            opacity: 0.8,
                                        }
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        </LayerGroup>
                    }.into_any()
                } else {
                    ().into_any()
                }}

                // Country boundaries (conditionally shown)
                {move || if show_borders.get() {
                    view! {
                        <GeoJsonLayer
                            data=country_geojson.clone()
                            options=GeoJsonOptions {
                                fill_color: Some("transparent".to_string()),
                                fill_opacity: Some(0.0),
                                color: Some("rgba(255,255,255,0.12)".to_string()),
                                weight: Some(0.5),
                                interactive: true,
                            }
                        />
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </Map>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
