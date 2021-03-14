use actix_files::NamedFile;
use actix_session::CookieSession;
use actix_web::Result;
use actix_web::{web, App, HttpServer};
use cat_server::foundation::middleware::{
    call_api_counter_middleware, html_404_middleware, validation_error_handler,
};
use cat_server::route::cat;
use env_logger;

async fn top() -> Result<NamedFile> {
    Ok(NamedFile::open("html/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false)) // create cookie based session
            .wrap_fn(html_404_middleware)
            .service(web::resource("/").to(top))
            .service(
                web::scope("/api")
                    .wrap_fn(call_api_counter_middleware)
                    .app_data(
                        actix_web_validator::JsonConfig::default()
                            .error_handler(validation_error_handler),
                    )
                    .service(cat::endpoint_get)
                    .service(cat::endpoint_put),
            )
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
