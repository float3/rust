pub struct ConverterABC {
    subconverter: SubConverter,
    registerFormats: ,
    registerInputExtensions: ,
}

impl ConverterABC {
    pub fn new() -> ConverterABC {
        ConverterABC {
            subconverter: SubConverter::new(),
            registerFormats: todo!(),
            registerInputExtensions: todo!(),
        }
    }
    
    pub fn parseData(&self, strData: ) {
        todo!()
    }
    pub fn parseFile(&self, filePath: ) {
        todo!()
    }
}