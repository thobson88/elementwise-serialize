use std::io::Error;
use std::path::Path;
use serde::Serialize;

pub trait ElementwiseSerialize
where
    Self: Serialize,
{
    /// Serializes each field of the struct to a separate JSON file at the given path.
    /// Each file name matches the corresponding struct field, with `.json` extension.
    /// Files are never overwritten. If the file already exists the field is skipped.
    /// Any fields with value Option::None are skipped.
    fn elementwise_serialize(&self, path: &Path) -> Result<(), Error>;
}
