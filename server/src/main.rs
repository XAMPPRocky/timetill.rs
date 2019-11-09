use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

fn main() {
    eprintln!(
        "Loading environment variables from '{}'.",
        dotenv::dotenv().unwrap().display()
    );
    env_logger::init();

    let pg = {
        let manager = diesel::r2d2::ConnectionManager::new(&**server::env::DATABASE_URL);
        r2d2::Pool::builder().build(manager).unwrap()
    };

    let redis = {
        let manager = r2d2_redis::RedisConnectionManager::new(&**server::env::REDIS_URL).unwrap();
        r2d2::Pool::builder().build(manager).unwrap()
    };
    let http = reqwest::Client::new();

    HttpServer::new(move || {
        let http = http.clone();
        let pg = pg.clone();
        let redis = redis.clone();

        App::new()
            .register_data(web::Data::new(server::Clients { http, redis, pg }))
            .wrap(CookieSession::signed(&server::env::SECRET_KEY).secure(false))
            .wrap(actix_cors::Cors::new())
            .wrap(actix_web::middleware::Logger::default())
            .route("/authorise", web::get().to(server::users::authorise))
            .service(
                web::resource("/events")
                    .route(web::get().to(server::events::list))
                    .route(web::post().to(server::events::create)),
            )
            .route("/events/{slug}", web::get().to(server::events::get))
            .route(
                "/events/{slug}/attend",
                web::get().to(server::events::attend),
            )
            .route("/user", web::get().to(server::users::current))
            .route(
                "/add-reviewer/{github_id}",
                web::get().to(server::users::add_reviewer),
            )
            .route("/login", web::get().to(server::users::login))
    })
    .bind("0.0.0.0:5000")
    .expect("Can not bind to port 5000")
    .run()
    .unwrap();
}
