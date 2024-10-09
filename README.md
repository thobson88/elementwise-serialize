# elementwise-serialize
Rust crates for elementwise serialization and deserialization.

## Usage
The following code snippet provides a simple usage example. 

The necessary steps are:
- Import the `ElementwiseSerialize` and `ElementwiseDeserialize` traits and the corresponding derive macros.
- Import `Serialize` and `Deserialize` from the [serde](https://crates.io/crates/serde) crate.
- Add `#[derive]` attributes on any struct you want to serialize elementwise.
- Call the automatically generated functions `elementwise_serialize` and `elementwise_deserialize`.

```rust
use elementwise_serialize::{ElementwiseDeserialize, ElementwiseSerialize};
use elementwise_serialize_derive::{ElementwiseDeserialize, ElementwiseSerialize};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct RequesterDetails {
    requester_org: String,
    operator_name: String,
}

#[derive(Serialize, Deserialize, ElementwiseSerialize, ElementwiseDeserialize)]
struct Attestation {
    requester_details: RequesterDetails,
    udid: Option<String>,
    ddid: Option<String>,
}

fn main() {
    let requester_details = RequesterDetails {
        requester_org: "ACME Corp.".to_string(),
        operator_name: "W. E. Coyote".to_string(),
    };
    let udid = Some("did:ion:EiCClfEdkTv".to_string());
    let ddid = None;

    let attestation = Attestation {
        requester_details,
        udid,
        ddid,
    };

    // Serialize to a given directory.
    let path = PathBuf::new();
    attestation.elementwise_serialize(&path).unwrap();

    // Deserialize.
    let result = Attestation::elementwise_deserialize(&path).unwrap();

    assert_eq!(result.requester_details.operator_name, "W. E. Coyote");
    assert_eq!(result.udid, Some("did:ion:EiCClfEdkTv".to_string()));
    assert!(result.ddid.is_none());
}
```

## Example
Run the example in the `elementwise-serialize` crate with:
```
cargo run
```

## Tests
Run integration tests in the `tests` crate with:
```
cargo test --tests
```
