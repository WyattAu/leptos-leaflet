use leptos::prelude::*;

use crate::types::PopupOptions;

/// A popup that displays content when a marker is clicked.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::{Popup, PopupOptions};
///
/// let _ = view! {
///     <Popup options=PopupOptions::default()>
///         <div class="popup-content">
///             <h3>"London"</h3>
///             <p>"Population: 8.9M"</p>
///         </div>
///     </Popup>
/// };
/// ```
#[component]
pub fn Popup(
    /// Popup display options.
    #[prop(optional)] options: Option<PopupOptions>,
    /// Content to display in the popup.
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let _options = options.unwrap_or_default();

    view! {
        <div
            data-leaflet-popup="true"
            style="display:none"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// A tooltip that displays content on hover.
///
/// # Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::Tooltip;
///
/// let _ = view! {
///     <Tooltip content="London, UK" />
/// };
/// ```
#[component]
pub fn Tooltip(
    /// Tooltip text content.
    #[prop(into)] content: String,
    /// Whether the tooltip follows the mouse.
    #[prop(optional)] sticky: Option<bool>,
) -> impl IntoView {
    let _sticky = sticky.unwrap_or(false);

    view! {
        <div
            data-leaflet-tooltip="true"
            data-tooltip-content=content
            style="display:none"
        ></div>
    }
}
