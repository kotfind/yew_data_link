use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_data_link::{use_bind_link, use_data, use_link, MsgData, UseLinkHandle};

struct Num(i64);

enum NumMsg {
    Inc,
    Dec,
}

impl MsgData for Num {
    type Msg = NumMsg;

    fn msg(&mut self, msg: NumMsg) {
        match msg {
            NumMsg::Inc => self.0 += 1,
            NumMsg::Dec => self.0 -= 1,
        };
    }
}

#[autoprops]
#[function_component]
fn Counter(#[prop_or_default] link: &UseLinkHandle<Num>) -> Html {
    let num = use_data(|| Num(0));
    use_bind_link(link.clone(), num.clone());

    html! {
        <div>{num.current().0}</div>
    }
}

#[function_component]
fn App() -> Html {
    let link = use_link();

    html! {
        <div>
            <button onclick={
                let link = link.clone();
                move |_| link.msg(NumMsg::Inc)
            }>{"+"}</button>

            <Counter link={link.clone()} />

            <button onclick={
                let link = link.clone();
                move |_| link.msg(NumMsg::Dec)
            }>{"-"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
