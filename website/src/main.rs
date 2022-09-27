use yew::prelude::*;
use yew::TargetCast;
use web_sys::HtmlInputElement;
use gloo_console::log;
use gloo_net::http::Request;
use serde::Serialize;
use js_sys::eval as js_eval;

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
                <button id="przechuj" type="button" onclick={onclick}>{"Zleć dodanie do kolejki"}</button>
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
                let endpoint = js_eval(r#"
                    window.location.hostname
                "#);
                match endpoint {
                    Ok(endpoint_value) => {
                        if let Some(endpoint) = endpoint_value.as_string() {
                            log!("Got endpoint from JS: ",&endpoint);
                            let mut endpoint = endpoint.replace("?", "");
                            //let mut endpoint = endpoint.replace("8080", "8090");
                            //endpoint.push_str(":8090/enqueue");
                            let endpoint = format!("http://{}:8090/enqueue",endpoint);
                            log!("Final endpoint: ",&endpoint);
                            let post = Request::post(&endpoint)
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
                                                log!("Failed to send request: ",e.to_string());
                                            }
                                        }
                                        log!("POST-sending future completes.");
                                    });
                                }
                                Err(e) => {
                                    log!("Failed to create POST request: ",e.to_string());
                                }
                            }
                        } else {
                            log!("Ni chuja: JsValue to nie String");
                        }
                    }
                    Err(e) => {
                        log!("No i chuj: window.location.host nie teges")
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
            <h2>{"Wklej jakiś link z YouTube."}</h2>
            <Form/>
            <p>{"Po kliknięciu na guzior nie pojawi się żaden komunikat jak coś."}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}