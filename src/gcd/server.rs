use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

use crate::gcd::gcd;

#[derive(Deserialize)]
struct GcdParams {
  m: u64,
  n: u64
}

pub async fn run() {
  let server = HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(get_index))
      .route("/gcd", web::post().to(post_gcd))
  });
  let addr = "127.0.0.1:3000";

  println!("Serving on http://{}", addr);

  server.bind(addr).expect("Error binding server to address")
    .run().await.expect("Error running server");
}

async fn get_index() -> impl Responder {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(
      r#"
        <title>GCD calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="m"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
      "#
    )
}

async fn post_gcd(form: web::Form<GcdParams>) -> impl Responder {
  if form.m == 0 || form.n == 0 {
    return HttpResponse::BadRequest().content_type("text/html").body("Parameters cannot be zero");
  }

  let result = gcd(form.m, form.n);

  let body = format!(
    "The greatest common divisor of the numbers {} and {} is <b>{}</b>",
    form.m, form.n, result
  );

  HttpResponse::Ok().content_type("text/html").body(body)
}
