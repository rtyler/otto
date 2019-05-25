//! Server implementation of eventbus_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use eventbus_api::{Api, ApiError,
                      ChannelNameGetResponse,
                      ChannelNameOffsetGetResponse,
                      ChannelNamePatchResponse,
                      ChannelNamePostResponse,
                      ChannelNamePutResponse,
                      ListChannelsResponse,
                      OffsetConsumerGetResponse,
                      OffsetConsumerPatchResponse,
                      OffsetConsumerPostResponse
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

    /// Fetch the metadata about a specific channel
    fn channel_name_get(&self, name: String, context: &C) -> Box<Future<Item=ChannelNameGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_get(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Fetch an item from the channel
    fn channel_name_offset_get(&self, name: String, offset: i64, context: &C) -> Box<Future<Item=ChannelNameOffsetGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_offset_get(\"{}\", {}) - X-Span-ID: {:?}", name, offset, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Modify the channel configuration
    fn channel_name_patch(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePatchResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_patch(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create a channel
    fn channel_name_post(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_post(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Publish an item to the channel
    fn channel_name_put(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePutResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_put(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// List existing channels in the event bus
    fn list_channels(&self, context: &C) -> Box<Future<Item=ListChannelsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("list_channels() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// List offset metadata about a named consumer
    fn offset_consumer_get(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("offset_consumer_get(\"{}\") - X-Span-ID: {:?}", consumer, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Update the offset for the named consumer
    fn offset_consumer_patch(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerPatchResponse, Error=ApiError>> {
        let context = context.clone();
        println!("offset_consumer_patch(\"{}\") - X-Span-ID: {:?}", consumer, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create a named consumer to store metadata
    fn offset_consumer_post(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("offset_consumer_post(\"{}\") - X-Span-ID: {:?}", consumer, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
