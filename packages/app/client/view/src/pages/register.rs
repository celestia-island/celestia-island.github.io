use log::{error, info};
use wasm_bindgen::JsCast as _;
use web_sys::HtmlInputElement;

use stylist::{css, yew::styled_component};
use yew::prelude::*;

use _utils::app::get_auth_cache;
use _utils::functions::auth::register;

#[styled_component]
pub fn Register() -> HtmlResult {
    use_effect_with((), move |_| {
        if get_auth_cache().is_ok() {
            gloo::utils::window()
                .location()
                .set_href("/personal")
                .unwrap();
        }

        || {}
    });

    let name_raw = use_state(|| "".to_string());
    let password_raw = use_state(|| "".to_string());
    let email_raw = use_state(|| "".to_string());

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
                color: var(--font-color);
                user-select: none;
            ")}>
                { "Register" }
            </h1>

            <input
                class={input_style.clone()}
                type="text"
                placeholder={"Username"}
                value={(*name_raw).clone()}
                oninput={
                    let name_raw = name_raw.clone();

                    move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            name_raw.set(input.value());
                        }
                    }
                }
            />
            <input
                class={input_style.clone()}
                type="text"
                placeholder={"Password"}
                value={(*password_raw).clone()}
                oninput={
                    let password_raw = password_raw.clone();

                    move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            password_raw.set(input.value());
                        }
                    }
                }
            />
            <input
                class={input_style.clone()}
                type="email"
                placeholder={"Email"}
                value={(*email_raw).clone()}
                oninput={
                    let email_raw = email_raw.clone();

                    move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            email_raw.set(input.value());
                        }
                    }
                }
            />

            <button
                class={css!("
                    margin-top: 16px;
                    width: 60%;
                    height: 48px;

                    font-size: 16px;
                    line-height: 48px;
                ")}
                onclick={
                    let name_raw = name_raw.clone();
                    let password_raw = password_raw.clone();
                    let email_raw = email_raw.clone();

                    move |_| {
                        let name = (*name_raw).clone();
                        let password_raw = (*password_raw).clone();
                        let email = (*email_raw).clone();

                        wasm_bindgen_futures::spawn_local(async move {
                            match register(
                                name,
                                password_raw,
                                email,
                            ).await {
                                Ok(_) => {
                                    info!("Register success");
                                    gloo::utils::window().location().set_href("/").unwrap();
                                }
                                Err(err) => {
                                    error!("Register failed: {:?}", err);
                                    gloo::dialogs::alert(&format!("Register failed: {:?}", err));
                                }
                            }
                        });
                    }
                }
            >
                {"Register"}
            </button>
        </div>
    })
}
