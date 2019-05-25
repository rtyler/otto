//! Main library entry point for eventbus_api implementation.
#[macro_use]
extern crate error_chain;
extern crate swagger;

mod server;

mod errors {
    error_chain!{}
}

pub use self::errors::*;
use std::io;
use std::clone::Clone;
use std::marker::PhantomData;
use hyper;
use eventbus_api;
use swagger::{Has, XSpanIdString};
use swagger::auth::Authorization;

pub struct NewService<C>{
    marker: PhantomData<C>
}

impl<C> NewService<C>{
    pub fn new() -> Self {
        NewService{marker:PhantomData}
    }
}

impl<C> hyper::server::Service for NewService<C> where C: Has<XSpanIdString>  + Clone + 'static {
    type Request = (hyper::Request, C);
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Instance = eventbus_api::server::Service<server::Server<C>, C>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(eventbus_api::server::Service::new(server::Server::new()))
    }
}
