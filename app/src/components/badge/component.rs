use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "badge-default",
            BadgeVariant::Secondary => "badge-secondary",
            BadgeVariant::Destructive => "badge-destructive",
            BadgeVariant::Outline => "badge-outline",
            BadgeVariant::Success => "badge-success",
            BadgeVariant::Warning => "badge-warning",
        }
    }
}

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let variant = variant.unwrap_or(BadgeVariant::Default);
    let classes = format!("badge {} {}", variant.classes(), class.unwrap_or(""));

    view! { <span class=classes>{children()}</span> }
}
