use leptos::prelude::*;

#[component]
pub fn FormField(
    children: Children,
    #[prop(optional)] error: Option<ReadSignal<Option<String>>>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("form-field {}", class.unwrap_or(""));

    view! {
        <div class=classes>
            {children()}
            {move || {
                error
                    .and_then(|sig| sig.get())
                    .map(|msg| {
                        view! { <p class="form-field-error">{msg}</p> }
                    })
            }}
        </div>
    }
}
