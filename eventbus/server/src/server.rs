//! Server implementation of eventbus_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use eventbus_api::{Api, ApiError,
                      ChannelGetResponse,
                      ChannelNameGetResponse,
                      ChannelNameOffsetGetResponse,
                      ChannelNamePatchResponse,
                      ChannelNamePostResponse,
                      ChannelNamePutResponse,
                      OffsetConsumerGetResponse,
                      OffsetConsumerPatchResponse,
                      OffsetConsumerPostResponse
};
use eventbus_api::models;

//#[derive(Copy, Clone)]
#[derive(Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
    channels: HashMap<String, models::Channel>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
            channels: HashMap::new(),
        }
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{
    /// List existing channels in the event bus
    fn channel_get(&self, context: &C) -> Box<Future<Item=ChannelGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Fetch the metadata about a specific channel
    fn channel_name_get(&self, name: String, context: &C) -> Box<Future<Item=ChannelNameGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("channel_name_get(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());

        if !self.channels.contains_key(&name) {
            println!("Channel does not exist");
            return Box::new(
                futures::future::ok(
                    ChannelNameGetResponse::CouldNotFindTheNamedChannel));
        }

        Box::new(
            futures::future::ok(
                ChannelNameGetResponse::SuccessfulRetrievalOfMetadata(self.channels[&name].clone())
                ))
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
