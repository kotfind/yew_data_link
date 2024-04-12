use shadow_clone::shadow_clone;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_bind_link, use_data, UseLinkHandle};

pub struct DynListData {
    items: Vec<String>,
}

impl DynListData {
    fn new() -> Self {
        Self {
            items: (1..=3).map(|n| format!("Item {n}")).collect(),
        }
    }

    pub fn reset(&mut self) {
        self.items.clear();
    }

    pub fn get(&self) -> Vec<String> {
        self.items.clone()
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

    let items = data.apply(|data| data.items.clone());
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
            data.apply_mut(|data| data.items.push(name));
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
