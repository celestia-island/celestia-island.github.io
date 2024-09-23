use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[styled_component]
pub fn Background() -> HtmlResult {
    Ok(html! {
        <>
            <div class={css!("
                width: 100vw;
                height: 100vh;
                position: fixed;
                top: 0;
                left: 0;

                background-image: linear-gradient(
                    to top,
                    rgb(244, 244, 244) 0%,
                    rgb(223, 222, 220) 100%
                );
                z-index: -1;
            ")} />
        </>
    })
}
