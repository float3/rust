pub(crate) struct VariantStorage {
    stream: Stream,
}

impl VariantStorage {
    pub(crate) fn new() -> VariantStorage {
        VariantStorage {
            stream: Stream::new(),
        }
    }
}
