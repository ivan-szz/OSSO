use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Accent,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "btn-primary",
            ButtonVariant::Destructive => "btn-destructive",
            ButtonVariant::Outline => "btn-outline",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Ghost => "btn-ghost",
            ButtonVariant::Link => "btn-link",
            ButtonVariant::Accent => "btn-accent",
        }
    }
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Default => "btn-default",
            ButtonSize::Sm => "btn-sm",
            ButtonSize::Lg => "btn-lg",
            ButtonSize::Icon => "btn-icon",
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let size = size.unwrap_or(ButtonSize::Default);
    let disabled = disabled.unwrap_or(false);

    let classes = format!(
        "btn {} {} {}",
        variant.classes(),
        size.classes(),
        class.unwrap_or("")
    );

    view! {
        <button
            class=classes
            disabled=disabled
            on:click=move |_| {
                if let Some(ref handler) = on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}
