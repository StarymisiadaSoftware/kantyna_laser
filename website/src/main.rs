use yew::prelude::*;
use yew::TargetCast;
struct Form {
    url: String,
}

enum Msg {
    InputValue(String),
    Submit
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Form{
            url: String::default()
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onclick = link.callback(|e: Event| {
            Msg::Submit
        });
        html! {
            <form>
                <input type="text" id="twoj_stary" />
                <button id="przechuj" onlick={onclick} >{"Pierdol sie"}</button>
            </form>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="chuj_mnie_strzeli">
            <h1>{"Dodaj do kolejki"}</h1>
            <h2>{"Lorem kurwa ipsum ci w dupe na śniadanie dolor sit amet jego mać"}</h2>
            <Form/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}