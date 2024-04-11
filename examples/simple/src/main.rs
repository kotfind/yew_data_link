use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_bind_link, use_data, use_link, DataLink};

#[autoprops]
#[function_component]
fn Counter(link: &DataLink<i64>) -> Html {
    let num = use_data(|| 0);
    use_bind_link(link.clone(), num.clone());

    html! {
        <div>
            {num.get_cloned()}
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
            num.apply_mut(|num| *num += 1);
        })
    };

    let on_dec = {
        shadow_clone!(link);
        Callback::from(move |_| {
            let num = link.get().unwrap();
            num.apply_mut(|num| *num -= 1);
        })
    };

    html! {
        <div>
            <button onclick={on_inc}>{"+"}</button>
            <Counter link={(*link).clone()} />
            <button onclick={on_dec}>{"-"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
