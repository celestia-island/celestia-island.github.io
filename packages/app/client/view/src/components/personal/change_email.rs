use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use stylist::{css, yew::styled_component};
use yew::prelude::*;

use _client_functions::secure::update_personal_email;
use _types::website::request::api::PASSWORD_SPECIAL_CHARS_PATTERN;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub open: bool,
    pub onclose: Callback<()>,
}

#[styled_component]
pub fn ChangeEmail(props: &Props) -> HtmlResult {
    let new_email = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());

    Ok(html! {
        <>
            <div
                class={classes!(css!("
                    width: 100vw;
                    height: 100vh;
                    position: fixed;
                    top: 0;
                    left: 0;

                    display: flex;
                    align-items: center;
                    justify-content: center;

                    background: var(--color-dark-half);
                    z-index: 2000;
                    transition: all 0.3s;
                "), {
                    if props.open {
                        css!("
                            opacity: 1;
                        ")
                    } else {
                        css!("
                            opacity: 0;
                            pointer-events: none;
                        ")
                    }
                })}
            >
                <div
                    class={css!("
                        position: absolute;
                        width: 100%;
                        height: 100%;
                        top: 0;
                        left: 0;

                        z-index: -1;
                    ")}
                    onclick={
                        let onclose = props.onclose.clone();

                        move |_| {
                            onclose.emit(());
                        }
                    }
                />

                <section class={css!("
                    position: relative;
                    width: calc(100% - 32px * 2);
                    margin-bottom: 32px;
                    padding: 16px;

                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    justify-content: center;

                    background: var(--color-light);
                    border-radius: 4px;
                    box-shadow: var(--shadow-half);
                ")}>
                    <p class={css!("
                        height: 48px;
                        margin-left: 16px;

                        font-size: 20px;
                        line-height: 48px;
                        color: var(--font-title-color);
                        user-select: none;
                    ")}>
                        {"Change Email"}
                    </p>

                    <div class={css!("
                        width: 100%;

                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                    ")}>
                        <input
                            class={css!("
                                width: calc(100% - 32px);
                                height: 48px;
                                margin-top: 16px;
                            ")}
                            type={"email"}
                            placeholder={"New Email"}
                            value={(*new_email).clone()}
                            oninput={{
                                let new_name = new_email.clone();

                                move |event: InputEvent| {
                                    let target = event.target();
                                    let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                                    if let Some(input) = input {
                                        new_name.set(input.value());
                                    }
                                }
                            }}
                        />


                        <input
                            class={css!("
                                width: calc(100% - 32px);
                                height: 48px;
                                margin-top: 16px;
                            ")}
                            type={"password"}
                            pattern={PASSWORD_SPECIAL_CHARS_PATTERN}
                            placeholder={"Confirm Password"}
                            value={(*confirm_password).clone()}
                            oninput={{
                                let confirm_password = confirm_password.clone();

                                move |event: InputEvent| {
                                    let target = event.target();
                                    let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                                    if let Some(input) = input {
                                        confirm_password.set(input.value());
                                    }
                                }
                            }}
                        />

                        <button
                            class={css!("
                                width: 64px;
                                height: 48px;
                                margin-top: 16px;

                                font-size: 16px;
                            ")}
                            onclick={
                                let new_email = new_email.clone();
                                let confirm_password = confirm_password.clone();
                                let onclose = props.onclose.clone();

                                move |_| {
                                    let new_email = (*new_email).clone();
                                    let confirm_password = (*confirm_password).clone();
                                    let onclose = onclose.clone();

                                    wasm_bindgen_futures::spawn_local(async move {
                                        if update_personal_email(new_email, confirm_password).await.is_ok() {
                                            onclose.emit(());
                                            gloo::dialogs::alert("Successfully changed email");
                                            gloo::utils::window().location().reload().unwrap();
                                        } else {
                                            gloo::dialogs::alert("Failed to change email");
                                        }
                                    });
                                }
                            }
                        >
                            {"Submit"}
                        </button>
                    </div>
                </section>
            </div>
        </>
    })
}
