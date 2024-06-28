use crate::*;
use reqwest::Client;
// pub mod api_proxy;
// pub mod cron;
// pub mod my_submissions;
// pub mod my_userboard;
// pub mod problem_png;
// pub mod submission;
// pub mod submissions;
pub mod template;
// pub mod visualize;

use actix_web::{web, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render("Hello, world!<br><a href='/comm'>communicate</a>"))
}
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommQuery {
    #[serde(default)]
    q: String,
    #[serde(default)]
    raw: bool,
}

pub async fn comm(query: web::Query<CommQuery>) -> impl Responder {
    let client = Client::new();
    let res = client
        .post("https://boundvariable.space/communicate")
        .header(
            "Authorization",
            "Bearer 1b2a9024-2287-4eac-a58f-66a33726e529",
        )
        .body(if query.raw {
            query.q.to_owned()
        } else {
            "S".to_owned() + &encode_str(&query.q)
        })
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(&format!(
            r#"
            <form>
                <textarea name="q" placeholder="message" autofocus required cols="160">{}</textarea>
                <div>
                    <input type="checkbox" name="raw" id="raw"{}><label for="raw">raw</label>
                    <button type="submit">Send</button>
                </div>
            </form>
            <textarea placeholder="response" readonly cols="160" rows="10">{}</textarea>
            <textarea placeholder="response" readonly cols="160" rows="50">{}</textarea>
            "#,
            query.q,
            if query.raw { " checked" } else { "" },
            body,
            decode(&body),
        )))
}
