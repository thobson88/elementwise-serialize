use elementwise_serialize::{ElementwiseDeserialize, ElementwiseSerialize};
use elementwise_serialize_derive::{ElementwiseDeserialize, ElementwiseSerialize};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequesterDetails {
    requester_org: String,
    operator_name: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ElementwiseSerialize, ElementwiseDeserialize)]
struct Attestation {
    requester_details: RequesterDetails,
    udid: Option<String>,
    ddid: Option<String>,
}

fn main() {
    let requester_details = RequesterDetails {
        requester_org: "Turing".to_string(),
        operator_name: "Jason".to_string(),
    };
    let udid = Some("1abc".to_string());
    let ddid = None; // None attributes are skipped.
    let attestation = Attestation {
        requester_details,
        udid,
        ddid,
    };

    // Serialize to the current directory.
    let path_buf = PathBuf::new();
    let _ = attestation.elementwise_serialize(&path_buf);

    // Deserialize.
    let deserialized = Attestation::elementwise_deserialize(&path_buf).unwrap();
    println!("Deserialized struct:\n{:?}", deserialized);
}
