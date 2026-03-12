use crate::components::alert::component::{Alert, AlertDescription, AlertVariant};
use leptos::prelude::*;

#[component]
pub fn Form(
    children: Children,
    #[prop(optional)] on_submit: Option<Box<dyn Fn() + 'static>>,
    #[prop(optional)] error: Option<ReadSignal<Option<String>>>,
    #[prop(optional)] loading: Option<ReadSignal<bool>>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let form_classes = move || {
        let loading_class = if loading.map(|s| s.get()).unwrap_or(false) {
            " form-loading"
        } else {
            ""
        };
        format!(
            "form-container{} {}",
            loading_class,
            class.unwrap_or("")
        )
    };

    view! {
        <div class=form_classes>
            <form on:submit=move |ev| {
                ev.prevent_default();
                if let Some(ref handler) = on_submit {
                    handler();
                }
            }>
                {move || {
                    error
                        .and_then(|sig| sig.get())
                        .map(|msg| {
                            view! {
                                <Alert variant=AlertVariant::Destructive>
                                    <AlertDescription>{msg}</AlertDescription>
                                </Alert>
                            }
                        })
                }} {children()}
            </form>
        </div>
    }
}
