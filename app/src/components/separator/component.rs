use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    #[prop(optional)] orientation: Option<SeparatorOrientation>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or(SeparatorOrientation::Horizontal);
    let orientation_class = match orientation {
        SeparatorOrientation::Horizontal => "separator-horizontal",
        SeparatorOrientation::Vertical => "separator-vertical",
    };

    let classes = format!(
        "separator {} {}",
        orientation_class,
        class.unwrap_or("")
    );

    view! {
        <div class=classes role="separator">
            {label.map(|l| view! { <span class="separator-label">{l}</span> })}
        </div>
    }
}
