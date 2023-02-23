use anyhow::Context;
use common::{EnqueueRequest, EnqueueRequestReply, MusicQueuePreview, Song};
use js_sys::eval as js_eval;
use seed::{prelude::*, *};
use wasm_bindgen_futures::spawn_local;
// use gloo_net::http::Request;

fn init(_: Url, o: &mut impl Orders<Msg>) -> Model {
    let sender = o.send_msg(Msg::RefreshQueuePreview);
    Model {
        youtube_url: "".to_owned(),
        music_queue_preview: None,
        last_enqueue_request_reply: None,
    }
}

struct Model {
    youtube_url: String,
    music_queue_preview: Option<Box<MusicQueuePreview>>,
    last_enqueue_request_reply: Option<Box<EnqueueRequestReply>>,
}

#[derive(Debug, Clone)]
enum Msg {
    InputValue(String),
    Submit,
    RefreshQueuePreview,
    DisplayNewQueuePreview(Box<MusicQueuePreview>),
}

type MsgSender = std::rc::Rc<dyn Fn(Option<Msg>)>;

fn update(msg: Msg, model: &mut Model, o: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => {
            log!("Inside Submit handler.");
            let sender = o.skip().msg_sender();
            run_submit(sender,model.youtube_url.clone());
        }
        Msg::InputValue(url) => {
            log!("Handling URL change: ", &url);
            o.skip();
            model.youtube_url = url;
        }
        Msg::RefreshQueuePreview => {
            let sender = o.skip().msg_sender();
            run_refresh_queue(sender);
            
        }
        Msg::DisplayNewQueuePreview(preview) => {
            model.music_queue_preview = Some(preview);
            // This happens by default
            // o.render();
        }
    }
}

fn get_endpoint_base() -> anyhow::Result<String> {
    let endpoint = js_eval(
        r#"
            window.location.hostname
        "#,
    );
    let Ok(Some(endpoint)) = endpoint.map(|x| x.as_string()) else {
        anyhow::bail!("'window.location.host' could not be evaluated to a string.");
    };
    log!("Got endpoint from JS: ", &endpoint);
    let endpoint = endpoint.replace("?", "");
    //let mut endpoint = endpoint.replace("8080", "8090");
    //endpoint.push_str(":8090/enqueue");
    let endpoint = format!("http://{}:8090", endpoint);
    Ok(endpoint)
}

fn run_refresh_queue(msg_sender: MsgSender) {
    let inner = move ||{
        // spawn_local(async move {
        //     sender(Some(Msg::Submit));
        // });
        anyhow::Ok(())
    };
    if let Err(e) = inner() {
        log!(e);
    }
}

fn run_submit(msg_sender: MsgSender, url: String) {
    let inner = move || {
        let endpoint = format!("{}/enqueue", get_endpoint_base()?);
        log!("[Submit] Final endpoint: ", &endpoint);
        let post = Request::new(endpoint)
            .method(Method::Post)
            .json(&EnqueueRequest { url });
        match post {
            Ok(r) => {
                spawn_local(async move {
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
                anyhow::bail!("Failed to create POST request: {:?}", e)
            }
        }
        anyhow::Ok(())
    };
    if let Err(e) = inner() {
        log!(e);
    }
}

fn view(model: &Model) -> Node<Msg> {
    let show_song = |song: &Song| {
        div![
            p!["Todo: image"],
            div![
                p![song.title.clone().unwrap_or_default()],
                i![&song.url],
                i![format!("Seconds of length: {}", song.duration.unwrap_or(0))]
            ]
        ]
    };
    let queue_preview = {
        match &model.music_queue_preview {
            Some(queue_preview) => {
                let queue = &queue_preview.queue;
                let mut queued_songs = Vec::new();
                for i in &queue.queue {
                    queued_songs.push(show_song(i));
                }
                div![
                    C!["column"],
                    div![
                        id!["current_song"],
                        if let Some(cp) = &queue.currently_played {
                            show_song(cp)
                        } else {
                            i!["Obecnie nic nie jest odtwarzane"]
                        }
                    ],
                    div![id!["queued_songs"], queued_songs],
                    i!["Todo: total length"]
                ]
            }
            None => {
                i!["Nie ma nic do pokazania"]
            }
        }
    };
    let message_box_content = {
        match &model.last_enqueue_request_reply {
            Some(reply) => {
                let ct = match reply.error_message.as_ref() {
                    Some(error_msg) => {
                        b![format!("Error occured: {}", error_msg)]
                    }
                    None => {
                        div![
                            C!["column"],
                            h3!["Your song has been added to the queue."],
                            p![format!(
                                "Position in queue: {}",
                                reply.pos_in_queue.unwrap_or(0)
                            )],
                            p![format!("TTW: {}", reply.time_to_wait.unwrap_or(0))],
                            if let Some(s) = &reply.song_info.as_ref() {
                                show_song(s)
                            } else {
                                i!["No song info"]
                            }
                        ]
                    }
                };
                div![id!["message_box_content"], ct]
            }
            None => {
                // Meant to be empty
                p![]
            }
        }
    };
    div![
        C!["column"],
        // Encompasses the whole screen
        id!["top_frame"],
        div![
            // Contains the form and the message area
            id!["url_frame"],
            C!["spaced"],
            C!["column"],
            C!["in_border"],
            h1![C!["spaced"], "Dodaj do kolejki"],
            div![
                // Contains the form
                id!["input_frame"],
                C!["in_border"],
                C!["column"],
                C!["spaced"],
                h3![C!["spaced"], "Wklej jakiś link z YouTube."],
                div![
                    C!["row"],
                    C!["spaced"],
                    input![
                        C!["spaced"],
                        &model.youtube_url,
                        input_ev(Ev::Input, |v: String| Msg::InputValue(v))
                    ],
                    button![
                        C!["spaced"],
                        "Zleć dodanie do kolejki",
                        ev(Ev::Click, |_| Msg::Submit)
                    ]
                ],
            ],
            message_box_content
        ],
        div![
            id!["queue_frame"],
            C!["column"],
            C!["spaced"],
            C!["in_border"],
            h1![C!["spaced"], "Kolejka",],
            button![
                C!["spaced"],
                C!["to_the_right_inside_flex"],
                "Odśwież podgląd kolejki",
                ev(Ev::Click, |_| Msg::RefreshQueuePreview)
            ],
            div![
                id!["queue_preview_frame"],
                C!["column"],
                C!["spaced"],
                queue_preview
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
