//! Server implementation of orchestrator_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use orchestrator_api::{Api, ApiError,
                      FetchManifestResponse
};
use orchestrator_api::models;

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

    /// Fetch manifest for execution by the given agent
    fn fetch_manifest(&self, context: &C) -> Box<Future<Item=FetchManifestResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fetch_manifest() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
