use stylist::yew::styled_component;
use yew::prelude::*;

use _resources::icons;

#[styled_component]
pub fn Portal() -> HtmlResult {
    Ok(html! {
        <div class={css!("
            position: relative;
            width: 100%;

            display: flex;
            flex-wrap: wrap;
            align-items: center;
            justify-content: center;
        ")}>
            <icons::CircularProgress />
        </div>
    })
}
