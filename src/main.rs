use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn failing(_req: HttpRequest) -> Result<String, Error> {
//     Err(io::Error::new(io::ErrorKind::Other, "An error happens here").into())
// }

#[get("*")]
async fn denied() -> impl Responder {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            // ip_address: "{{auto}}",
            ..Default::default()
        }));
    });

    HttpResponse::Ok().body("access denied")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _guard = sentry::init((
        "https://6d05fde40fc64e5aac12f5d809576bfa@o476480.ingest.sentry.io/5777758",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            max_breadcrumbs: 50,
            ..Default::default()
        },
    ));
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(|| App::new().wrap(sentry_actix::Sentry::new()).service(denied))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
