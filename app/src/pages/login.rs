use crate::components::button::component::{Button, ButtonVariant};
use crate::components::form::component::Form;
use crate::components::input::component::Input;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Title text="Login" />
        <div class="auth-container">
            <Form>
                <h1>Login</h1>
                <Input label="Email" placeholder="Enter your email" input_type="email" />
                <Button variant=ButtonVariant::Accent>Submit</Button>
            </Form>
        </div>
    }
}
