use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Rusty Microservice {}!", app_name))
}
#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("We are healthy")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(web::scope("").service(index).service(healthcheck));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    app_name: String::from("Rusty Microservice"),
                })
            )
            .service(index)
            .service(healthcheck)
    })
        .bind("0.0.0.0:8080")?
        .run().await
}
