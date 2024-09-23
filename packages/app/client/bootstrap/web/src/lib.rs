use wasm_bindgen::prelude::*;

use _utils::app::get_auth_cache;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {}

#[wasm_bindgen]
impl WebHandle {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        console_log::init_with_level(log::Level::Debug).unwrap();

        #[cfg(not(debug_assertions))]
        console_log::init_with_level(log::Level::Info).unwrap();

        Self {}
    }

    #[wasm_bindgen]
    pub async fn start(&self) -> Result<(), wasm_bindgen::JsValue> {
        use _client_view::app::{App, AppStates};

        let state: AppStates = serde_json::from_str(
            &gloo::utils::document()
                .get_element_by_id("__ssr_data")
                .ok_or(wasm_bindgen::JsValue::from_str(
                    "Cannot get SSR data element",
                ))?
                .inner_html(),
        )
        .map_err(|err| wasm_bindgen::JsValue::from_str(err.to_string().as_str()))?;
        gloo::utils::body().remove_child(
            &gloo::utils::document()
                .get_element_by_id("__ssr_data")
                .ok_or(wasm_bindgen::JsValue::from_str(
                    "Cannot get SSR data element",
                ))?
                .into(),
        )?;

        let state = AppStates {
            title: state.title,
            auth: get_auth_cache().map(|auth| Some(auth)).unwrap_or(None),
        };

        <App as hikari_boot::Application>::render_with_root(
            gloo::utils::document()
                .get_element_by_id("app")
                .ok_or(wasm_bindgen::JsValue::from_str("Cannot get root element"))?,
            state,
        );
        Ok(())
    }
}
