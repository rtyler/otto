#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;

extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;


use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has};
use swagger::auth::Scopes;

use {Api,
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
#[allow(unused_imports)]
use models;

pub mod auth;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/v1/channel$",
            r"^/v1/channel/(?P<name>[^/?#]*)$",
            r"^/v1/channel/(?P<name>[^/?#]*)/(?P<offset>[^/?#]*)$",
            r"^/v1/offset/(?P<consumer>[^/?#]*)$"
        ]).unwrap();
    }
    pub static ID_CHANNEL: usize = 0;
    pub static ID_CHANNEL_NAME: usize = 1;
    lazy_static! {
        pub static ref REGEX_CHANNEL_NAME: regex::Regex = regex::Regex::new(r"^/v1/channel/(?P<name>[^/?#]*)$").unwrap();
    }
    pub static ID_CHANNEL_NAME_OFFSET: usize = 2;
    lazy_static! {
        pub static ref REGEX_CHANNEL_NAME_OFFSET: regex::Regex = regex::Regex::new(r"^/v1/channel/(?P<name>[^/?#]*)/(?P<offset>[^/?#]*)$").unwrap();
    }
    pub static ID_OFFSET_CONSUMER: usize = 3;
    lazy_static! {
        pub static ref REGEX_OFFSET_CONSUMER: regex::Regex = regex::Regex::new(r"^/v1/offset/(?P<consumer>[^/?#]*)$").unwrap();
    }
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        match &method {

            // ChannelGet - GET /channel
            &hyper::Method::Get if path.matched(paths::ID_CHANNEL) => {







                Box::new({
                        {{

                                Box::new(api_impl.channel_get(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelGetResponse::ChannelsSuccessfullyListed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::CHANNEL_GET_CHANNELS_SUCCESSFULLY_LISTED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                ChannelGetResponse::InvalidRequest


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // ChannelNameGet - GET /channel/{name}
            &hyper::Method::Get if path.matched(paths::ID_CHANNEL_NAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_CHANNEL_NAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHANNEL_NAME in set but failed match against \"{}\"", path, paths::REGEX_CHANNEL_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter name: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.channel_name_get(param_name, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelNameGetResponse::SuccessfulRetrievalOfMetadata

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::CHANNEL_NAME_GET_SUCCESSFUL_RETRIEVAL_OF_METADATA.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                ChannelNameGetResponse::InvalidFormattedChannelNameOrRequest


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                ChannelNameGetResponse::UserIsNotAuthorizedToAccessTheChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                ChannelNameGetResponse::CouldNotFindTheNamedChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // ChannelNameOffsetGet - GET /channel/{name}/{offset}
            &hyper::Method::Get if path.matched(paths::ID_CHANNEL_NAME_OFFSET) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_CHANNEL_NAME_OFFSET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHANNEL_NAME_OFFSET in set but failed match against \"{}\"", path, paths::REGEX_CHANNEL_NAME_OFFSET.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter name: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"]))))
                };
                let param_offset = match percent_encoding::percent_decode(path_params["offset"].as_bytes()).decode_utf8() {
                    Ok(param_offset) => match param_offset.parse::<i64>() {
                        Ok(param_offset) => param_offset,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter offset: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["offset"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.channel_name_offset_get(param_name, param_offset, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelNameOffsetGetResponse::SuccessfulFetchOfTheItem


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                ChannelNameOffsetGetResponse::CouldNotFindTheNamedChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                                ChannelNameOffsetGetResponse::CouldNotFindAnItemAtTheGivenOffset


                                                => {
                                                    response.set_status(StatusCode::try_from(416).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // ChannelNamePatch - PATCH /channel/{name}
            &hyper::Method::Patch if path.matched(paths::ID_CHANNEL_NAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_CHANNEL_NAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHANNEL_NAME in set but failed match against \"{}\"", path, paths::REGEX_CHANNEL_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter name: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.channel_name_patch(param_name, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelNamePatchResponse::SuccessfulUpdateOfTheChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                ChannelNamePatchResponse::SuggestedChannelConfigurationWasInvalid


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                ChannelNamePatchResponse::UserIsNotAuthorizedToModifyTheChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                ChannelNamePatchResponse::CouldNotFindTheNamedChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // ChannelNamePost - POST /channel/{name}
            &hyper::Method::Post if path.matched(paths::ID_CHANNEL_NAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_CHANNEL_NAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHANNEL_NAME in set but failed match against \"{}\"", path, paths::REGEX_CHANNEL_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter name: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.channel_name_post(param_name, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelNamePostResponse::ChannelCreatedSuccessfully


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                ChannelNamePostResponse::SuggestedChannelConfigurationWasInvalid


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                ChannelNamePostResponse::UserIsNotAuthorizedToCreateAChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // ChannelNamePut - PUT /channel/{name}
            &hyper::Method::Put if path.matched(paths::ID_CHANNEL_NAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_CHANNEL_NAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHANNEL_NAME in set but failed match against \"{}\"", path, paths::REGEX_CHANNEL_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter name: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.channel_name_put(param_name, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChannelNamePutResponse::SuccessfulPublishOfTheItem


                                                => {
                                                    response.set_status(StatusCode::try_from(201).unwrap());

                                                },
                                                ChannelNamePutResponse::UserIsNotAuthorizedToPublishToTheChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                ChannelNamePutResponse::CouldNotFindTheNamedChannel


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // OffsetConsumerGet - GET /offset/{consumer}
            &hyper::Method::Get if path.matched(paths::ID_OFFSET_CONSUMER) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_OFFSET_CONSUMER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE OFFSET_CONSUMER in set but failed match against \"{}\"", path, paths::REGEX_OFFSET_CONSUMER.as_str())
                    );

                let param_consumer = match percent_encoding::percent_decode(path_params["consumer"].as_bytes()).decode_utf8() {
                    Ok(param_consumer) => match param_consumer.parse::<String>() {
                        Ok(param_consumer) => param_consumer,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter consumer: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["consumer"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.offset_consumer_get(param_consumer, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OffsetConsumerGetResponse::SuccessfulAccessOfTheConsumerMetadata


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                OffsetConsumerGetResponse::ImproperlyFormattedConsumerName


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                OffsetConsumerGetResponse::UserIsNotAuthorizedToAccessThisConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                OffsetConsumerGetResponse::CouldNotFindTheNamedConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // OffsetConsumerPatch - PATCH /offset/{consumer}
            &hyper::Method::Patch if path.matched(paths::ID_OFFSET_CONSUMER) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_OFFSET_CONSUMER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE OFFSET_CONSUMER in set but failed match against \"{}\"", path, paths::REGEX_OFFSET_CONSUMER.as_str())
                    );

                let param_consumer = match percent_encoding::percent_decode(path_params["consumer"].as_bytes()).decode_utf8() {
                    Ok(param_consumer) => match param_consumer.parse::<String>() {
                        Ok(param_consumer) => param_consumer,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter consumer: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["consumer"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.offset_consumer_patch(param_consumer, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OffsetConsumerPatchResponse::SuccessfulModificationOfTheConsumerMetadata


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                OffsetConsumerPatchResponse::ImproperlyFormattedMetadata


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                OffsetConsumerPatchResponse::UserIsNotAuthorizedToModifyThisConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                OffsetConsumerPatchResponse::CouldNotFindTheNamedConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // OffsetConsumerPost - POST /offset/{consumer}
            &hyper::Method::Post if path.matched(paths::ID_OFFSET_CONSUMER) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_OFFSET_CONSUMER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE OFFSET_CONSUMER in set but failed match against \"{}\"", path, paths::REGEX_OFFSET_CONSUMER.as_str())
                    );

                let param_consumer = match percent_encoding::percent_decode(path_params["consumer"].as_bytes()).decode_utf8() {
                    Ok(param_consumer) => match param_consumer.parse::<String>() {
                        Ok(param_consumer) => param_consumer,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter consumer: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["consumer"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.offset_consumer_post(param_consumer, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OffsetConsumerPostResponse::SuccessfulCreationOfTheNamedConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                OffsetConsumerPostResponse::ImproperlyFormattedConsumerMetadata


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                OffsetConsumerPostResponse::UserIsNotAuthorizedToCreateAConsumer


                                                => {
                                                    response.set_status(StatusCode::try_from(403).unwrap());

                                                },
                                                OffsetConsumerPostResponse::TheNamedConsumerAlreadyExistsAndIsInUse


                                                => {
                                                    response.set_status(StatusCode::try_from(409).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

