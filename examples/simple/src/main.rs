use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_create_data, use_link, use_link_data, UseLinkHandle};

#[autoprops]
#[function_component]
fn Counter(#[prop_or_default] link: &UseLinkHandle<i64>) -> Html {
    let num = use_create_data(|| 0);
    link.bind(&num);

    html! {
        <div>
            {num.get_cloned()}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let link = use_link();
    let num = use_link_data(link.clone());

    let on_inc = {
        shadow_clone!(num);
        Callback::from(move |_| {
            num.as_ref().unwrap().apply(|num| *num += 1);
        })
    };

    let on_dec = {
        shadow_clone!(num);
        Callback::from(move |_| {
            num.as_ref().unwrap().apply(|num| *num -= 1);
        })
    };

    html! {
        <div>
            <button onclick={on_inc}>{"+"}</button>
            <Counter link={link} />
            <button onclick={on_dec}>{"-"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
