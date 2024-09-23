use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[styled_component]
pub fn Header(props: &Props) -> HtmlResult {
    Ok(html! {
        <>
            {props.children.clone()}
        </>
    })
}
