use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool) + 'static>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let classes = format!("checkbox {}", class.unwrap_or(""));

    view! {
        <input
            type="checkbox"
            class=classes
            id=id
            checked=checked
            disabled=disabled
            on:change=move |ev| {
                if let Some(ref handler) = on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}
