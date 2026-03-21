use crate::components::alert::component::{Alert, AlertVariant};
use crate::components::button::component::*;
use crate::components::card::component::*;
use crate::components::input::component::Input;
use crate::server;
use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_meta::Title;
use server::auth::LoginAction;

#[component]
pub fn Login() -> impl IntoView {
    let login = ServerAction::<LoginAction>::new();
    let value = login.value();
    let pending = login.pending();

    let error_message = move || {
        value.with(|val| match val {
            Some(Err(e)) => Some(e.to_string()),
            _ => None,
        })
    };

    view! {
        <Title text="Login" />
        <div class="auth-container">
            <Card>
                <ActionForm action=login>
                    <CardHeader>
                        <CardTitle>
                            <h1>Login</h1>
                        </CardTitle>
                        <CardDescription>
                            {move || {
                                error_message()
                                    .map(|msg| {
                                        view! {
                                            <Alert variant=AlertVariant::Destructive>{msg}</Alert>
                                        }
                                    })
                            }}
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Input
                            label="Email"
                            placeholder="Enter your email"
                            input_type="email"
                            name="email"
                        />
                        <Input
                            label="Password"
                            placeholder="Enter your password"
                            input_type="password"
                            name="password"
                        />
                    </CardContent>
                    <CardFooter>
                        <Button
                            button_type="button"
                            disabled=pending
                            variant=ButtonVariant::Destructive
                        >
                            Cancel
                        </Button>
                        <Button button_type="submit" disabled=pending variant=ButtonVariant::Accent>
                            {move || if pending.get() { "Logging in..." } else { "Submit" }}
                        </Button>
                    </CardFooter>
                </ActionForm>
            </Card>
        </div>
    }
}
