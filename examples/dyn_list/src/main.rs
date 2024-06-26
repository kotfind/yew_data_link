use dyn_list::{DynList, DynListData, DynListMsg};
use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_data_link::use_link;

mod dyn_list;

#[function_component]
fn App() -> Html {
    let dyn_list_link = use_link::<DynListData>();

    dyn_list_link.msg_on_bind(DynListMsg::Set((1..=3).map(|n| format!("Item {n}")).collect()));

    let on_clear = {
        shadow_clone!(dyn_list_link);
        Callback::from(move |_| dyn_list_link.msg(DynListMsg::Clear))
    };

    let on_print = {
        shadow_clone!(dyn_list_link);
        Callback::from(move |_| dyn_list_link.msg(DynListMsg::Log))
    };

    html! {
        <div>
            <h3>{"Mutable list"}</h3>
            <DynList mutable=true link={dyn_list_link} />
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
