use elementwise_serialize::{ElementwiseDeserialize, ElementwiseSerialize};
use elementwise_serialize_derive::{ElementwiseDeserialize, ElementwiseSerialize};
use serde::{Deserialize, Serialize};

extern crate elementwise_serialize;
extern crate elementwise_serialize_derive;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Nonce(String);

#[derive(Debug, Serialize, Deserialize)]
struct RequesterDetails {
    requester_org: String,
    operator_name: String,
}

#[derive(Debug, Serialize, Deserialize, ElementwiseSerialize, ElementwiseDeserialize)]
struct Payload {
    requester_details: RequesterDetails,
    nonce: Option<Nonce>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_elementwise_serialize() {
        let requester_details = RequesterDetails {
            requester_org: "Turing".to_string(),
            operator_name: "Jason".to_string(),
        };
        let nonce = Some(Nonce("a36f0149".to_string()));
        let payload = Payload {
            requester_details,
            nonce,
        };

        let path = tempdir().unwrap().into_path();

        // Initially the temp directory is empty.
        assert!(!path.join("requester_details.json").exists());
        assert!(!path.join("nonce.json").exists());

        let result = payload.elementwise_serialize(&path);

        assert!(result.is_ok());

        // After serialization the temp directory contains a file for each non-None field.
        assert!(path.join("requester_details.json").exists());
        assert!(path.join("nonce.json").exists());
    }

    #[test]
    fn test_elementwise_deserialize() {
        let requester_details = RequesterDetails {
            requester_org: "Turing".to_string(),
            operator_name: "Jason".to_string(),
        };
        let nonce = Some(Nonce("a36f0149".to_string()));
        let payload = Payload {
            requester_details,
            nonce,
        };

        let path = tempdir().unwrap().into_path();
        let _ = payload.elementwise_serialize(&path);

        let deserialized = Payload::elementwise_deserialize(&path).unwrap();

        assert_eq!(
            deserialized.requester_details.requester_org,
            "Turing".to_string()
        );
        assert_eq!(
            deserialized.requester_details.operator_name,
            "Jason".to_string()
        );
        assert_eq!(deserialized.nonce, Some(Nonce("a36f0149".to_string())));
    }
}
