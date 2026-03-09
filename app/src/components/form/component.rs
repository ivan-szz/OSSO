use leptos::prelude::*;

#[component]
pub fn Form(children: Children) -> impl IntoView {
    view! {
        <div class="form-container">
            <form>{children()}</form>
        </div>
    }
}
