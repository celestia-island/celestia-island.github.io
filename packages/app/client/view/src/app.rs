use anyhow::Result;
use serde::{Deserialize, Serialize};

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{Background, Header},
    pages::*,
};
use _types::website::response::api::AuthInfo;
use _utils::browser::global_state::Provider;
use hikari_boot::{DeclType, DeriveApplication, DeriveRoutes, RoutesOutsideProps};

#[derive(PartialEq, Clone, Debug, DeriveRoutes, Routable)]
pub enum Routes {
    #[at("/")]
    #[component(Portal)]
    Portal,

    #[at("/login")]
    #[component(Login)]
    Login,
    #[at("/register")]
    #[component(Register)]
    Register,
    #[at("/personal")]
    #[component(Personal)]
    Personal,
    #[not_found]
    #[at("/404")]
    #[component(NotFound)]
    NotFound,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct AppStates {
    pub title: String,
    pub auth: AuthInfo,
}

#[derive(Clone, Debug, DeriveApplication)]
pub struct App;

impl DeclType for App {
    type Routes = Routes;
    type AppStates = AppStates;

    #[allow(non_upper_case_globals)]
    fn decl_render_outside(props: &RoutesOutsideProps<Self::AppStates>) -> yew::HtmlResult {
        let theme_raw = r#"
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            :root {
                --color-primary: rgb(34, 111, 159);
                --color-primary-most: rgba(34, 111, 159, 0.8);
                --color-primary-half: rgba(34, 111, 159, 0.5);
                --color-primary-less: rgba(34, 111, 159, 0.2);

                --color-dark: rgb(0, 0, 0);
                --color-dark-most: rgba(0, 0, 0, 0.8);
                --color-dark-half: rgba(0, 0, 0, 0.5);
                --color-dark-less: rgba(0, 0, 0, 0.2);

                --color-light: rgb(255, 255, 255);
                --color-light-most: rgba(255, 255, 255, 0.8);
                --color-light-half: rgba(255, 255, 255, 0.5);
                --color-light-less: rgba(255, 255, 255, 0.2);

                --color-shadow-half: 0 0 2px 2px var(--color-primary-less);
                --color-shadow-most: 0 0 4px 4px var(--color-primary-less);

                --icon-color: var(--color-primary-most);
                --font-color: var(--color-dark-most);
                --font-title-color: var(--color-primary-most);
            }

            button,
            h1,
            h2,
            h3,
            h4,
            h5,
            h6,
            a {
                outline: none;
                text-decoration: none;
                color: var(--font-title-color);
            }

            span, p {
                color: var(--font-color);
            }

            button {
                text-align: center;
                user-select: none;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;

                border: 1px solid var(--color-primary-half);
                border-radius: 4px;
                color: var(--font-title-color);

                outline: transparent;
                transition: all 0.3s;

                &:hover {
                    box-shadow: var(--color-shadow-half);
                }
                &:active {
                    box-shadow: var(--color-shadow-most);
                }
            }

            input {
                padding: 0 16px;
                border: 1px solid #ccc;
                border-radius: 4px;
                outline: none;

                background: var(--color-light);
                box-shadow: var(--shadow-half);

                line-height: 48px;
                font-size: 16px;

                cursor: text;
            }

            input:invalid {
                border: 1px solid red;
            }
        "#;

        Ok(yew::html! {
            <Provider>
                <style>
                    {theme_raw}
                </style>

                <Background />

                <Header>
                    {props.children.clone()}
                </Header>
            </Provider>
        })
    }

    fn render_to_string_outside(
        style_raw: String,
        _html_raw: String,
        state: Self::AppStates,
    ) -> Result<String> {
        let title = state.title.clone();
        let title_suffix = "Celestia";

        let state_raw = ::serde_json::to_string(&state).unwrap();

        Ok(format!("
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset='utf-8'>
                    <meta name='viewport' content='width=device-width, initial-scale=1'>
                    {style_raw}
                    <title>{title} - {title_suffix}</title>
                </head>
                <body>
                    <textarea id='__ssr_data' style='display: none;'>{state_raw}</textarea>
                    <div id='app'>
                        <style>
                            @keyframes rotate {{
                                from {{
                                    transform: rotate(0deg);
                                }}
                                to {{
                                    transform: rotate(360deg);
                                }}
                            }}

                            body {{
                                margin: 0;
                                padding: 0;
                            }}

                            #__loading_outside {{
                                width: 100vw;
                                height: 100vh;

                                display: flex;
                                align-items: center;
                                justify-content: center;
                            }}

                            #__loading {{
                                width: 64px;
                                height: 64px;

                                border-radius: 50%;
                                border: 2px solid transparent;
                                border-top-color: var(--icon-color);
                                border-bottom-color: var(--icon-color);

                                animation: rotate 1s linear infinite;
                            }}
                        </style>
                        <div id='__loading_outside'>
                            <div id='__loading'></div>
                        </div>
                    </div>
                    <script src='/app.js'></script>
                    <script>(async () => {{await wasm_vendor_entry('/app.wasm');(await (new wasm_vendor_entry.WebHandle())).start();}})()</script>
                </body>
            </html>
        "))
    }
}
