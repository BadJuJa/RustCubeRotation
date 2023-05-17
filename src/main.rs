use actix_files as fs;
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

extern crate tera;
use tera::{Context, Tera};

async fn index(
    _req: HttpRequest,
) -> impl Responder {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    context.insert("title", "template");
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}