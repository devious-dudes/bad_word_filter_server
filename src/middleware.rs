// src/middleware.rs

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::dev::{Transform, Service};
use actix_web::body::EitherBody;
use futures_util::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::cell::RefCell;

pub struct AuthMiddleware {
  token: Option<String>,
}

impl AuthMiddleware {
  pub fn new(token: Option<String>) -> Self {
    AuthMiddleware { token }
  }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthMiddlewareService<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AuthMiddlewareService {
      service: Rc::new(RefCell::new(service)),
      token: self.token.clone(),
    })
  }
}

pub struct AuthMiddlewareService<S> {
  service: Rc<RefCell<S>>,
  token: Option<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.borrow_mut().poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let token = self.token.clone();
    let svc = self.service.clone();
        
    Box::pin(async move {
      if let Some(expected_token) = token {
        if let Some(auth_header) = req.headers().get("Authorization") {
          if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", expected_token) {
              let res = svc.call(req).await?;
              let res = res.map_into_left_body();
              return Ok(res);
            } else {
              // Extract the IP address from the request
              let ip_address = req.peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());

              println!(
                "Authorization failed: IP: {}, auth_str: |{}|",
                ip_address,
                expected_token
              );
            }
          }
        }
        let response = HttpResponse::Unauthorized().finish().map_into_right_body();
        return Ok(ServiceResponse::new(req.into_parts().0, response));
      }
      let res = svc.call(req).await?;
      let res = res.map_into_left_body();
      Ok(res)
    })
  }
}
