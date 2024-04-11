use crate::dyn_list::DynList;
use dyn_list::DynListData;
use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_data_link::use_link;

mod dyn_list;

#[function_component]
fn App() -> Html {
    let dyn_list_link = use_link::<DynListData>();

    let on_clear = {
        shadow_clone!(dyn_list_link);
        Callback::from(move |_| {
            let data = dyn_list_link.get().unwrap();
            data.apply_mut(|data| data.reset())
        })
    };

    let on_print = {
        shadow_clone!(dyn_list_link);
        Callback::from(move |_| {
            let data = dyn_list_link.get().unwrap();
            data.apply(|data| {
                log::info!("List items:");
                for item in data.get() {
                    log::info!("{}", item);
                }
            });
        })
    };

    html! {
        <div>
            <h3>{"Mutable list"}</h3>
            <DynList mutable=true link={(*dyn_list_link).clone()} />
            <button onclick={on_clear}>{"Clear"}</button>
            <button onclick={on_print}>{"Print to console"}</button>

            <h3>{"Immutable list"}</h3>
            <DynList />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
