pub struct VariantStorage {
    stream: Stream,
}

impl VariantStorage {
    pub fn new() -> VariantStorage {
        VariantStorage {
            stream: Stream::new(),
        }
    }
}
