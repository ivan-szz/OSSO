use leptos::prelude::*;

#[component]
pub fn Switch(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool) + 'static>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let (is_checked, set_is_checked) = signal(checked.unwrap_or(false));

    let classes = move || {
        format!(
            "switch{}{} {}",
            if is_checked.get() { " switch-checked" } else { "" },
            if disabled { " switch-disabled" } else { "" },
            class.unwrap_or("")
        )
    };

    view! {
        <button
            type="button"
            role="switch"
            aria-checked=move || is_checked.get().to_string()
            class=classes
            id=id
            disabled=disabled
            on:click=move |_| {
                let new_val = !is_checked.get();
                set_is_checked.set(new_val);
                if let Some(ref handler) = on_change {
                    handler(new_val);
                }
            }
        >
            <span class="switch-thumb"></span>
        </button>
    }
}
