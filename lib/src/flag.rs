use anyhow::Result;

pub trait FlagModule {
    /// The name of the flag
    fn name(&self) -> &str;
    /// Generates a flag with no data
    fn generate(&self) -> Result<Vec<u8>>;
    /// Encode the data into a flag
    fn encode_bytes(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>>;
    /// Decode the data from a flag
    fn decode_bytes(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>>;
    /// Check if the flag has valid data
    fn is_valid(&self, data: impl Into<Vec<u8>>) -> bool;
}
