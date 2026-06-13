use serde::{Deserialize, Serialize};

/// Map initialization options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapOptions {
    /// Map center [lat, lng]
    pub center: [f64; 2],
    /// Initial zoom level
    pub zoom: i32,
    /// Minimum zoom level
    pub min_zoom: Option<i32>,
    /// Maximum zoom level
    pub max_zoom: Option<i32>,
    /// Show zoom controls
    pub zoom_control: bool,
    /// Show attribution control
    pub attribution_control: bool,
    /// Enable continuous world scrolling
    pub continuous_world: bool,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            center: [30.0, 0.0],
            zoom: 2,
            min_zoom: Some(2),
            max_zoom: Some(10),
            zoom_control: true,
            attribution_control: false,
            continuous_world: true,
        }
    }
}

/// Tile layer options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileLayerOptions {
    /// Maximum zoom level for tiles
    pub max_zoom: Option<i32>,
    /// Cross-origin setting for tiles
    pub cross_origin: bool,
    /// Subdomains for tile URLs
    pub subdomains: Option<Vec<String>>,
    /// Attribution text
    pub attribution: Option<String>,
}

impl Default for TileLayerOptions {
    fn default() -> Self {
        Self {
            max_zoom: Some(18),
            cross_origin: true,
            subdomains: None,
            attribution: None,
        }
    }
}

/// Circle marker options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircleMarkerOptions {
    /// Radius in pixels
    pub radius: f64,
    /// Stroke color (CSS color string)
    pub color: String,
    /// Fill color (CSS color string)
    pub fill_color: String,
    /// Stroke width in pixels
    pub weight: f64,
    /// Stroke opacity (0-1)
    pub opacity: f64,
    /// Fill opacity (0-1)
    pub fill_opacity: f64,
}

impl Default for CircleMarkerOptions {
    fn default() -> Self {
        Self {
            radius: 10.0,
            color: "#ff0000".to_string(),
            fill_color: "#ff0000".to_string(),
            weight: 1.0,
            opacity: 0.8,
            fill_opacity: 0.5,
        }
    }
}

/// GeoJSON layer options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeoJsonOptions {
    /// Default fill color
    pub fill_color: Option<String>,
    /// Default fill opacity
    pub fill_opacity: Option<f64>,
    /// Default stroke color
    pub color: Option<String>,
    /// Default stroke width
    pub weight: Option<f64>,
    /// Whether features are interactive
    pub interactive: bool,
}

impl Default for GeoJsonOptions {
    fn default() -> Self {
        Self {
            fill_color: Some("transparent".to_string()),
            fill_opacity: Some(0.0),
            color: Some("rgba(255,255,255,0.12)".to_string()),
            weight: Some(0.5),
            interactive: true,
        }
    }
}

/// Div icon options for custom HTML markers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DivIconOptions {
    /// CSS class name
    pub class_name: String,
    /// HTML content
    pub html: String,
    /// Icon size [width, height] in pixels
    pub icon_size: [i32; 2],
    /// Icon anchor point [x, y] in pixels
    pub icon_anchor: [i32; 2],
}

impl Default for DivIconOptions {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            html: String::new(),
            icon_size: [30, 14],
            icon_anchor: [15, 7],
        }
    }
}

/// Popup options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopupOptions {
    /// Maximum width in pixels
    pub max_width: Option<i32>,
    /// Minimum width in pixels
    pub min_width: Option<i32>,
    /// Offset from the marker
    pub offset: Option<[i32; 2]>,
    /// Whether to auto-pan the map
    pub auto_pan: bool,
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self {
            max_width: None,
            min_width: None,
            offset: None,
            auto_pan: true,
        }
    }
}

/// Fit bounds options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FitBoundsOptions {
    /// Padding in pixels [top, right, bottom, left]
    pub padding: Option<[i32; 2]>,
    /// Maximum zoom level
    pub max_zoom: Option<i32>,
    /// Whether to animate the transition
    pub animate: bool,
}

impl Default for FitBoundsOptions {
    fn default() -> Self {
        Self {
            padding: None,
            max_zoom: None,
            animate: true,
        }
    }
}

/// Layer state for toggling visibility
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LayerState {
    pub quakes: bool,
    pub borders: bool,
    pub capitals: bool,
}

/// A geographic coordinate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

/// Bounding box
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LatLngBounds {
    pub south_west: LatLng,
    pub north_east: LatLng,
}

impl LatLngBounds {
    pub fn new(south_west: LatLng, north_east: LatLng) -> Self {
        Self {
            south_west,
            north_east,
        }
    }
}
