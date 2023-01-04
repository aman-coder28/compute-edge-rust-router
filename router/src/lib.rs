use fastly::http::{Method, StatusCode};
use fastly::{Error, Request, Response};

use matchit::{Match, Node};
use std::collections::HashMap;

type HandlerFn = fn(Request, RouteContext) -> Result<Response, Error>;

pub struct RouteParams(HashMap<String, String>);

impl RouteParams {
  fn get(&self, key: &str) -> Option<&String> {
    self.0.get(key)
  }
}

enum Handler {
  Sync(HandlerFn),
}

impl Clone for Handler {
  fn clone(&self) -> Self {
    match self {
      Self::Sync(func) => Self::Sync(*func),
    }
  }
}

/// A path-based HTTP router supporting exact-match or wildcard placeholders and shared data.
pub struct Router {
  handlers: HashMap<Method, Node<Handler>>,
}

/// Container for a route's parsed parametersata, and environment bindings from the Runtime (such
/// as KV Storesurable Objects, Variables, and Secrets).
pub struct RouteContext {
  params: RouteParams,
}

impl RouteContext {
  /// Get a URL parameter parsed by the router, by the name of its match or wildecard placeholder.
  pub fn param(&self, key: &str) -> Option<&String> {
    self.params.get(key)
  }
}

impl Router {
  /// Construct a new `Router`. Or, call `Router::with_data(D)` to add arbitrary data that will be
  /// available to your various routes.
  pub fn new() -> Self {
    Self::with_data()
  }
}

impl<'a: 'a> Router {
  /// Construct a new `Router` with arbitrary data that will be available to your various routes.
  pub fn with_data() -> Self {
    Self {
      handlers: HashMap::new(),
    }
  }
  /// Register an HTTP handler that will exclusively respond to HEAD requests.
  pub fn head(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::HEAD]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to GET requests.
  pub fn get(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::GET]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to POST requests.
  pub fn post(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::POST]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to PUT requests.
  pub fn put(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::PUT]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to PATCH requests.
  pub fn patch(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::PATCH]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to DELETE requests.
  pub fn delete(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::DELETE]);
    self
  }

  /// Register an HTTP handler that will exclusively respond to OPTIONS requests.
  pub fn options(mut self, pattern: &str, func: HandlerFn) -> Self {
    self.add_handler(pattern, Handler::Sync(func), vec![Method::OPTIONS]);
    self
  }

  fn add_handler(&mut self, pattern: &str, func: Handler, methods: Vec<Method>) {
    for method in methods {
      self
        .handlers
        .entry(method.clone())
        .or_insert_with(Node::new)
        .insert(pattern, func.clone())
        .unwrap_or_else(|e| {
          panic!(
            "failed to register {:?} route for {} pattern: {}",
            method, pattern, e
          )
        });
    }
  }

  /// Handle the request provided to the `Router` and return a `Future`.
  pub fn run(self, req: Request) -> Result<Response, Error> {
    let handlers = self.handlers;

    if let Some(handlers) = handlers.get(&req.get_method()) {
      if let Ok(Match { value, params }) = handlers.at(&req.get_path()) {
        let route_info = RouteContext {
          params: params.into(),
        };

        return match value {
          Handler::Sync(func) => (func)(req, route_info),
        };
      }
    }

    // Needs improvments
    for method in vec![
      Method::GET,
      Method::POST,
      Method::DELETE,
      Method::HEAD,
      Method::OPTIONS,
      Method::CONNECT,
      Method::PATCH,
      Method::TRACE,
    ] {
      if method == Method::GET || method == Method::OPTIONS || method == Method::TRACE {
        continue;
      }

      if let Some(handlers) = handlers.get(&method) {
        if let Ok(Match { .. }) = handlers.at(&req.get_path()) {
          return Ok(
            Response::from_status(StatusCode::METHOD_NOT_ALLOWED).with_body_text_plain("Method Not Allowed"),
          );
        }
      }
    }

    Ok(Response::from_status(StatusCode::NOT_FOUND).with_body_text_plain("404 Not Found"))
  }
}

impl From<matchit::Params<'_, '_>> for RouteParams {
  fn from(p: matchit::Params) -> Self {
    let mut route_params = RouteParams(HashMap::new());
    for (ident, value) in p.iter() {
      route_params.0.insert(ident.into(), value.into());
    }

    route_params
  }
}
