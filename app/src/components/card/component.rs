use leptos::prelude::*;

#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card-header {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card-title {}", class.unwrap_or(""));

    view! { <h3 class=classes>{children()}</h3> }
}

#[component]
pub fn CardDescription(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card-description {}", class.unwrap_or(""));

    view! { <p class=classes>{children()}</p> }
}

#[component]
pub fn CardContent(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card-content {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardFooter(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("card-footer {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}
