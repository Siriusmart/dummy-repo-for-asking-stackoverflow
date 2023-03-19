use actix_files::NamedFile;
use actix_web::{middleware::Logger, App, HttpServer, get, Result, post, web::{Json, Path}};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(root)
            .service(api)
            .service(getfile)
    })
    .bind(("127.0.0.1", 8080))
    .expect("cannot bind to port")
    .run()
    .await
    .expect("server down");
}

#[get("/")]
async fn root() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[post("/api")]
async fn api(body: Json<Req>) -> String {
    format!("{body:?}")
}

#[get("/{path:.*}")]
async fn getfile(path: Path<String>) -> Result<NamedFile> {
    Ok(NamedFile::open_async(path.into_inner()).await?)
}

#[derive(Debug, Deserialize)]
struct Req {
    field1: String,
    field2: String,
    field3: String,
}
