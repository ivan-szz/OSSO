use leptos::prelude::*;

#[component]
pub fn Dialog(
    children: Children,
    open: ReadSignal<bool>,
    on_close: Callback<()>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("dialog-panel {}", class.unwrap_or(""));
    let rendered_children = children();

    view! {
        <div
            class="dialog-overlay"
            style:display=move || if open.get() { "flex" } else { "none" }
            on:click=move |_| on_close.run(())
        >
            <div class=classes.clone() on:click=move |ev| ev.stop_propagation()>
                {rendered_children}
            </div>
        </div>
    }
}

#[component]
pub fn DialogHeader(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("dialog-header {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! { <h2 class="dialog-title">{children()}</h2> }
}

#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! { <p class="dialog-description">{children()}</p> }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("dialog-content {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn DialogFooter(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("dialog-footer {}", class.unwrap_or(""));

    view! { <div class=classes>{children()}</div> }
}
