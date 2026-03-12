use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Default,
    Destructive,
    Warning,
    Success,
}

impl AlertVariant {
    fn classes(&self) -> &'static str {
        match self {
            AlertVariant::Default => "alert-default",
            AlertVariant::Destructive => "alert-destructive",
            AlertVariant::Warning => "alert-warning",
            AlertVariant::Success => "alert-success",
        }
    }
}

#[component]
pub fn Alert(
    children: Children,
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let variant = variant.unwrap_or(AlertVariant::Default);
    let classes = format!("alert {} {}", variant.classes(), class.unwrap_or(""));

    view! {
        <div class=classes role="alert">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("alert-title {}", class.unwrap_or(""));

    view! { <h5 class=classes>{children()}</h5> }
}

#[component]
pub fn AlertDescription(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("alert-description {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}
