use std::collections::HashMap;

use fastly::http::{request, StatusCode};
use fastly::{Error, Request, Response};

use router::Router;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
  let router = Router::new();

  Ok(
    router
      .get("/", |_, _| {
        Ok(Response::from_status(StatusCode::OK).with_body_text_plain("Hello from Rust at the Edge."))
      })
      .get("/params/:param", |_request, ctx| {
        let param = ctx.param("param").unwrap();

        Ok(Response::from_status(StatusCode::OK).with_body_text_plain(param))
      })
      .get("/query/", |request, _ctx| {
        let qs: HashMap<String, String> = request.get_query()?;

        let qs_value = match qs.get("text") {
          Some(text) => text.to_string(),
          _ => String::from(""),
        };

        Ok(Response::from_status(StatusCode::OK).with_body_text_plain(&qs_value))
      })
      .run(req)?,
  )
}
