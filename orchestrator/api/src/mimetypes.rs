/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for FetchManifest
    lazy_static! {
        pub static ref FETCH_MANIFEST_AGENT_ID_FOUND_AND_MANIFEST_GENERATED: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
