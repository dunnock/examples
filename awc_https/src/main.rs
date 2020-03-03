use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, client::{Client, Connector}};

async fn index(req: HttpRequest) -> HttpResponse {

	let now = std::time::Instant::now();
    let payload =
        Client::default()
        .get("https://upload.wikimedia.org/wikipedia/commons/f/ff/Pizigani_1367_Chart_10MB.jpg")
        .send()
        .await
        .unwrap()
        .body()
        .limit(20_000_000)  // sets max allowable payload size
        .await
        .unwrap();

    println!("awc time elapsed while reading bytes into memory: {} ms", now.elapsed().as_millis());

	HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(payload)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
