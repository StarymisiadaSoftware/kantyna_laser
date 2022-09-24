use yew::prelude::*;
use yew::TargetCast;
use web_sys::HtmlInputElement;
use gloo_console::log;
use gloo_net::http::Request;
use serde::Serialize;

#[derive(Debug,Serialize)]
struct EnqueueRequest {
    url: String
}


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
        let onchange = link.callback(|e: Event| {
            log!("Inside event handler");
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            log!("sending value: ",&value);
            Msg::InputValue(value)
        });
        let onclick = link.callback(|_e: MouseEvent| {
            Msg::Submit
        });
        html! {
            <form>
            <label for="dangerous-input">
            { "Link do filmu:" }
            <input onchange={onchange}
                id="dangerous-input"
                type="text"
                value={self.url.clone()}
            />
            </label>
                <button id="przechuj" onclick={onclick}>{"Zleć dodanie do kolejki"}</button>
            </form>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(url) => {
                log!("Handling URL change: ",&url);
                self.url = url;
            }
            Msg::Submit => {
                log!("Inside Submit handler.");

                let post = Request::post("http://192.168.0.20:8090/enqueue")
                    .json(&EnqueueRequest{ url: self.url.clone() });
                match post {
                    Ok(r) => {
                        wasm_bindgen_futures::spawn_local(async move {
                            log!("Hello from POST-sending future.");
                            match r.send().await {
                                Ok(res) => {
                                    log!("Got response: ",res.status_text());
                                }
                                Err(e) => {
                                    log!("fuck: ",e.to_string());
                                }
                            }
                            log!("POST-sending future completes.");
                        });
                    }
                    Err(e) => {
                        log!("fuck: ",e.to_string());
                    }
                }
            }
        }
        true
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