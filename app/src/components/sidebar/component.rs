use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct SidebarState {
    pub open: RwSignal<bool>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SidebarSide {
    Left,
    Right,
}

impl SidebarSide {
    fn class(&self) -> &'static str {
        match self {
            SidebarSide::Left => "sidebar-left",
            SidebarSide::Right => "sidebar-right",
        }
    }
}

#[component]
pub fn SidebarProvider(
    children: Children,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let open = RwSignal::new(default_open.unwrap_or(true));
    provide_context(SidebarState { open });

    let classes = format!("sidebar-provider {}", class.unwrap_or(""));

    view! {
        <div class=classes data-sidebar-state=move || if open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(optional)] side: Option<SidebarSide>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let state = use_context::<SidebarState>().expect("Sidebar must be used within SidebarProvider");
    let side = side.unwrap_or(SidebarSide::Left);

    let classes = move || {
        format!(
            "sidebar {} {} {}",
            side.class(),
            if state.open.get() { "sidebar-open" } else { "sidebar-closed" },
            class.unwrap_or("")
        )
    };

    view! {
        <aside class=classes data-state=move || if state.open.get() { "open" } else { "closed" }>
            <div class="sidebar-inner">{children()}</div>
        </aside>
        <div
            class="sidebar-overlay"
            style:display=move || if state.open.get() { "block" } else { "none" }
            on:click=move |_| state.open.set(false)
        />
    }
}

#[component]
pub fn SidebarHeader(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-header {}", class.unwrap_or(""));
    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-content {}", class.unwrap_or(""));
    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn SidebarFooter(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-footer {}", class.unwrap_or(""));
    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn SidebarGroup(
    children: Children,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-group {}", class.unwrap_or(""));
    view! {
        <div class=classes>
            {label.map(|l| view! { <span class="sidebar-group-label">{l}</span> })} {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-menu {}", class.unwrap_or(""));
    view! { <ul class=classes>{children()}</ul> }
}

#[component]
pub fn SidebarMenuItem(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = format!("sidebar-menu-item {}", class.unwrap_or(""));
    view! { <li class=classes>{children()}</li> }
}

#[component]
pub fn SidebarMenuLink(
    children: Children,
    href: &'static str,
    #[prop(optional)] active: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let active = active.unwrap_or(false);
    let classes = format!(
        "sidebar-menu-link{} {}",
        if active { " sidebar-menu-link-active" } else { "" },
        class.unwrap_or("")
    );

    view! {
        <a href=href class=classes>
            {children()}
        </a>
    }
}

#[component]
pub fn SidebarTrigger(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    let state =
        use_context::<SidebarState>().expect("SidebarTrigger must be used within SidebarProvider");

    let classes = format!("sidebar-trigger {}", class.unwrap_or(""));

    view! {
        <button type="button" class=classes on:click=move |_| state.open.update(|v| *v = !*v)>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="3" y1="6" x2="21" y2="6" />
                <line x1="3" y1="12" x2="21" y2="12" />
                <line x1="3" y1="18" x2="21" y2="18" />
            </svg>
        </button>
    }
}

#[component]
pub fn SidebarRail(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    let state =
        use_context::<SidebarState>().expect("SidebarRail must be used within SidebarProvider");

    let classes = format!("sidebar-rail {}", class.unwrap_or(""));

    view! {
        <button
            type="button"
            class=classes
            tabindex=-1
            aria-hidden="true"
            on:click=move |_| state.open.update(|v| *v = !*v)
        />
    }
}
