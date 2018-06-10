extern crate actix_web;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate serde;

use actix_web::{http, server, App, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use rand::Rng;
use serde::Serialize;

struct State {
  handlebars: Handlebars,
}

fn main() {
  let server_result = server::new(|| {
    App::with_state(State {
      handlebars: init_handlebars(),
    }).resource("/", |r| r.f(index))
  }).bind("0.0.0.0:8080");
  server_result
    .map(|ok| ok.run())
    .expect("Server could not be started!");
}

/**
 * Handle index page.
 */
fn index(_req: HttpRequest<State>) -> HttpResponse {
  let names = vec!["User", "Friend", "Visitor"];
  let random = rand::thread_rng().gen_range(0, names.len());
  respond_with_template(_req, "index", json!({ "name": names[random] }))
}

fn init_handlebars() -> Handlebars {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("index", "./templates/index.hbs")
    .expect("Could not load index template!");

  handlebars
}

fn respond_with_template<T>(
  request: HttpRequest<State>,
  template_name: &'static str,
  data: T,
) -> HttpResponse
where
  T: Serialize,
{
  let body_o = request.state().handlebars.render(&template_name, &data);
  match body_o {
    Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
    _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}
