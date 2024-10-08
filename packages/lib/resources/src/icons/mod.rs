mod circular_progress;

pub use circular_progress::CircularProgress;

use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or(32)]
    pub size: u32,
    #[prop_or("var(--icon-color)".to_string())]
    pub color: String,
}

macro_rules! icon {
    ($name: ident, $source_path: expr) => {
        #[styled_component]
        pub fn $name(props: &Props) -> Html {
            html! {
                <div
                    class={css!("
                        width: var(--icon-size);
                        height: var(--icon-size);

                        & > svg {
                            color: var(--icon-color);
                            transition: all 0s;
                        }
                    ")}
                    style={format!("
                        --icon-size: {}px;
                        --icon-color: {};
                    ", props.size, props.color)}
                >
                    {Html::from_html_unchecked(include_str!($source_path).into())}
                </div>
            }
        }
    };
}

icon!(Check, "./check.svg");
icon!(Close, "./close.svg");
icon!(Account, "./account.svg");
icon!(AccountEdit, "./account_edit.svg");
icon!(Logout, "./logout.svg");
icon!(Menu, "./menu.svg");
icon!(MenuRight, "./menu_right.svg");
icon!(MenuBack, "./menu_back.svg");

pub const FAVICON: &[u8] = include_bytes!("favicon.ico");
