use leptos::prelude::*;

#[component]
pub fn Label(
    children: Children,
    #[prop(optional)] for_id: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("label {}", class.unwrap_or(""));

    view! {
        <label class=classes for=for_id>
            {children()}
        </label>
    }
}
