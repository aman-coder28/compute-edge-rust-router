## Simple Edge Router

> A port of [Workers-rs Router](https://github.com/cloudflare/workers-rs/blob/main/worker/src/router.rs)

**Work-in-progress** ergonomic Router for Fastly Compute@Edge Applications that's using Rust

Parameterize routes and access the parameter values from within a handler. Each handler function takes a
`Request`, and a `RouteContext`. The `RouteContext` has route params.

```rust
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use router::Router;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
  let router = Router::new();

  Ok(
    router
      .on("/", |_, _| {
        Ok(Response::from_status(StatusCode::OK).with_body_text_plain("Hello from Rust at the Edge."))
      })
      .get("/params/:param", |_request, ctx| {
        let param = ctx.param("param").unwrap();

        Ok(Response::from_status(StatusCode::OK).with_body_text_plain(param))
      })
      .get("/query/", |request, _ctx| {
        let qs: HashMap<String, String> = request.get_query()?;

        let qs_value =qs.get("text") {
          Some(text) => text.to_string(),
          _ => String::from(""),
        };

        Ok(Response::from_status(StatusCode::OK).with_body_text_plain(&qs_value))
      })
      .run(req)?,
  )
}
```


