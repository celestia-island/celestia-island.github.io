use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::personal::*;
use _client_functions::secure::get_personal_info;
use _resources::icons;
use _utils::app::get_auth_cache;

#[styled_component]
pub fn Personal() -> HtmlResult {
    let info = use_state(|| None);

    use_effect_with((), {
        let info = info.clone();

        move |_| {
            if get_auth_cache().is_err() {
                gloo::utils::window().location().set_href("/login").unwrap();
            }

            wasm_bindgen_futures::spawn_local({
                let info = info.clone();

                async move {
                    info.set(
                        get_personal_info()
                            .await
                            .map(|val| Some(val))
                            .unwrap_or(None),
                    );
                }
            });

            || {}
        }
    });

    let has_change_name_modal_open = use_state(|| false);
    let has_change_email_modal_open = use_state(|| false);
    let has_change_password_modal_open = use_state(|| false);

    Ok(html! {
        <div class={css!("
            position: relative;
            width: 100%;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        ")}>
            <section class={css!("
                position: relative;
                width: 100%;
                margin-bottom: 32px;

                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
            ")}>
                <div class={css!("
                    width: 128px;
                    height: 128px;

                    display: flex;
                    align-items: center;
                    justify-content: center;

                    background: var(--color-primary-less);
                    border-radius: 50%;
                    border: 1px solid var(--color-dark-half);
                ")}>
                    <icons::Account size={64} color={"var(--color-light-most)"} />
                </div>

                {
                    if let Some(info) = (*info).clone() {
                        html! {
                            <span class={css!("
                                display: flex;
                                flex-direction: column;
                                align-items: center;
                                justify-content: center;
                            ")}>
                                <h3>
                                    {info.name}
                                </h3>
                                <p class={css!("
                                    height: 24px;
                                    margin: 4px;
                                    padding: 4px 8px;

                                    display: flex;
                                    align-items: center;
                                    justify-content: center;

                                    background: var(--color-primary-most);
                                    color: var(--color-light-most);
                                    border-radius: 4px;

                                    font-size: 12px;
                                    font-weight: bolder;
                                    line-height: 24px;

                                    user-select: none;
                                    opacity: 0.8;
                                    transition: all 0.3s;

                                    &:hover {
                                        opacity: 1;
                                    }
                                ")}>
                                    {if info.permission.is_root() {
                                        {"Manager"}
                                    } else {
                                        {"User"}
                                    }}
                                </p>

                                <h6>
                                    {info.email}
                                </h6>
                            </span>
                        }
                    } else {
                        html! {
                            <h3>{"Checking status"}</h3>
                        }
                    }
                }

                <div class={css!("
                    position: relative;
                    width: 100%;
                    height: 48px;

                    display: flex;
                    align-items: center;
                    justify-content: center;

                    @media (max-width: 768px) {
                        height: max-content;
                        margin-top: 16px;
                        flex-direction: column;
                    }
                ")}>
                    <button
                        class={css!("
                            width: 128px;
                            height: 32px;
                            margin: 4px;

                            font-size: 16px;

                            @media (max-width: 768px) {
                                height: 48px;
                            }
                        ")}
                        onclick={
                            let has_change_name_modal_open = has_change_name_modal_open.clone();

                            move |_| {
                                has_change_name_modal_open.set(true);
                            }
                        }
                    >
                        {"Rename"}
                    </button>
                    <button
                        class={css!("
                            width: 128px;
                            height: 32px;
                            margin: 4px;

                            font-size: 16px;

                            @media (max-width: 768px) {
                                height: 48px;
                            }
                        ")}
                        onclick={
                            let has_change_email_modal_open = has_change_email_modal_open.clone();

                            move |_| {
                                has_change_email_modal_open.set(true);
                            }
                        }
                    >
                        {"Change Email"}
                    </button>
                    <button
                        class={css!("
                            width: 128px;
                            height: 32px;
                            margin: 4px;

                            font-size: 16px;

                            @media (max-width: 768px) {
                                height: 48px;
                            }
                        ")}
                        onclick={
                            let has_change_password_modal_open = has_change_password_modal_open.clone();

                            move |_| {
                                has_change_password_modal_open.set(true);
                            }
                        }
                    >
                        {"Change Password"}
                    </button>
                </div>
            </section>

            <ChangeName
                open={*has_change_name_modal_open}
                onclose={{
                    let has_change_name_modal_open = has_change_name_modal_open.clone();

                    move |_| has_change_name_modal_open.set(false)
                }}
            />

            <ChangeEmail
                open={*has_change_email_modal_open}
                onclose={{
                    let has_change_email_modal_open = has_change_email_modal_open.clone();

                    move |_| has_change_email_modal_open.set(false)
                }}
            />

            <ChangePassword
                open={*has_change_password_modal_open}
                onclose={{
                    let has_change_password_modal_open = has_change_password_modal_open.clone();

                    move |_| has_change_password_modal_open.set(false)
                }}
            />
        </div>
    })
}
