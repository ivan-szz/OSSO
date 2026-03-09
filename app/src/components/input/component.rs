use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] error: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or("text");
    let disabled = disabled.unwrap_or(false);
    let error = error.unwrap_or(false);

    let classes = format!(
        "input{}{} {}",
        if error { " input-error" } else { "" },
        if disabled { " disabled" } else { "" },
        class.unwrap_or("")
    );

    view! {
        <label class="input-wrapper">
            {label.map(|l| view! { <span class="input-label">{l}</span> })}
            <input
                type=input_type
                placeholder=placeholder
                name=name
                value=value
                disabled=disabled
                class=classes
                on:input=move |ev| {
                    if let Some(ref handler) = on_input {
                        handler(event_target_value(&ev));
                    }
                }
            />
        </label>
    }
}
