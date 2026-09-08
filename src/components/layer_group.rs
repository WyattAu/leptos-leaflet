use leptos::prelude::*;


/// A layer group that contains multiple markers or layers.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet_wyatt::LayerGroup;
///
/// let _ = view! {
///     <LayerGroup id="earthquakes">
///         // Child markers will be added to this group
///     </LayerGroup>
/// };
/// ```
#[component]
pub fn LayerGroup(
    /// Unique identifier for this layer group.
    #[prop(into)] id: String,
    /// Child markers to include in this group.
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-leaflet-layer-group="true"
            data-layer-id=id
            style="display:none"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// A div icon for custom HTML markers
#[component]
pub fn DivIcon(
    /// HTML content for the icon.
    #[prop(into)] html: String,
    /// CSS class name for the icon.
    #[prop(optional)] class_name: String,
    /// Icon size `[width, height]` in pixels.
    #[prop(optional)] icon_size: Option<[i32; 2]>,
    /// Icon anchor point `[x, y]` in pixels.
    #[prop(optional)] icon_anchor: Option<[i32; 2]>,
) -> impl IntoView {
    let size = icon_size.unwrap_or([30, 14]);
    let anchor = icon_anchor.unwrap_or([15, 7]);

    view! {
        <div
            data-leaflet-div-icon="true"
            data-html=html
            data-class-name=class_name
            data-icon-size=format!("{},{}", size[0], size[1])
            data-icon-anchor=format!("{},{}", anchor[0], anchor[1])
            style="display:none"
        ></div>
    }
}
