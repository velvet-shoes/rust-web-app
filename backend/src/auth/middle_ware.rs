use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use actix_web::{
          Error, HttpMessage,
          dev::{Service, ServiceRequest, ServiceResponse, Transform}
     };
     
use crate::{auth::token::tokend_data_extractor};
     
pub struct MiddleWareJwT{
     secret: String
}

impl MiddleWareJwT{
    pub fn new(secret: String) -> Self{
          Self { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for MiddleWareJwT
where 
     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
     B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MiddleWareJwTService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MiddleWareJwTService {
          service: Rc::new(service),
          secret: self.secret.clone(),
        })
    } 
}

pub struct MiddleWareJwTService<S>{
     service: Rc<S>,
     secret: String,
}

impl<S, B> Service<ServiceRequest> for MiddleWareJwTService<S>
where 
     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
     B: 'static,
{
     type Response = ServiceResponse<B>;
     type Error = Error;
     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

     fn poll_ready(&self, ctx: &mut core::task::Context<'_>)
     -> std::task::Poll<Result<(), Self::Error>> {
          self.service.poll_ready(ctx)
     }

     fn call(&self, req: ServiceRequest) -> Self::Future {
         let service = self.service.clone();
         let secret = self.secret.clone();
         Box::pin(async move {
               let token_opt = req
               .headers()
               .get("Authorization")
               .map(|h| h.to_str().unwrap().split_at(7).1.to_string());
          
               let dec_jwt = tokend_data_extractor(token_opt.expect("fail to extract header"), &secret).await;
               match dec_jwt {
                    Ok(data) => req.extensions_mut().insert(data.claims),
                    Err(_e) => return Err(actix_web::error::ErrorUnauthorized("Invalid token"))
               };
               
          service.call(req).await
         })
     }
}

