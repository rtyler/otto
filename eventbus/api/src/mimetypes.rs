/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for ChannelGet
    lazy_static! {
        pub static ref CHANNEL_GET_CHANNELS_SUCCESSFULLY_LISTED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for ChannelNameGet
    lazy_static! {
        pub static ref CHANNEL_NAME_GET_SUCCESSFUL_RETRIEVAL_OF_METADATA: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
