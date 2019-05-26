#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "1.0.0";


#[derive(Debug, PartialEq)]
pub enum ChannelGetResponse {
    /// Channels successfully listed
    ChannelsSuccessfullyListed ( Vec<models::Channel> ) ,
    /// Invalid request
    InvalidRequest ,
}

#[derive(Debug, PartialEq)]
pub enum ChannelNameGetResponse {
    /// Successful retrieval of metadata
    SuccessfulRetrievalOfMetadata ( models::Channel ) ,
    /// Invalid formatted channel name or request
    InvalidFormattedChannelNameOrRequest ,
    /// User is not authorized to access the channel
    UserIsNotAuthorizedToAccessTheChannel ,
    /// Could not find the named channel
    CouldNotFindTheNamedChannel ,
}

#[derive(Debug, PartialEq)]
pub enum ChannelNameOffsetGetResponse {
    /// Successful fetch of the item
    SuccessfulFetchOfTheItem ,
    /// Could not find the named channel
    CouldNotFindTheNamedChannel ,
    /// Could not find an item at the given offset
    CouldNotFindAnItemAtTheGivenOffset ,
}

#[derive(Debug, PartialEq)]
pub enum ChannelNamePatchResponse {
    /// Successful update of the channel
    SuccessfulUpdateOfTheChannel ,
    /// Suggested channel configuration was invalid
    SuggestedChannelConfigurationWasInvalid ,
    /// User is not authorized to modify the channel
    UserIsNotAuthorizedToModifyTheChannel ,
    /// Could not find the named channel
    CouldNotFindTheNamedChannel ,
}

#[derive(Debug, PartialEq)]
pub enum ChannelNamePostResponse {
    /// Channel created successfully
    ChannelCreatedSuccessfully ,
    /// Suggested channel configuration was invalid
    SuggestedChannelConfigurationWasInvalid ,
    /// User is not authorized to create a channel
    UserIsNotAuthorizedToCreateAChannel ,
}

#[derive(Debug, PartialEq)]
pub enum ChannelNamePutResponse {
    /// Successful publish of the item
    SuccessfulPublishOfTheItem ,
    /// User is not authorized to publish to the channel
    UserIsNotAuthorizedToPublishToTheChannel ,
    /// Could not find the named channel
    CouldNotFindTheNamedChannel ,
}

#[derive(Debug, PartialEq)]
pub enum OffsetConsumerGetResponse {
    /// Successful access of the consumer metadata
    SuccessfulAccessOfTheConsumerMetadata ,
    /// Improperly formatted consumer name
    ImproperlyFormattedConsumerName ,
    /// User is not authorized to access this consumer
    UserIsNotAuthorizedToAccessThisConsumer ,
    /// Could not find the named consumer
    CouldNotFindTheNamedConsumer ,
}

#[derive(Debug, PartialEq)]
pub enum OffsetConsumerPatchResponse {
    /// Successful modification of the consumer metadata
    SuccessfulModificationOfTheConsumerMetadata ,
    /// Improperly formatted metadata
    ImproperlyFormattedMetadata ,
    /// User is not authorized to modify this consumer
    UserIsNotAuthorizedToModifyThisConsumer ,
    /// Could not find the named consumer
    CouldNotFindTheNamedConsumer ,
}

#[derive(Debug, PartialEq)]
pub enum OffsetConsumerPostResponse {
    /// Successful creation of the named consumer
    SuccessfulCreationOfTheNamedConsumer ,
    /// Improperly formatted consumer metadata
    ImproperlyFormattedConsumerMetadata ,
    /// User is not authorized to create a consumer
    UserIsNotAuthorizedToCreateAConsumer ,
    /// The named consumer already exists and is in use
    TheNamedConsumerAlreadyExistsAndIsInUse ,
}


/// API
pub trait Api<C> {

    /// List existing channels in the event bus
    fn channel_get(&self, context: &C) -> Box<Future<Item=ChannelGetResponse, Error=ApiError>>;

    /// Fetch the metadata about a specific channel
    fn channel_name_get(&self, name: String, context: &C) -> Box<Future<Item=ChannelNameGetResponse, Error=ApiError>>;

    /// Fetch an item from the channel
    fn channel_name_offset_get(&self, name: String, offset: i64, context: &C) -> Box<Future<Item=ChannelNameOffsetGetResponse, Error=ApiError>>;

    /// Modify the channel configuration
    fn channel_name_patch(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePatchResponse, Error=ApiError>>;

    /// Create a channel
    fn channel_name_post(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePostResponse, Error=ApiError>>;

    /// Publish an item to the channel
    fn channel_name_put(&self, name: String, context: &C) -> Box<Future<Item=ChannelNamePutResponse, Error=ApiError>>;

    /// List offset metadata about a named consumer
    fn offset_consumer_get(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerGetResponse, Error=ApiError>>;

    /// Update the offset for the named consumer
    fn offset_consumer_patch(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerPatchResponse, Error=ApiError>>;

    /// Create a named consumer to store metadata
    fn offset_consumer_post(&self, consumer: String, context: &C) -> Box<Future<Item=OffsetConsumerPostResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// List existing channels in the event bus
    fn channel_get(&self) -> Box<Future<Item=ChannelGetResponse, Error=ApiError>>;

    /// Fetch the metadata about a specific channel
    fn channel_name_get(&self, name: String) -> Box<Future<Item=ChannelNameGetResponse, Error=ApiError>>;

    /// Fetch an item from the channel
    fn channel_name_offset_get(&self, name: String, offset: i64) -> Box<Future<Item=ChannelNameOffsetGetResponse, Error=ApiError>>;

    /// Modify the channel configuration
    fn channel_name_patch(&self, name: String) -> Box<Future<Item=ChannelNamePatchResponse, Error=ApiError>>;

    /// Create a channel
    fn channel_name_post(&self, name: String) -> Box<Future<Item=ChannelNamePostResponse, Error=ApiError>>;

    /// Publish an item to the channel
    fn channel_name_put(&self, name: String) -> Box<Future<Item=ChannelNamePutResponse, Error=ApiError>>;

    /// List offset metadata about a named consumer
    fn offset_consumer_get(&self, consumer: String) -> Box<Future<Item=OffsetConsumerGetResponse, Error=ApiError>>;

    /// Update the offset for the named consumer
    fn offset_consumer_patch(&self, consumer: String) -> Box<Future<Item=OffsetConsumerPatchResponse, Error=ApiError>>;

    /// Create a named consumer to store metadata
    fn offset_consumer_post(&self, consumer: String) -> Box<Future<Item=OffsetConsumerPostResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {

    /// List existing channels in the event bus
    fn channel_get(&self) -> Box<Future<Item=ChannelGetResponse, Error=ApiError>> {
        self.api().channel_get(&self.context())
    }

    /// Fetch the metadata about a specific channel
    fn channel_name_get(&self, name: String) -> Box<Future<Item=ChannelNameGetResponse, Error=ApiError>> {
        self.api().channel_name_get(name, &self.context())
    }

    /// Fetch an item from the channel
    fn channel_name_offset_get(&self, name: String, offset: i64) -> Box<Future<Item=ChannelNameOffsetGetResponse, Error=ApiError>> {
        self.api().channel_name_offset_get(name, offset, &self.context())
    }

    /// Modify the channel configuration
    fn channel_name_patch(&self, name: String) -> Box<Future<Item=ChannelNamePatchResponse, Error=ApiError>> {
        self.api().channel_name_patch(name, &self.context())
    }

    /// Create a channel
    fn channel_name_post(&self, name: String) -> Box<Future<Item=ChannelNamePostResponse, Error=ApiError>> {
        self.api().channel_name_post(name, &self.context())
    }

    /// Publish an item to the channel
    fn channel_name_put(&self, name: String) -> Box<Future<Item=ChannelNamePutResponse, Error=ApiError>> {
        self.api().channel_name_put(name, &self.context())
    }

    /// List offset metadata about a named consumer
    fn offset_consumer_get(&self, consumer: String) -> Box<Future<Item=OffsetConsumerGetResponse, Error=ApiError>> {
        self.api().offset_consumer_get(consumer, &self.context())
    }

    /// Update the offset for the named consumer
    fn offset_consumer_patch(&self, consumer: String) -> Box<Future<Item=OffsetConsumerPatchResponse, Error=ApiError>> {
        self.api().offset_consumer_patch(consumer, &self.context())
    }

    /// Create a named consumer to store metadata
    fn offset_consumer_post(&self, consumer: String) -> Box<Future<Item=OffsetConsumerPostResponse, Error=ApiError>> {
        self.api().offset_consumer_post(consumer, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
