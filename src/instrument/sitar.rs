pub(crate)  struct Sitar {
    stringinstrument: StringInstrument,
}

impl Sitar {
    pub(crate)  fn new() -> Sitar {
        Sitar {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
