use yew::prelude::*;

use _types::website::response::api::AuthInfo;

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalState {
    pub token: AuthInfo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GlobalStateAction {
    SetToken(AuthInfo),
}

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        GlobalState {
            token: match action {
                GlobalStateAction::SetToken(token) => token,
            },
        }
        .into()
    }
}

pub type GlobalStateContext = UseReducerHandle<GlobalState>;

#[derive(Properties, Debug, PartialEq)]
pub struct ProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Provider(props: &ProviderProps) -> Html {
    let state = use_reducer(|| GlobalState { token: None });

    html! {
        <ContextProvider<GlobalStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}
