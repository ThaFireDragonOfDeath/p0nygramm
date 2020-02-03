#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use actix_files as fs;
use handlebars::Handlebars;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "/home/voldracarno/Schreibtisch/Git/P0nygramm/templates/index.html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .service(fs::Files::new("/static", "./static/webcontent/").index_file("index.html"))
            .service(fs::Files::new("/uploads", "./static/uploads/").index_file("index.html"))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}
