pub(crate)  struct ConverterABC {
    subconverter: SubConverter,
    registerFormats: ,
    registerInputExtensions: ,
}

impl ConverterABC {
    pub(crate)  fn new() -> ConverterABC {
        ConverterABC {
            subconverter: SubConverter::new(),
            registerFormats: todo!(),
            registerInputExtensions: todo!(),
        }
    }
    
    pub(crate)  fn parseData(&self, strData: ) {
        todo!()
    }
    pub(crate)  fn parseFile(&self, filePath: ) {
        todo!()
    }
}