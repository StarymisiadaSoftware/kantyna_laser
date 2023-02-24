use common::{EnqueueRequest, EnqueueRequestReply, MusicQueuePreview, Song, BACKEND_PORT};
use gloo_timers::future::TimeoutFuture;
use js_sys::eval as js_eval;
use seed::{prelude::*, *};
use serde_json::from_str;
use wasm_bindgen_futures::spawn_local;
// use gloo_net::http::Request;

fn init(_: Url, o: &mut impl Orders<Msg>) -> Model {
    o.send_msg(Msg::RefreshQueuePreview);
    let msg_sender = o.msg_sender();
    spawn_local(async move {
        log!("[Auto Queue Refresh Loop] Started automatic queue refresh loop.");
        loop {
            TimeoutFuture::new(60_000).await;
            log!("[Auto Queue Refresh Loop] Issuing automatic queue refresh.");
            msg_sender(Some(Msg::RefreshQueuePreview));
        }
    });
    Model {
        youtube_url: "".to_owned(),
        music_queue_preview: None,
        message_box_content: MessageBoxContent::Nothing,
    }
}

enum MessageBoxContent {
    Nothing,
    EnqueueRequestReply(Box<EnqueueRequestReply>),
    LoadingScreen,
}

struct Model {
    youtube_url: String,
    music_queue_preview: Option<Box<MusicQueuePreview>>,
    message_box_content: MessageBoxContent,
}

#[derive(Debug, Clone)]
enum Msg {
    InputValue(String),
    Submit,
    RefreshQueuePreview,
    ShowEnqueueProcessingScreen,
    DisplayNewQueuePreview(Box<MusicQueuePreview>),
    DisplayEnqueueRequestReply(Box<EnqueueRequestReply>),
}

type MsgSender = std::rc::Rc<dyn Fn(Option<Msg>)>;

fn update(msg: Msg, model: &mut Model, o: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => {
            log!("Inside Submit handler.");
            let sender = o.skip().msg_sender();
            run_submit(sender, model.youtube_url.clone());
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
        Msg::DisplayEnqueueRequestReply(reply) => {
            model.message_box_content = MessageBoxContent::EnqueueRequestReply(reply);
            // This happens by default
            // o.render();
        }
        Msg::ShowEnqueueProcessingScreen => {
            model.message_box_content = MessageBoxContent::LoadingScreen;
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
    let endpoint = format!("http://{}:{}", endpoint, BACKEND_PORT);
    Ok(endpoint)
}

fn pretty_print_seconds(seconds_total: u32) -> String {
    let minutes_total = seconds_total / 60;
    let hours_total = minutes_total / 60;
    let days = hours_total / 24;
    let hours = hours_total - days * 24;
    let minutes = minutes_total - hours_total * 60;
    let seconds = seconds_total - minutes_total * 60;

    let mut ret = String::new();

    if days > 0 {
        ret.push_str(&format!("{} dni ", days));
    }
    if hours > 0 {
        ret.push_str(&format!("{} godzin ", hours));
    }
    if minutes > 0 {
        ret.push_str(&format!("{} minut ", minutes));
    }
    ret.push_str(&format!("{} sekund ", seconds));
    ret
}

fn run_refresh_queue(msg_sender: MsgSender) {
    let inner = move || {
        let endpoint = format!("{}/preview_queue", get_endpoint_base()?);
        log!("[Refresh Queue] Final endpoint: ", &endpoint);
        spawn_local(async move {
            log!("[Refresh Queue] Sending GET request...");
            let rq = Request::new(endpoint).method(Method::Get).fetch();
            match rq.await {
                Ok(res) => {
                    let text = res.text().await;
                    log!("[Refresh Queue] Got response: ", &text);
                    match text.map(|txt| from_str::<MusicQueuePreview>(&txt)) {
                        Ok(Ok(reply)) => {
                            log!("[Refresh Queue] Reply has been deserialized successfully.");
                            msg_sender(Some(Msg::DisplayNewQueuePreview(Box::from(reply))));
                        }
                        Ok(Err(se)) => {
                            let e = format!("Failed to deserialize reply: {:?}", se);
                            log!("[Refresh Queue] ", e);
                            let reply = EnqueueRequestReply::from_err_msg(&e);
                            msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(reply))));
                        }
                        Err(e) => {
                            let e = format!("Failed to get a response: {:?}", e);
                            log!("[Refresh Queue] ", e);
                            let reply = EnqueueRequestReply::from_err_msg(&e);
                            msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(reply))));
                        }
                    }
                }
                Err(e) => {
                    let e = format!("Failed to send request: {:?}", e);
                    log!("[Refresh Queue] ", e);
                    let reply = EnqueueRequestReply::from_err_msg(&e);
                    msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(reply))));
                }
            }
            log!("[Refresh Queue] GET-sending future completes.");
        });

        anyhow::Ok(())
    };
    if let Err(e) = inner() {
        log!("[Refresh Queue] ", e);
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
                    msg_sender(Some(Msg::ShowEnqueueProcessingScreen));
                    log!("[Submit] Sending POST request...");
                    match r.fetch().await {
                        Ok(res) => {
                            let text = res.text().await;
                            log!("[Submit] Got response: ", &text);
                            match text.map(|txt| from_str::<EnqueueRequestReply>(&txt)) {
                                Ok(Ok(reply)) => {
                                    log!("[Submit] Reply has been deserialized successfully.");
                                    msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(
                                        reply,
                                    ))));
                                    spawn_local(async move {
                                        TimeoutFuture::new(1_500).await;
                                        msg_sender(Some(Msg::RefreshQueuePreview));
                                    });
                                }
                                Ok(Err(se)) => {
                                    let e = format!("Failed to deserialize reply: {:?}", se);
                                    log!("[Submit] ", e);
                                    let reply = EnqueueRequestReply::from_err_msg(&e);
                                    msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(
                                        reply,
                                    ))));
                                }
                                Err(e) => {
                                    let e = format!("Failed to get a response: {:?}", e);
                                    log!("[Submit] ", e);
                                    let reply = EnqueueRequestReply::from_err_msg(&e);
                                    msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(
                                        reply,
                                    ))));
                                }
                            }
                        }
                        Err(e) => {
                            let e = format!("Failed to send request: {:?}", e);
                            log!("[Submit] ", e);
                            let reply = EnqueueRequestReply::from_err_msg(&e);
                            msg_sender(Some(Msg::DisplayEnqueueRequestReply(Box::from(reply))));
                        }
                    }
                    log!("[Submit] POST-sending future completes.");
                });
            }
            Err(e) => {
                anyhow::bail!("Failed to create POST request: {:?}", e)
            }
        }
        anyhow::Ok(())
    };
    if let Err(e) = inner() {
        log!("[Submit] ", e);
    }
}

fn view(model: &Model) -> Node<Msg> {
    let show_song = |song: &Song| {
        div![
            C!["spaced"],
            C!["row"],
            C!["curly_border"],
            C!["in_border"],
            C!["in_padding"],
            C!["nice_background"],
            C!["song_info"],
            match &song.thumbnail_url {
                Some(m) => {
                    img![
                        attrs!(At::Src => m),
                        style!(St::MaxWidth => "100px", St::MaxHeight => "100px", St::MarginRight => "5px")
                    ]
                    .into_nodes()
                }
                None => {
                    i!["No image"].into_nodes()
                }
            },
            div![
                C!["column"],
                a![
                    attrs! {At::Href => &song.url},
                    p![b![
                        style!(St::Color => "blueviolet"),
                        song.title.clone().unwrap_or_default()
                    ]]
                ],
                i![format!(
                    "Długość: {}",
                    pretty_print_seconds(song.duration.unwrap_or(0) as u32)
                )]
            ]
        ]
    };
    let queue_preview = {
        match &model.music_queue_preview {
            Some(queue_preview) => {
                let queue = &queue_preview.queue;
                let mut total_queue_length: u32 = 0;
                let mut queued_songs = Vec::new();
                for i in &queue.queue {
                    queued_songs.push(show_song(i));
                    total_queue_length += i.duration.unwrap_or(0) as u32
                }
                total_queue_length += queue
                    .currently_played
                    .as_ref()
                    .and_then(|x| x.duration)
                    .unwrap_or(0) as u32;
                div![
                    C!["column"],
                    h3!["Obecnie odtwarzane"],
                    div![
                        id!["current_song"],
                        if let Some(cp) = &queue.currently_played {
                            show_song(cp)
                        } else {
                            i!["Obecnie nic nie jest odtwarzane"]
                        }
                    ],
                    h3!["Następnie"],
                    div![
                        id!["queued_songs"],
                        C!["spaced"],
                        if queued_songs.is_empty() {
                            i!["Nie ma dalej nic w kolejce"].into_nodes()
                        } else {
                            queued_songs.into_nodes()
                        }
                    ],
                    i![format!(
                        "Całkowita długość: {}",
                        pretty_print_seconds(total_queue_length)
                    )]
                ]
            }
            None => {
                i!["Nie ma nic do pokazania"]
            }
        }
    };
    let message_box_content = {
        match &model.message_box_content {
            MessageBoxContent::EnqueueRequestReply(reply) => {
                let ct = match reply.error_message.as_ref() {
                    Some(error_msg) => {
                        h3![
                            C!["spaced"],
                            style! {St::Color => "red"},
                            "Wystąpił błąd:",
                            br![],
                            code![error_msg]
                        ]
                    }
                    None => {
                        div![
                            C!["spaced"],
                            C!["column"],
                            h3![
                                style! {St::Color => "green"},
                                "Twoja piosenka została dodana do kolejki."
                            ],
                            p![
                                style!(St::Color => "blueviolet"),
                                format!("Pozycja w kolejce: {}", reply.pos_in_queue.unwrap_or(0))
                            ],
                            p![
                                style!(St::Color => "blueviolet"),
                                format!(
                                    "Szacowany czas oczekiwania: {}",
                                    pretty_print_seconds(reply.time_to_wait.unwrap_or(0))
                                )
                            ],
                            if let Some(s) = &reply.song_info.as_ref() {
                                show_song(s)
                            } else {
                                i![
                                    style!(St::Color => "blueviolet"),
                                    "Brak informacji o piosence"
                                ]
                            }
                        ]
                    }
                };
                ct
            }
            MessageBoxContent::LoadingScreen => {
                h3![C!["spaced"], style!(St::Color => "pink"), "Ładowanie..."]
            }
            MessageBoxContent::Nothing => {
                empty![]
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
                C!["curly_border"],
                C!["column"],
                C!["spaced"],
                C!["nice_background"],
                h3![C!["spaced"], "Wklej jakiś link z YouTube."],
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
                style! {St::FontSize => "1.4rem"},
                "↻",
                ev(Ev::Click, |_| Msg::RefreshQueuePreview)
            ],
            div![
                id!["queue_preview_frame"],
                C!["column"],
                C!["spaced"],
                queue_preview
            ]
        ],
        div![
            id!["bottom_frame"],
            C!["column"],
            C!["spaced"],
            C!["in_border"],
            b![C!["spaced"], "Kantyna Laser"],
            p![
                C!["spaced"],
                "Żeby dobrze grała muzyczka i każdy mógł coś puścić",
                br![],
                small![a![
                    attrs! {At::Href => "https://github.com/StarymisiadaSoftware/kantyna_laser"},
                    style! {St::Color => "white"},
                    "Strona projektu"
                ]]
            ],
            i![
                C!["spaced"],
                "Written by Jakub Smulski, <hgonomeg@gmail.com>",
                br![],
                "Copyright © 2023 Starymisiada Software"
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
