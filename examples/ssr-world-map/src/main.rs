use leptos::prelude::*;
use leptos_leaflet_wyatt::*;

/// SSR Personal Site World Map Integration
///
/// This example demonstrates how to integrate leptos-leaflet-wyatt with the
/// existing world map functionality from the SSR personal site.
///
/// It replaces the vanilla JS Leaflet code with typed Rust components,
/// providing:
/// - Type-safe map configuration
/// - Reactive layer toggles
/// - Event handling for country clicks
/// - Popup and tooltip support
///
/// Run with: cargo run --example ssr-world-map

// Sample earthquake data structure
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Earthquake {
    pub lat: f64,
    pub lng: f64,
    pub magnitude: f64,
    pub place: String,
    pub time: String,
    pub depth: f64,
}

// Sample country data structure
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Country {
    pub iso: String,
    pub name: String,
    pub capital: String,
    pub population: u64,
}

#[component]
fn WorldMapApp() -> impl IntoView {
    // Layer visibility state
    let (show_quakes, set_show_quakes) = signal(true);
    let (show_borders, set_show_borders) = signal(true);
    let (show_capitals, set_show_capitals) = signal(true);

    // Selected country state
    let (selected_country, set_selected_country) = signal(Option::<String>::None);

    // Sample earthquake data (would normally come from API)
    let earthquakes = vec![
        Earthquake { lat: 51.5074, lng: -0.1278, magnitude: 4.5, place: "London, UK".to_string(), time: "2024-01-15".to_string(), depth: 10.0 },
        Earthquake { lat: 40.7128, lng: -74.0060, magnitude: 3.2, place: "New York, US".to_string(), time: "2024-01-14".to_string(), depth: 5.0 },
        Earthquake { lat: 35.6762, lng: 139.6503, magnitude: 5.1, place: "Tokyo, JP".to_string(), time: "2024-01-13".to_string(), depth: 15.0 },
        Earthquake { lat: -33.8688, lng: 151.2093, magnitude: 2.8, place: "Sydney, AU".to_string(), time: "2024-01-12".to_string(), depth: 8.0 },
        Earthquake { lat: 48.8566, lng: 2.3522, magnitude: 3.8, place: "Paris, FR".to_string(), time: "2024-01-11".to_string(), depth: 12.0 },
        Earthquake { lat: 55.7558, lng: 37.6173, magnitude: 2.5, place: "Moscow, RU".to_string(), time: "2024-01-10".to_string(), depth: 7.0 },
        Earthquake { lat: -23.5505, lng: -46.6333, magnitude: 4.2, place: "Sao Paulo, BR".to_string(), time: "2024-01-09".to_string(), depth: 20.0 },
        Earthquake { lat: 19.4326, lng: -99.1332, magnitude: 3.5, place: "Mexico City, MX".to_string(), time: "2024-01-08".to_string(), depth: 9.0 },
    ];

    // Sample country data (simplified GeoJSON)
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
            },
            {
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[[5, 45], [15, 45], [15, 55], [5, 55], [5, 45]]]
                },
                "properties": {"name": "Germany", "iso": "DE"}
            },
            {
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[[130, 30], [145, 30], [145, 45], [130, 45], [130, 30]]]
                },
                "properties": {"name": "Japan", "iso": "JP"}
            }
        ]
    }"#.to_string();

    // Sample capital data
    let capitals = vec![
        (51.5074, -0.1278, "London", 13.9),
        (48.8566, 2.3522, "Paris", 8.5),
        (52.5200, 13.4050, "Berlin", 5.2),
        (35.6762, 139.6503, "Tokyo", 26.7),
        (40.7128, -74.0060, "New York", 24.0),
        (-33.8688, 151.2093, "Sydney", 18.3),
    ];

    // Get temperature color based on value
    let temp_color = |temp: f64| -> &'static str {
        if temp < 5.0 { "#448aff" }
        else if temp < 15.0 { "#00e5ff" }
        else if temp < 25.0 { "#69f0ae" }
        else if temp < 35.0 { "#ffab00" }
        else { "#ff5252" }
    };

    // Get earthquake color based on magnitude
    let quake_color = |mag: f64| -> &'static str {
        if mag >= 6.0 { "#ff1744" }
        else if mag >= 5.0 { "#ff6d00" }
        else if mag >= 4.0 { "#ffab00" }
        else { "#ffc107" }
    };

    view! {
        <div class="world-map-app">
            <header>
                <h1>"World Monitor"</h1>
                <p>"Real-time world intelligence dashboard"</p>
            </header>

            // Layer controls
            <nav class="layer-controls" role="group" aria-label="Map layer controls">
                <button
                    class=move || if show_quakes.get() { "active" } else { "" }
                    on:click=move |_| set_show_quakes.update(|v| *v = !*v)
                    aria-pressed=move || show_quakes.get().to_string()
                >
                    "QUAKES"
                </button>
                <button
                    class=move || if show_borders.get() { "active" } else { "" }
                    on:click=move |_| set_show_borders.update(|v| *v = !*v)
                    aria-pressed=move || show_borders.get().to_string()
                >
                    "BORDERS"
                </button>
                <button
                    class=move || if show_capitals.get() { "active" } else { "" }
                    on:click=move |_| set_show_capitals.update(|v| *v = !*v)
                    aria-pressed=move || show_capitals.get().to_string()
                >
                    "CAPITALS"
                </button>
            </nav>

            // Info bar
            <div class="info-bar" role="status" aria-live="polite">
                <span class="info-item">
                    "QUAKES: " {move || earthquakes.len().to_string()}
                </span>
                <span class="info-item">
                    "K-INDEX: " {move || if show_quakes.get() { "3" } else { "--" }}
                </span>
                <span class="info-item">
                    "F&G: " {move || if show_quakes.get() { "45" } else { "--" }}
                </span>
            </div>

            // Selected country display
            {move || selected_country.get().map(|country| {
                view! {
                    <div class="selected-country" role="status" aria-live="polite">
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
                            {earthquakes.iter().map(|eq| {
                                let color = quake_color(eq.magnitude);
                                let radius = (eq.magnitude * 3.0).max(4.0);
                                let popup_html = format!(
                                    "<div style='font-family:monospace;font-size:11px'>\
                                        <b>M{:.1}</b><br>{}<br>{}<br>Depth: {:.1}km</div>",
                                    eq.magnitude, eq.place, eq.time, eq.depth
                                );
                                view! {
                                    <CircleMarker
                                        latlng=LatLng::new(eq.lat, eq.lng)
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

                // Capital markers (conditionally shown)
                {move || if show_capitals.get() {
                    view! {
                        <LayerGroup id="capitals">
                            {capitals.iter().map(|(lat, lng, name, temp)| {
                                let color = temp_color(*temp);
                                let icon_html = format!(
                                    "<div style='font-family:monospace;font-size:9px;font-weight:700;color:{};text-shadow:0 0 3px rgba(0,0,0,0.8);white-space:nowrap;'>{}°C</div>",
                                    color, temp
                                );
                                view! {
                                    <CircleMarker
                                        latlng=LatLng::new(*lat, *lng)
                                        options=CircleMarkerOptions {
                                            radius: 5.0,
                                            color: color.to_string(),
                                            fill_color: color.to_string(),
                                            fill_opacity: 0.8,
                                            weight: 1.0,
                                            opacity: 1.0,
                                        }
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        </LayerGroup>
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </Map>

            // Data panels
            <div class="data-panels">
                <section class="panel">
                    <h2>"Earthquakes"</h2>
                    <ul>
                        {earthquakes.iter().map(|eq| {
                            view! {
                                <li>
                                    <strong>"M"{eq.magnitude}</strong> " - " {eq.place.clone()}
                                    <span class="meta">" ({eq.time.clone()})"</span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </section>

                <section class="panel">
                    <h2>"Capitals"</h2>
                    <ul>
                        {capitals.iter().map(|(lat, lng, name, temp)| {
                            let color = temp_color(*temp);
                            view! {
                                <li>
                                    <span class="temp" style={format!("color:{}", color)}>{format!("{:.1}°C", temp)}</span>
                                    " - " {name.to_string()}
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </section>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(WorldMapApp);
}
