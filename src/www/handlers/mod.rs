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
        .body(www::handlers::template::render(
            "Hello, world!<br><a href='/comm'>communicate</a>",
        ))
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
            <h4>Raw response:</h4>
            <textarea placeholder="raw response" readonly cols="160" rows="10">{}</textarea>
            <h4>Decoded response:</h4>
            <textarea placeholder="response" readonly cols="160" rows="20" id="response">{}</textarea>
            <h4>Rendered response:</h4>
            <section id="rendered" style="font-size:xx-small">
                <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
                <script>
                    try {{
                        let r = document.getElementById("response");
                        let md = r.value.replace(/\[(\w+)\](?!\()/g, "[$1](?q=get+$1)");
                        document.write(marked.parse(md));
                    }} catch (e) {{}}
                </script>
            </section>
            "#,
            query.q,
            if query.raw { " checked" } else { "" },
            body,
            decode(&body),
        )))
}
