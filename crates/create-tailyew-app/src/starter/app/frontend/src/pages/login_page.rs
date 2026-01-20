use crate::helpers::{persist_login, send_request, ResponseError};
use crate::helpers::persist_login::LoginResponse;
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};
use tailyew::form::{async_callback, e_input_value, Form, FormSubmitCallback, Input, InputType};
use tailyew::organisms::Card;
use web_sys::SubmitEvent;
use yew::prelude::*;

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SuccessResponse<T> {
    data: Option<T>,
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let login_data = LoginRequest { email, password };

    // TODO: Update this with a real url to login to your app 
    let url = format!("https://tools-httpstatus.pickup-services.com/{}", 401);
    web_sys::console::log_1(&format!("Mock login hitting: {url}").into());

    let response_data: Result<SuccessResponse<LoginResponse>, ResponseError> =
        send_request(url.to_string(), Method::POST, Some(login_data)).await;

    response_data
        .map_err(|err| match err {
            ResponseError::StatusCodeError(401, _) => {
                "Unauthorized: Incorrect email or password.".to_string()
            }
            ResponseError::StatusCodeError(403, _) => {
                "Forbidden: You do not have permission to log in.".to_string()
            }
            _ => format!("Failed to login: {}", err),
        })
        .and_then(|response| response.data.ok_or_else(|| "No data returned.".to_string()))
}

#[component(LoginPage)]
pub fn login_page() -> Html {
    let error_message = use_state(|| None::<String>);

    // Async form submit callback
    let onsubmit_callback: FormSubmitCallback = async_callback({
        let error_message = error_message.clone();

        move |event: SubmitEvent| {
            let email = e_input_value("email", &event);
            let password = e_input_value("password", &event);

            let error_message = error_message.clone();

            async move {
                if email.is_empty() || password.is_empty() {
                    error_message.set(Some("Please enter both email and password.".into()));
                    return Err("Missing credentials".into());
                }

                match login(email, password).await {
                    Ok(response) => {
                        persist_login(&response);
                        Ok(Some("You logged in to the app!".to_string()))
                    }
                    Err(err) => {
                        error_message.set(Some(err.clone()));
                        Err(err)
                    }
                }
            }
        }
    });

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-800">
            <Card
                id="login-card"
                title="Login"
                subtitle={Some("Please log in to continue.".to_string())}
                class={classes!("max-w-md", "w-full")}
            >
                    <Form
                        onsubmit_callback={onsubmit_callback}
                        form_class={classes!("space-y-4")}
                    >
                        <Input
                            label="Email"
                            id="email"
                            input_type={InputType::Email}
                            placeholder="Enter your email"
                            required={true}
                        />
                        <Input
                            label="Password"
                            id="password"
                            input_type={InputType::Password}
                            placeholder="Enter your password"
                            required={true}
                        />
                    </Form>
            </Card>
        </div>
    }
}
