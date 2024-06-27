use crate::*;

// pub mod api_proxy;
// pub mod cron;
// pub mod my_submissions;
// pub mod my_userboard;
// pub mod problem_png;
// pub mod submission;
// pub mod submissions;
pub mod template;
// pub mod visualize;

use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render("Hello, world!"))
}
