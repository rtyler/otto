#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;

use serde_xml_rs;
use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Channel")]
pub struct Channel {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Number of current consumers
    #[serde(rename = "consumers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consumers: Option<i64>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Channel Status
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            id: None,
            name: None,
            consumers: None,
            updated_at: None,
            status: None,
        }
    }
}
