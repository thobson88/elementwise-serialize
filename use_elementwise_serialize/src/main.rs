use std::io::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::Serialize;
use serde_with::skip_serializing_none;
use elementwise_serialize::ElementwiseSerialize;
use elementwise_serialize_derive::ElementwiseSerialize;

#[derive(Debug, Clone, Serialize, ElementwiseSerialize)]
pub struct RequesterDetails {
    requester_org: String,
    operator_name: String,
}


#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, ElementwiseSerialize)]
struct Attestation {
    requester_details: RequesterDetails,
    udid: Option<String>,
    ddid: Option<String>,
}

fn main() {
    let requester_details = RequesterDetails{ 
        requester_org: "Turing".to_string(), 
        operator_name: "Jason".to_string()
    };
    let udid = Some("1abc".to_string());
    let ddid = None; // None attributes are skipped.
    let attestation = Attestation{ requester_details, udid, ddid };
    let path_buf = PathBuf::new();
    let _ = attestation.elementwise_serialize(&path_buf);
}