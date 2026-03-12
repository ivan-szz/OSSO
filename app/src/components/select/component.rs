use std::rc::Rc;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[component]
pub fn Select(
    options: Vec<SelectOption>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] error: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let error = error.unwrap_or(false);
    let placeholder = placeholder.unwrap_or("Select...");

    let initial_label = value
        .as_ref()
        .and_then(|v| options.iter().find(|o| o.value == *v).map(|o| o.label.clone()));

    let (is_open, set_is_open) = signal(false);
    let (selected_value, set_selected_value) = signal(value.unwrap_or_default());
    let (selected_label, set_selected_label) = signal(initial_label.unwrap_or_default());
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    // Close on outside click via focusout with relatedTarget check
    let on_focusout = move |ev: leptos::web_sys::FocusEvent| {
        use leptos::web_sys::wasm_bindgen::JsCast;
        if let Some(wrapper) = wrapper_ref.get() {
            let related = ev.related_target()
                .and_then(|t| t.dyn_into::<leptos::web_sys::Node>().ok());
            let wrapper_el: &leptos::web_sys::Node = &wrapper;
            let still_inside = related
                .map(|r| wrapper_el.contains(Some(&r)))
                .unwrap_or(false);
            if !still_inside {
                set_is_open.set(false);
            }
        }
    };

    let trigger_classes = move || {
        format!(
            "select-trigger{}{}{} {}",
            if error { " select-error" } else { "" },
            if disabled { " disabled" } else { "" },
            if is_open.get() { " select-open" } else { "" },
            class.unwrap_or("")
        )
    };

    let has_value = move || !selected_value.get().is_empty();

    view! {
        <div class="select-wrapper" node_ref=wrapper_ref on:focusout=on_focusout>
            {label.map(|l| view! { <span class="select-label">{l}</span> })}
            <button
                type="button"
                class=trigger_classes
                disabled=disabled
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <span class="select-value" class:select-placeholder=move || !has_value()>
                    {move || {
                        let label = selected_label.get();
                        if label.is_empty() { placeholder.to_string() } else { label }
                    }}
                </span>
                <svg
                    class="select-chevron"
                    xmlns="http://www.w3.org/2000/svg"
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M6 9l6 6 6-6"></path>
                </svg>
            </button>
            <div
                class="select-content"
                style:display=move || if is_open.get() { "block" } else { "none" }
            >
                {
                    let on_change = Rc::new(on_change);
                    options
                        .into_iter()
                        .map(|opt| {
                            let val_for_selected = opt.value.clone();
                            let val_for_click = opt.value.clone();
                            let val_for_check = opt.value.clone();
                            let val_for_icon = opt.value.clone();
                            let label_for_click = opt.label.clone();
                            let label_for_display = opt.label.clone();
                            let on_change = on_change.clone();
                            view! {
                                <button
                                    type="button"
                                    class="select-item"
                                    class:select-item-selected=move || {
                                        selected_value.get() == val_for_selected
                                    }
                                    on:click=move |_| {
                                        set_selected_value.set(val_for_click.clone());
                                        set_selected_label.set(label_for_click.clone());
                                        set_is_open.set(false);
                                        if let Some(ref handler) = *on_change {
                                            handler(val_for_check.clone());
                                        }
                                    }
                                >
                                    <span>{label_for_display}</span>
                                    <svg
                                        class="select-item-check"
                                        style:visibility=move || {
                                            if selected_value.get() == val_for_icon {
                                                "visible"
                                            } else {
                                                "hidden"
                                            }
                                        }
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="14"
                                        height="14"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <polyline points="20 6 9 17 4 12"></polyline>
                                    </svg>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}
