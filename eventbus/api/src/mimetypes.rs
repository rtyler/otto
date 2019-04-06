/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for ListChannels
    lazy_static! {
        pub static ref LIST_CHANNELS_SUCCESSFUL_ENUMERATION: Mime = "application/xml".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
