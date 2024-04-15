use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_data_link, use_link, UseLinkHandle};

#[autoprops]
#[function_component]
fn Counter(#[prop_or_default] link: &UseLinkHandle<i64>) -> Html {
    let num_link = use_data_link(|| 0);
    link.bind(&num_link);
    let num_data = num_link.get().unwrap();

    html! {
        <div>
            {num_data.get_cloned()}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let link = use_link();

    let on_inc = {
        shadow_clone!(link);
        Callback::from(move |_| {
            let num = link.get().unwrap();
            num.apply(|num| *num += 1);
        })
    };

    let on_dec = {
        shadow_clone!(link);
        Callback::from(move |_| {
            let num = link.get().unwrap();
            num.apply(|num| *num -= 1);
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
