/// NBT options.
pub struct Config {
    /// Determines the endianess of integers.
    /// Should be false for Java Edition, true for Bedrock Edition.
    /// 
    /// Defaults to false (big endian).
    pub little_endian: bool
}

impl Default for Config {
    fn default() -> Self {
        Config {
            little_endian: false,
        }
    }
}