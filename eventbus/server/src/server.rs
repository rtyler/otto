//! Server implementation of eventbus_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use eventbus_api::{Api, ApiError,
                      ListChannelsResponse
};
use eventbus_api::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// List existing channels in the event bus
    fn list_channels(&self, context: &C) -> Box<Future<Item=ListChannelsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("list_channels() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
