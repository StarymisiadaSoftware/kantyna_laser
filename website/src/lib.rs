use common::EnqueueRequest;
use js_sys::eval as js_eval;
use seed::{prelude::*, *};
// use gloo_net::http::Request;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        youtube_url: "".to_owned(),
    }
}

struct Model {
    youtube_url: String,
}

#[derive(Debug, Clone)]
enum Msg {
    InputValue(String),
    Submit,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => {
            log!("Inside Submit handler.");
            let endpoint = js_eval(
                r#"
                    window.location.hostname
                "#,
            );
            match endpoint {
                Ok(endpoint_value) => {
                    if let Some(endpoint) = endpoint_value.as_string() {
                        log!("Got endpoint from JS: ", &endpoint);
                        let endpoint = endpoint.replace("?", "");
                        //let mut endpoint = endpoint.replace("8080", "8090");
                        //endpoint.push_str(":8090/enqueue");
                        let endpoint = format!("http://{}:8090/enqueue", endpoint);
                        log!("Final endpoint: ", &endpoint);
                        let post =
                            Request::new(endpoint)
                                .method(Method::Post)
                                .json(&EnqueueRequest {
                                    url: model.youtube_url.clone(),
                                });
                        match post {
                            Ok(r) => {
                                wasm_bindgen_futures::spawn_local(async move {
                                    log!("Hello from POST-sending future.");
                                    match r.fetch().await {
                                        Ok(res) => {
                                            log!("Got response: {:?}", res.text().await);
                                        }
                                        Err(e) => {
                                            let e = format!("{:?}", e);
                                            log!("Failed to send request: ", e);
                                        }
                                    }
                                    log!("POST-sending future completes.");
                                });
                            }
                            Err(e) => {
                                let e = format!("{:?}", e);
                                log!("Failed to create POST request: ", e);
                            }
                        }
                    } else {
                        log!("'window.location.host' could not be evaluated to a string.");
                    }
                }
                Err(e) => {
                    log!("Could not evaluate 'window.location.host'.", e);
                }
            }
        }
        Msg::InputValue(url) => {
            log!("Handling URL change: ", &url);
            model.youtube_url = url;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["url_frame"],
        C!["column"],
        h1!["Dodaj do kolejki"],
        h2!["Wklej jakiś link z YouTube."],
        div![
            C!["input_frame"],
            C!["row"],
            input![
                &model.youtube_url,
                input_ev(Ev::Input, |v: String| Msg::InputValue(v))
            ],
            button!["Zleć dodanie do kolejki", ev(Ev::Click, |_| Msg::Submit)]
        ],
        p!["Po kliknięciu na guzior nie pojawi się żaden komunikat jak coś."],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
