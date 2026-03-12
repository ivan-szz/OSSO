use leptos::prelude::*;

#[component]
pub fn Tabs(
    children: Children,
    default_value: &'static str,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let active_tab = RwSignal::new(default_value.to_string());
    provide_context(active_tab);

    let classes = format!("tabs {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("tabs-list {}", class.unwrap_or(""));

    view! {
        <div class=classes role="tablist">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(
    children: Children,
    value: &'static str,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let active_tab =
        use_context::<RwSignal<String>>().expect("TabsTrigger must be used within Tabs");
    let disabled = disabled.unwrap_or(false);

    let classes = move || {
        format!(
            "tabs-trigger{} {}",
            if active_tab.get() == value {
                " tabs-trigger-active"
            } else {
                ""
            },
            class.unwrap_or("")
        )
    };

    view! {
        <button
            type="button"
            role="tab"
            class=classes
            disabled=disabled
            aria-selected=move || (active_tab.get() == value).to_string()
            on:click=move |_| active_tab.set(value.to_string())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    children: Children,
    value: &'static str,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let active_tab =
        use_context::<RwSignal<String>>().expect("TabsContent must be used within Tabs");

    let classes = format!("tabs-content {}", class.unwrap_or(""));

    view! {
        <div class=classes role="tabpanel" hidden=move || active_tab.get() != value>
            {children()}
        </div>
    }
}
