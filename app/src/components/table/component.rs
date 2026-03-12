use leptos::prelude::*;

#[component]
pub fn Table(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("table {}", class.unwrap_or(""));

    view! {
        <div class="table-wrapper">
            <table class=classes>{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! { <thead class="table-header">{children()}</thead> }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! { <tbody class="table-body">{children()}</tbody> }
}

#[component]
pub fn TableRow(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("table-row {}", class.unwrap_or(""));

    view! { <tr class=classes>{children()}</tr> }
}

#[component]
pub fn TableHead(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("table-head {}", class.unwrap_or(""));

    view! { <th class=classes>{children()}</th> }
}

#[component]
pub fn TableCell(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("table-cell {}", class.unwrap_or(""));

    view! { <td class=classes>{children()}</td> }
}
