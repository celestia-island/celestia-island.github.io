use log::{error, info};
use wasm_bindgen::JsCast as _;
use web_sys::HtmlInputElement;

use stylist::{css, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Routes;
use _utils::functions::auth::login;

#[styled_component]
pub fn Login() -> HtmlResult {
    let navigator = use_navigator().unwrap();

    let is_verifying = use_state(|| false);

    let email_raw = use_state(|| "".to_string());
    let password_raw = use_state(|| "".to_string());

    #[rustfmt::skip]
    let input_style = css!("
        width: 80%;
        height: 48px;
        margin-top: 16px;
    ");

    Ok(html! {
        <div class={css!("
            position: fixed;
            width: 400px;
            left: calc(50% - 200px);
            top: calc(64px + 32px);
            padding: 64px 16px;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            background: var(--color-light-half);
            border-radius: 8px;
            border: 1px solid var(--color-primary-half);
            box-shadow: var(--shadow-half);

            @media (max-width: 768px) {
                background: none;
                border: none;
                box-shadow: none;
            }
        ")}>
            <h1 class={css!("
                height: 48px;
                margin: 16px;

                line-height: 48px;
                text-align: center;

                font-size: 24px;
                font-weight: bolder;
                user-select: none;
            ")}>
                { "Login" }
            </h1>

            <input
                class={input_style.clone()}
                type="text"
                placeholder={"Email"}
                value={(*email_raw).clone()}
                oninput={
                    let email_raw = email_raw.clone();

                    move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            let input = input.value().chars().filter(|c| c.is_ascii()).collect::<String>();
                            email_raw.set(input);
                        }
                    }
                }
            />
            <input
                class={input_style.clone()}
                type="password"
                placeholder={"Password"}
                value={(*password_raw).clone()}
                oninput={
                    let password_raw = password_raw.clone();

                    move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            let input = input.value().chars().filter(|c| c.is_ascii()).collect::<String>();
                            password_raw.set(input);
                        }
                    }
                }
            />

            <button
                class={classes!(css!("
                    margin-top: 16px;
                    width: 60%;
                    height: 48px;

                    font-size: 16px;
                    line-height: 48px;
                "), {
                    if *is_verifying {
                        css!("
                            background: var(--color-dark-half);
                            cursor: not-allowed;
                            pointer-events: none;
                        ")
                    } else {
                        css!("")
                    }
                })}
                onclick={
                    let navigator = navigator.clone();
                    let is_verifying = is_verifying.clone();
                    let email_raw = email_raw.clone();
                    let password_raw = password_raw.clone();

                    move |_| {
                        let navigator = navigator.to_owned();

                        is_verifying.set(true);
                        let is_verifying = is_verifying.clone();

                        let email_raw = (*email_raw).clone();
                        let password_raw = (*password_raw).clone();

                        wasm_bindgen_futures::spawn_local(async move {
                            let navigator = navigator.to_owned();

                            match login(email_raw, password_raw).await {
                                Ok(info) => {
                                    info!("Login success: {:?}", info);
                                    navigator.push(&Routes::Portal);
                                    gloo::utils::window().location().reload().unwrap();
                                }
                                Err(err) => {
                                    error!("Login failed: {:?}", err);
                                    gloo::dialogs::alert(&format!("Login failed: {:?}", err));
                                }
                            }

                            is_verifying.set(false);
                        });
                    }
                }
            >
                {"Login"}
            </button>

            <button
                class={css!("
                    margin-top: 16px;
                    width: 60%;
                    height: 48px;

                    font-size: 16px;
                    line-height: 48px;
                ")}
                onclick={
                    let navigator = navigator.clone();

                    move |_| {
                        navigator.push(&Routes::Register);
                    }
                }
            >
                {"Register"}
            </button>
        </div>
    })
}
