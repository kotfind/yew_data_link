use shadow_clone::shadow_clone;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_bind_link, use_data, MsgData, UseLinkHandle};

pub struct DynListData {
    items: Vec<String>,
}

pub enum DynListMsg {
    Clear,
    Log,
    Push(String),
    Set(Vec<String>),
}

impl MsgData for DynListData {
    type Msg = DynListMsg;

    fn msg(&mut self, msg: Self::Msg) {
        match msg {
            DynListMsg::Clear => self.items.clear(),
            DynListMsg::Log => {
                log::info!("List items:");
                for item in &self.items {
                    log::info!("{}", item);
                }
            }
            DynListMsg::Push(item) => self.items.push(item),
            DynListMsg::Set(items) => self.items = items,
        };
    }
}

impl DynListData {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

#[autoprops]
#[function_component]
pub fn DynList(
    #[prop_or_default] link: &UseLinkHandle<DynListData>,
    #[prop_or(false)] mutable: &bool,
) -> Html {
    let data = use_data(DynListData::new);
    use_bind_link(link.clone(), data.clone());

    let items = &data.current().items;
    let items = items.iter().map(|item| {
        html! {
            <li>{item}</li>
        }
    });

    let input_ref = use_node_ref();
    let onclick = {
        shadow_clone!(input_ref, data);
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let name = input.value();
            input.set_value("");
            data.msg(DynListMsg::Push(name));
        })
    };

    html! {
        <ul>
            {for items}

            if *mutable {
                <li>
                    <input ref={input_ref} type="text"/>
                    <button {onclick}>{"+"}</button>
                </li>
            }
        </ul>
    }
}
