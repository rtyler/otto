#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;

use serde_xml_rs;
use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Manifest")]
pub struct Manifest {
    /// The identifier of the agent
    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<String>,

    #[serde(rename = "services")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub services: Option<HashMap<String, models::Service>>,

    #[serde(rename = "ops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ops: Option<Vec<models::Operation>>,

}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            _self: None,
            services: None,
            ops: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    /// Globally unique ID to identify this specific operation in data stores, etc
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// Generally unique context ID to group different operations in the same context
    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,

    /// Type of operation
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<models::OperationType>,

    /// Operation type-specific data for the agent to use
    #[serde(rename = "data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<Object>,

}

impl Operation {
    pub fn new() -> Operation {
        Operation {
            id: None,
            context: None,
            _type: None,
            data: None,
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum OperationType { 
    #[serde(rename = "BEGINCTX")]
    BEGINCTX,
    #[serde(rename = "ENDCTX")]
    ENDCTX,
    #[serde(rename = "RUNPROC")]
    RUNPROC,
}

impl ::std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            OperationType::BEGINCTX => write!(f, "{}", "BEGINCTX"),
            OperationType::ENDCTX => write!(f, "{}", "ENDCTX"),
            OperationType::RUNPROC => write!(f, "{}", "RUNPROC"),
        }
    }
}

impl ::std::str::FromStr for OperationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BEGINCTX" => Ok(OperationType::BEGINCTX),
            "ENDCTX" => Ok(OperationType::ENDCTX),
            "RUNPROC" => Ok(OperationType::RUNPROC),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Service")]
pub struct Service {
    /// Key to identify the different services
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier: Option<String>,

    /// Resolvable URL to access APIs for the given service
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}

impl Service {
    pub fn new() -> Service {
        Service {
            identifier: None,
            url: None,
        }
    }
}
